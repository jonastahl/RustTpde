use crate::builder::Builder;
use crate::context::CodegenCx;
use crate::shared::ir::{Global, Module};
use rustc_codegen_ssa::traits::{MiscCodegenMethods, StaticBuilderMethods, StaticCodegenMethods};
use rustc_middle::mir::interpret::{read_target_uint, Allocation, ConstAllocation, InitChunk, Pointer};
use rustc_span::def_id::DefId;
use std::ops::Range;
use rustc_abi::Size;

impl<'tcx> StaticBuilderMethods for Builder<'_, '_, 'tcx> {
    fn get_static(&mut self, def_id: DefId) -> Self::Value {
        todo!()
    }
}

#[derive(PartialEq)]
pub enum IsInitOrFini {
    Yes,
    No,
}
impl<'tcx> CodegenCx<'_, 'tcx> {
    pub fn const_alloc_to_tpde(
        &self,
        module: &mut Module,
        g: Global,
        alloc: &Allocation,
        is_init_fini: IsInitOrFini,
    ) {
        let dl = &self.tcx.data_layout;
        let pointer_size = dl.pointer_size().bytes() as usize;

        fn append_chunks_of_bytes(
            cx: &CodegenCx,
            module: &mut Module,
            global: Global,
            alloc: &Allocation,
            range: Range<usize>
        ) {
            let chunks = alloc.init_mask().range_as_init_chunks(range.clone().into());

            let max = cx.sess().opts.unstable_opts.uninit_const_chunk_threshold;
            let allow_uninit_chunks = chunks.clone().take(max.saturating_add(1)).count() <= max;

            if allow_uninit_chunks {
                for chunk in chunks {
                    match chunk {
                        InitChunk::Init(range) => {
                            let range = (range.start.bytes() as usize)..(range.end.bytes() as usize);
                            let bytes = alloc.inspect_with_uninit_and_ptr_outside_interpreter(range);
                            module.global_add_init_chunk(global, bytes);
                        },
                        InitChunk::Uninit(range) => {
                            let len = (range.end.bytes() - range.start.bytes()) as u32;
                            module.global_add_unit_chunk(global, len);
                        }
                    };
                }
            } else {
                let bytes = alloc.inspect_with_uninit_and_ptr_outside_interpreter(range);
                module.global_add_init_chunk(global, bytes);
            }
        }

        let mut next_offset = 0;
        for &(offset, prov) in alloc.provenance().ptrs().iter() {
            let offset = offset.bytes();
            assert_eq!(offset as usize as u64, offset);
            let offset = offset as usize;
            if offset > next_offset {
                append_chunks_of_bytes(self, module, g, alloc, next_offset..offset);
            }
            let ptr_offset = read_target_uint(
                dl.endian,
                alloc.inspect_with_uninit_and_ptr_outside_interpreter(
                    offset..(offset + pointer_size),
                ),
            ).expect("could not read relocation pointer") as u64;

            {
                let address_space = self.tcx.global_alloc(prov.alloc_id()).address_space(self);

                let schema = if self.sess().pointer_authentication() {
                    match is_init_fini {
                        IsInitOrFini::Yes => self.sess().pointer_authentication_init_fini(),
                        IsInitOrFini::No => self.sess().pointer_authentication_functions(),
                    }
                } else {
                    None
                };
                let ptr = Pointer::new(prov, Size::from_bytes(ptr_offset));
                let ptr = self.ptr_to_backend(module, ptr);

                module.global_add_reloc_chunk(g, ptr);
            }

            // TODO push the pointer
            // llvals.push(cx.scalar_to_backend_with_pac(
            //     InterpScalar::from_pointer(Pointer::new(prov, Size::from_bytes(ptr_offset)), &cx.tcx),
            //     Scalar::Initialized {
            //         value: Primitive::Pointer(address_space),
            //         valid_range: WrappingRange::full(pointer_size),
            //     },
            //     cx.type_ptr_ext(address_space),
            //     schema,
            // ));

            next_offset = offset + pointer_size;
        }
        if alloc.len() >= next_offset {
            let range = next_offset..alloc.len();
            append_chunks_of_bytes(self, module, g, alloc, range);
        }
    }
}

impl<'tcx> StaticCodegenMethods for CodegenCx<'_, 'tcx> {
    fn static_addr_of(&self, alloc: ConstAllocation<'_>, kind: Option<&str>) -> Self::Value {
        todo!()
    }

    fn codegen_static(&mut self, def_id: DefId) {
        let mut module = self.module.borrow_mut();
        let g = *self.globals.get(&def_id).expect("Global was not declared before");

        let attrs = self.tcx.codegen_fn_attrs(def_id);

        let alloc = match self.tcx.eval_static_initializer(def_id) {
            Ok(alloc) => alloc,
            Err(_) => // Error has already been reported
                return,
        }.inner();

        // some weird renaming that we possibly don't need

        module.global_set_align(g, alloc.align.bytes() as u32);


        let is_init_fini = attrs
            .link_section
            .map(|link_section| {
                let s = link_section.as_str();
                if s.starts_with(".init_array") || s.starts_with(".fini_array") {
                    IsInitOrFini::Yes
                } else {
                    IsInitOrFini::No
                }
            })
            .unwrap_or(IsInitOrFini::No);
        self.const_alloc_to_tpde(&mut module, g, alloc, is_init_fini);

        // more dll stuff
    }
}