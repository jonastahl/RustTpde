
// Must mirror the layout of `User` in source.rs exactly (default Rust ABI,
// so the struct must *not* be `repr(C)` here either).
struct User {
    id: u64,
    age: u8,
}

extern "Rust" {
  fn increase_age(user: User) -> User;
}

fn check(id: u64, age: u8, expected_age: u8) {
  let u = unsafe { increase_age(User { id, age }) };
  assert_eq!(u.id, id);
  assert_eq!(u.age, expected_age);
}

fn main() {
  check(0, 0, 1);
  check(1, 41, 42);

  // Both halves of the pair at their extremes
  check(u64::MAX, 0, 1);
  check(0, u8::MAX - 1, u8::MAX);

  // The second half wraps without touching the first half
  check(u64::MAX, u8::MAX, 0);

  // Bit patterns that would show up as a mix-up of the two pair slots
  check(0xF0F0_F0F0_F0F0_F0F0, 0x0F, 0x10);
  check(0x0F0F_0F0F_0F0F_0F0F, 0xF0, 0xF1);
}
