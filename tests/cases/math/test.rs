// Every add/sub the backend emits, at every width, signed and unsigned.
//
// Rather than hand-writing expected values, each generated function is checked
// against Rust's own `wrapping_add`/`wrapping_sub` over the full cross product
// of interesting operands: zero, one, both sides of the signed boundary
// (0x7F.. / 0x80..), and the type's extremes. That is where a missing
// truncation, a sign-extension that should have been a zero-extension, or an
// operand swap shows up.

extern "C" {
  fn add_i8(a: i8, b: i8) -> i8;
  fn sub_i8(a: i8, b: i8) -> i8;
  fn add_i16(a: i16, b: i16) -> i16;
  fn sub_i16(a: i16, b: i16) -> i16;
  fn add_i32(a: i32, b: i32) -> i32;
  fn sub_i32(a: i32, b: i32) -> i32;
  fn add_i64(a: i64, b: i64) -> i64;
  fn sub_i64(a: i64, b: i64) -> i64;
  fn add_u8(a: u8, b: u8) -> u8;
  fn sub_u8(a: u8, b: u8) -> u8;
  fn add_u16(a: u16, b: u16) -> u16;
  fn sub_u16(a: u16, b: u16) -> u16;
  fn add_u32(a: u32, b: u32) -> u32;
  fn sub_u32(a: u32, b: u32) -> u32;
  fn add_u64(a: u64, b: u64) -> u64;
  fn sub_u64(a: u64, b: u64) -> u64;

  fn add3_u32(a: u32, b: u32, c: u32) -> u32;
  fn add_sub_u32(a: u32, b: u32) -> u32;
  fn add_sub_u8(a: u8, b: u8) -> u8;
  fn add8_args_u32(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32) -> u32;
}

/// Checks `add_$ty` and `sub_$ty` over every ordered pair drawn from `$vals`.
macro_rules! check_add_sub {
  ($ty:ty, $add:ident, $sub:ident, $vals:expr) => {{
    let vals: &[$ty] = &$vals;
    for &a in vals {
      for &b in vals {
        assert_eq!(
          unsafe { $add(a, b) },
          a.wrapping_add(b),
          concat!(stringify!($add), "({}, {})"), a, b
        );
        assert_eq!(
          unsafe { $sub(a, b) },
          a.wrapping_sub(b),
          concat!(stringify!($sub), "({}, {})"), a, b
        );
      }
    }
  }};
}

fn main() {
  check_add_sub!(i8, add_i8, sub_i8, [i8::MIN, i8::MIN + 1, -2, -1, 0, 1, 2, i8::MAX - 1, i8::MAX]);
  check_add_sub!(i16, add_i16, sub_i16, [i16::MIN, i16::MIN + 1, -2, -1, 0, 1, 2, i16::MAX - 1, i16::MAX]);
  check_add_sub!(i32, add_i32, sub_i32, [i32::MIN, i32::MIN + 1, -2, -1, 0, 1, 2, i32::MAX - 1, i32::MAX]);
  check_add_sub!(i64, add_i64, sub_i64, [i64::MIN, i64::MIN + 1, -2, -1, 0, 1, 2, i64::MAX - 1, i64::MAX]);
  check_add_sub!(u8, add_u8, sub_u8, [0, 1, 2, 0x7F, 0x80, 0x81, 0xFE, 0xFF]);
  check_add_sub!(u16, add_u16, sub_u16, [0, 1, 2, 0x7FFF, 0x8000, 0x8001, 0xFFFE, 0xFFFF]);
  check_add_sub!(u32, add_u32, sub_u32, [0, 1, 2, 0x7FFF_FFFF, 0x8000_0000, 0x8000_0001, 0xFFFF_FFFE, 0xFFFF_FFFF]);
  check_add_sub!(u64, add_u64, sub_u64, [0, 1, 2, 0x7FFF_FFFF_FFFF_FFFF, 0x8000_0000_0000_0000, 0x8000_0000_0000_0001, 0xFFFF_FFFF_FFFF_FFFE, 0xFFFF_FFFF_FFFF_FFFF]);

  // Chained arithmetic, including a carry out of the low word.
  assert_eq!(unsafe { add3_u32(1, 2, 3) }, 6);
  assert_eq!(unsafe { add3_u32(u32::MAX, 1, 1) }, 1);
  assert_eq!(unsafe { add3_u32(0x8000_0000, 0x8000_0000, 7) }, 7);

  // (a + b) - (a - b) == 2 * b, wrapping, for every operand pair.
  for a in [0u32, 1, 0x7FFF_FFFF, 0x8000_0000, u32::MAX] {
    for b in [0u32, 1, 0x7FFF_FFFF, 0x8000_0000, u32::MAX] {
      assert_eq!(unsafe { add_sub_u32(a, b) }, b.wrapping_mul(2), "add_sub_u32({}, {})", a, b);
    }
  }
  for a in [0u8, 1, 0x7F, 0x80, 0xFF] {
    for b in [0u8, 1, 0x7F, 0x80, 0xFF] {
      assert_eq!(unsafe { add_sub_u8(a, b) }, b.wrapping_mul(2), "add_sub_u8({}, {})", a, b);
    }
  }

  // Stack-passed arguments: the 7th and 8th must not be dropped or swapped.
  assert_eq!(unsafe { add8_args_u32(1, 2, 3, 4, 5, 6, 7, 8) }, 36);
  assert_eq!(unsafe { add8_args_u32(0, 0, 0, 0, 0, 0, 0, u32::MAX) }, u32::MAX);
  assert_eq!(unsafe { add8_args_u32(0, 0, 0, 0, 0, 0, u32::MAX, 0) }, u32::MAX);
  assert_eq!(unsafe { add8_args_u32(u32::MAX, 1, 0, 0, 0, 0, 0, 0) }, 0);
  assert_eq!(unsafe { add8_args_u32(1, 0, 0, 0, 0, 0, 0, u32::MAX) }, 0);
}
