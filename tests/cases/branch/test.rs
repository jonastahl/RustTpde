
extern "C" {
  fn simplebranch(a: u32) -> u32;
}

fn main() {
  // Taken branch
  assert_eq!(unsafe { simplebranch(101) }, 0xF0F0F0F0);

  // Not taken branch
  assert_eq!(unsafe { simplebranch(0) }, 0x0F0F0F0F);
  assert_eq!(unsafe { simplebranch(99) }, 0x0F0F0F0F);

  // Exactly on the boundary: `a > 100` must be false for 100
  assert_eq!(unsafe { simplebranch(100) }, 0x0F0F0F0F);

  // `a` is a u32, so `a > 100` must be an *unsigned* comparison. These fail
  // today: InstructionKind has no unsigned compares, so builder.rs maps
  // IntUGT onto CMPgt, which RustCompilerX64 lowers to the signed `jg`.
  assert_eq!(unsafe { simplebranch(0x8000_0000) }, 0xF0F0F0F0);
  assert_eq!(unsafe { simplebranch(u32::MAX) }, 0xF0F0F0F0);
}
