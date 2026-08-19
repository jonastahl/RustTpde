use crate::context::CodegenCx;
use crate::shared::ir::{FullType, Type};
use rustc_abi::{AddressSpace, BackendRepr, Primitive, Reg, Scalar};
use rustc_codegen_ssa::common::TypeKind;
use rustc_codegen_ssa::traits::{BaseTypeCodegenMethods, DerivedTypeCodegenMethods, LayoutTypeCodegenMethods, TypeMembershipCodegenMethods};
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::Ty;
use rustc_target::callconv::{CastTarget, FnAbi};

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
                _ => unreachable!()
            },
            FullType::Pair(ty1, ty2, _) => TypeKind::Struct,
        }
    }

    fn type_ptr(&self) -> Self::Type {
        todo!()
    }

    fn type_ptr_ext(&self, address_space: AddressSpace) -> Self::Type {
        todo!()
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

impl<'tcx> LayoutTypeCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        todo!()
    }

    fn cast_backend_type(&self, ty: &CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::FunctionSignature {
        todo!()
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
        todo!()
    }
}