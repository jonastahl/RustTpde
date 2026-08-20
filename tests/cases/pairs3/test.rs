
// Must mirror the layout of `User` in source.rs exactly (default Rust ABI,
// so the struct must *not* be `repr(C)` here either).
struct User {
    id: u64,
    age: u8,
}

extern "Rust" {
  fn find_user_by_age(age: u32) -> User;
}

fn check(age: u32, expected_id: u64, expected_age: u8) {
  let u = unsafe { find_user_by_age(age) };
  assert_eq!(u.id, expected_id, "id for age {}", age);
  assert_eq!(u.age, expected_age, "age for age {}", age);
}

fn main() {
  // Taken branch
  check(101, 0xF0F0_F0F0_F0F0_F0F0, 0xF1);

  // Not taken branch
  check(0, 0x0F0F_0F0F_0F0F_0F0F, 0xF2);
  check(99, 0x0F0F_0F0F_0F0F_0F0F, 0xF2);

  // Exactly on the boundary: `age > 100` must be false for 100
  check(100, 0x0F0F_0F0F_0F0F_0F0F, 0xF2);

  check(0x8000_0000, 0xF0F0_F0F0_F0F0_F0F0, 0xF1);
  check(u32::MAX, 0xF0F0_F0F0_F0F0_F0F0, 0xF1);
}
