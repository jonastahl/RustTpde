use crate::context::CodegenCx;
use crate::shared::ir::{FullType, Slot, Type};
use rustc_abi::Size;
use rustc_codegen_ssa::traits::ConstCodegenMethods;
use rustc_middle::mir::interpret::{GlobalAlloc, Scalar};
use rustc_session::PointerAuthSchema;

impl<'tcx> ConstCodegenMethods for CodegenCx<'_, 'tcx> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_undef(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_poison(&self, t: Self::Type) -> Self::Value {
        match t {
            FullType::Single(t) => self.tpde_module.borrow_mut().add_const(t, 0),
            FullType::Pair(ty_a, ty_b, o) => {
                let module = &mut self.tpde_module.borrow_mut();

                module.add_const_pair_values(ty_a, ty_b, o, 0, 0)
            },
            FullType::Memory { .. } => todo!()
        }
    }

    fn const_bool(&self, val: bool) -> Self::Value {
        todo!()
    }

    fn const_i8(&self, i: i8) -> Self::Value {
        todo!()
    }

    fn const_i16(&self, i: i16) -> Self::Value {
        todo!()
    }

    fn const_i32(&self, i: i32) -> Self::Value {
        todo!()
    }

    fn const_i64(&self, i: i64) -> Self::Value {
        todo!()
    }

    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        todo!()
    }

    fn const_u8(&self, i: u8) -> Self::Value {
        todo!()
    }

    fn const_u32(&self, i: u32) -> Self::Value {
        todo!()
    }

    fn const_u64(&self, i: u64) -> Self::Value {
        todo!()
    }

    fn const_u128(&self, i: u128) -> Self::Value {
        todo!()
    }

    fn const_usize(&self, i: u64) -> Self::Value {
        self.tpde_module.borrow_mut().add_const(Type::i64, i as u128)
    }

    fn const_uint(&self, ty: Self::Type, i: u64) -> Self::Value {
        let FullType::Single(ty) = ty else { unreachable!() };
        self.tpde_module.borrow_mut().add_const(ty, i as u128)
    }

    fn const_uint_big(&self, ty: Self::Type, u: u128) -> Self::Value {
        let FullType::Single(ty) = ty else { unreachable!() };
        self.tpde_module.borrow_mut().add_const(ty, u)
    }

    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value {
        todo!()
    }

    fn const_str(&self, s: &str) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn const_struct(&self, elts: &[Self::Value], packed: bool) -> Self::Value {
        todo!()
    }

    fn const_vector(&self, elts: &[Self::Value]) -> Self::Value {
        todo!()
    }

    fn const_to_opt_uint(&self, v: Self::Value) -> Option<u64> {
        todo!()
    }

    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128> {
        match v {
            Slot::Const(i) =>
                Some(self.tpde_module.borrow().consts.get(i as usize).unwrap().data()),
            _ => None
        }
    }

    fn scalar_to_backend_with_pac(&self, cv: Scalar, layout: rustc_abi::Scalar, ty: Self::Type, schema: Option<&PointerAuthSchema>) -> Self::Value {
        match ty {
            FullType::Single(ty) => {
                let data = match ty {
                    Type::i8 => cv.to_i8().unwrap() as u128,
                    Type::i16 => cv.to_i16().unwrap() as u128,
                    Type::i32 => cv.to_i32().unwrap() as u128,
                    Type::i64 => cv.to_i64().unwrap() as u128,
                    Type::i128 => cv.to_i128().unwrap() as u128,
                    Type::ptr => return self.ptr_scalar_to_backend(cv, layout, schema),
                    _ => todo!()
                };
                self.tpde_module.borrow_mut().add_const(ty, data)
            },
            FullType::Pair(ty_a, ty_b, offset_b) => {
                todo!()
            }
            _ => todo!()
        }
    }

    fn const_ptr_byte_offset(&self, val: Self::Value, offset: Size) -> Self::Value {
        todo!()
    }
}

impl<'tcx> CodegenCx<'_, 'tcx> {
    fn ptr_scalar_to_backend(
        &self,
        cv: Scalar,
        layout: rustc_abi::Scalar,
        schema: Option<&PointerAuthSchema>,
    ) -> Slot {
        match cv {
            Scalar::Int(int) => {
                todo!()
                // let data = int.to_bits(layout.size(self));
                // self.tpde_module.borrow_mut().add_const(Type::ptr, data)
            }
            Scalar::Ptr(ptr, _size) => {
                let (prov, offset) = ptr.prov_and_relative_offset();
                let global_alloc = self.tcx.global_alloc(prov.alloc_id());
                match global_alloc {
                    GlobalAlloc::Static(def_id) => {
                        assert!(self.tcx.is_static(def_id));
                        assert!(!self.tcx.is_thread_local_static(def_id));
                        Slot::new_global(*self.globals.get(&def_id).expect("Global undeclared"))
                    }
                    // GlobalAlloc::Memory(alloc) if alloc.inner().len() == 0 => {
                    //     todo!()
                    //     // let val = alloc.inner().align.bytes().wrapping_add(offset.bytes());
                    //     // let data = self.tcx.truncate_to_target_usize(val) as u128;
                    //     // self.tpde_module.borrow_mut().add_const(Type::ptr, data)
                    // }
                    // GlobalAlloc::Memory(alloc) => {
                    //     let id = prov.alloc_id().0;
                    //     let offset = offset.bytes();
                    //     // let global = self.global_for_alloc(prov.alloc_id());
                    //     // self.global_addr(global, offset.bytes() as i64)
                    //     todo!()
                    // }
                    // GlobalAlloc::Function { instance, .. } => {
                    //     assert_eq!(offset.bytes(), 0, "offset into a function pointer");
                    //     self.get_fn_addr(instance, schema)
                    // }
                    // // Drop the provenance, the offset contains the bytes of the hash
                    // GlobalAlloc::TypeId { .. } => self
                    //     .tpde_module
                    //     .borrow_mut()
                    //     .add_const(Type::ptr, offset.bytes() as u128),
                    // GlobalAlloc::VTable(ty, dyn_ty) => {
                    //     let global = self.global_for_vtable(ty, dyn_ty);
                    //     self.global_addr(global, offset.bytes() as i64)
                    // }
                    _ => todo!(),
                }
            }
        }
    }
}