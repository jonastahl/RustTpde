use crate::context::CodegenCx;
use crate::shared::ir::Type;
use rustc_abi::{AddressSpace, BackendRepr, Primitive, Reg, Scalar};
use rustc_codegen_ssa::common::TypeKind;
use rustc_codegen_ssa::traits::{BaseTypeCodegenMethods, DerivedTypeCodegenMethods, LayoutTypeCodegenMethods, TypeMembershipCodegenMethods};
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::Ty;
use rustc_target::callconv::{CastTarget, FnAbi};

impl<'tpde, 'tcx> CodegenCx<'tpde, 'tcx> {
    pub fn tpde_type(&self, ty: TyAndLayout<'tcx>) -> Type {
        match ty.backend_repr {
            BackendRepr::Scalar(scalar) => {
                if scalar.is_bool() {
                    Type::Bool
                } else {
                    self.tpde_scalar_type(scalar)
                }
            },
            _ => todo!()
        }
    }

    fn tpde_scalar_type(&self, scalar: Scalar) -> Type {
        match scalar.primitive() {
            Primitive::Int(i, _) => self.type_from_integer(i),
            Primitive::Float(f) => self.type_from_float(f),
            Primitive::Pointer(address_space) => self.type_ptr_ext(address_space),
        }
    }
}

impl<'tcx> BaseTypeCodegenMethods for CodegenCx<'_, 'tcx> {
    fn type_i8(&self) -> Self::Type {
        Type::i8
    }

    fn type_i16(&self) -> Self::Type {
        Type::i16
    }

    fn type_i32(&self) -> Self::Type {
        Type::i32
    }

    fn type_i64(&self) -> Self::Type {
        Type::i64
    }

    fn type_i128(&self) -> Self::Type {
        todo!()
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
        todo!()
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
        todo!()
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
        self.tpde_type(layout)
    }

    fn scalar_pair_element_backend_type(&self, layout: TyAndLayout<'tcx>, index: usize, immediate: bool) -> Self::Type {
        todo!()
    }
}