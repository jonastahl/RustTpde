use rustc_abi::Size;
use rustc_codegen_ssa::traits::ConstCodegenMethods;
use rustc_middle::mir::interpret::Scalar;
use rustc_session::PointerAuthSchema;
use crate::context::CodegenCx;
use crate::shared::ir::{FullType, Slot, Type};

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

                module.add_const_pair(ty_a, ty_b, o, 0, 0)
            },
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

    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        todo!()
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
        let FullType::Single(ty) = ty else { unreachable!() };
        let data = match ty {
            Type::i8 => cv.to_i8().unwrap() as u128,
            Type::i16 => cv.to_i16().unwrap() as u128,
            Type::i32 => cv.to_i32().unwrap() as u128,
            Type::i64 => cv.to_i64().unwrap() as u128,
            _ => todo!()
        };
        self.tpde_module.borrow_mut().add_const(ty, data)
    }

    fn const_ptr_byte_offset(&self, val: Self::Value, offset: Size) -> Self::Value {
        todo!()
    }
}