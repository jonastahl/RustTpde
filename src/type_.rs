use crate::context::CodegenCx;
use crate::shared::ir::{FullType, FunctionSignature, Type};
use rustc_abi::{AddressSpace, BackendRepr, Primitive, Reg, Scalar};
use rustc_codegen_ssa::common::TypeKind;
use rustc_codegen_ssa::traits::{BaseTypeCodegenMethods, DerivedTypeCodegenMethods, LayoutTypeCodegenMethods, TypeMembershipCodegenMethods};
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::Ty;
use rustc_target::callconv::{CastTarget, FnAbi, PassMode};

impl<'tpde, 'tcx> CodegenCx<'tpde, 'tcx> {
    pub fn tpde_direct_type(&self, ty: TyAndLayout<'tcx>) -> FullType {
        match ty.backend_repr {
            BackendRepr::Scalar(scalar) => {
                self.tpde_scalar_type(scalar)
            },
            BackendRepr::ScalarPair { a, b, b_offset } => {
                let FullType::Single(a) = self.tpde_scalar_type(a) else { unreachable!() };
                let FullType::Single(b) = self.tpde_scalar_type(b) else { unreachable!() };

                FullType::Pair(a, b, b_offset.bytes_usize() as u8)
            },
            BackendRepr::Memory { sized } => {
                FullType::Memory { sized }
            },
            _ => todo!()
        }
    }

    fn tpde_scalar_type(&self, scalar: Scalar) -> FullType {
        if scalar.is_bool() {
            return FullType::Single(Type::Bool);
        }
        match scalar.primitive() {
            Primitive::Int(i, _) => self.type_from_integer(i),
            Primitive::Float(f) => self.type_from_float(f),
            Primitive::Pointer(address_space) => self.type_ptr_ext(address_space),
        }
    }
}

impl<'tcx> BaseTypeCodegenMethods for CodegenCx<'_, 'tcx> {
    fn type_i8(&self) -> Self::Type {
        FullType::Single(Type::i8)
    }

    fn type_i16(&self) -> Self::Type {
        FullType::Single(Type::i16)
    }

    fn type_i32(&self) -> Self::Type {
        FullType::Single(Type::i32)
    }

    fn type_i64(&self) -> Self::Type {
        FullType::Single(Type::i64)
    }

    fn type_i128(&self) -> Self::Type {
        FullType::Single(Type::i128)
    }

    fn type_isize(&self) -> Self::Type {
        todo!()
    }

    fn type_f16(&self) -> Self::Type {
        todo!()
    }

    fn type_f32(&self) -> Self::Type {
        todo!()
    }

    fn type_f64(&self) -> Self::Type {
        todo!()
    }

    fn type_f128(&self) -> Self::Type {
        todo!()
    }

    fn type_array(&self, ty: Self::Type, len: u64) -> Self::Type {
        todo!()
    }

    fn type_func(&self, args: &[Self::Type], ret: Self::Type) -> Self::FunctionSignature {
        todo!()
    }

    fn type_kind(&self, ty: Self::Type) -> TypeKind {
        match ty {
            FullType::Single(ty) => match ty {
                Type::Void => TypeKind::Void,
                Type::Bool | Type::i8 |  Type::i16 | Type::i32 | Type::i64 => TypeKind::Integer,
                _ => todo!()
            },
            FullType::Pair(ty1, ty2, _) => TypeKind::Struct,
            FullType::Memory { .. } => todo!()
        }
    }

    fn type_ptr(&self) -> Self::Type {
        todo!()
    }

    fn type_ptr_ext(&self, address_space: AddressSpace) -> Self::Type {
        FullType::Single(Type::ptr)
    }

    fn element_type(&self, ty: Self::Type) -> Self::Type {
        todo!()
    }

    fn vector_length(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn float_width(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn int_width(&self, ty: Self::Type) -> u64 {
        todo!()
    }

    fn val_ty(&self, v: Self::Value) -> Self::Type {
        self.tpde_module.borrow().type_of_slot(v)
    }
}

impl<'tcx> TypeMembershipCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {

}

impl<'tcx> CodegenCx<'_, 'tcx> {
    pub fn create_function_signature(
        &self,
        fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> FunctionSignature {
        let mut args: Vec<Type> = {
            // we can ignore variadic arguments
            let args = if fn_abi.c_variadic {
                &fn_abi.args[..fn_abi.fixed_count as usize]
            } else {
                &fn_abi.args
            };
            args.iter()
                .flat_map(|arg| match &arg.mode {
                    PassMode::Ignore => vec![Type::Void],
                    PassMode::Direct(_) => {
                        let FullType::Single(ty) = self.tpde_direct_type(arg.layout) else {
                            unreachable!()
                        };
                        vec![ ty ]
                    }
                    PassMode::Pair(..) => {
                        let FullType::Pair(a, b, _) = self.tpde_direct_type(arg.layout) else {
                            unreachable!()
                        };
                        vec![a, b]
                    }
                    PassMode::Cast { cast, pad_i32: _ } => todo!(),
                    PassMode::Indirect {
                        attrs,
                        meta_attrs,
                        on_stack,
                    } => {
                        vec![Type::ptr]
                    }
                })
                .collect()
        };
        match fn_abi.ret.mode {
            PassMode::Indirect {
                attrs,
                meta_attrs,
                on_stack,
            } => {
                args.insert(0, Type::ptr);
            }
            _ => (),
        };

        let ret =
            if fn_abi.ret.is_ignore() {
                None
            } else {
                Some(self.tpde_direct_type(fn_abi.ret.layout))
            };

        FunctionSignature{ args, ret }
    }
}

impl<'tcx> LayoutTypeCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.tpde_direct_type(layout)
    }

    fn cast_backend_type(&self, ty: &CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::FunctionSignature {
        let signs = &mut self.function_signatures.borrow_mut();
        signs.push(self.create_function_signature(fn_abi));
        signs.len() - 1
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        // TODO adapt for i1
        self.tpde_direct_type(layout)
    }

    fn scalar_pair_element_backend_type(&self, layout: TyAndLayout<'tcx>, index: usize, immediate: bool) -> Self::Type {
        self.tpde_direct_type(layout)
    }
}