
extern "C" {
  fn add(a: u32, b: u32) -> u32;
  fn sub(a: u32, b: u32) -> u32;
}

fn main() {
  // u32
  assert_eq!(unsafe { add(1, 2) }, 3);
  assert_eq!(unsafe { sub(2, 1) }, 1);
  assert_eq!(unsafe { add(0, 0) }, 0);

  assert_eq!(unsafe { add(0xF0F0F0F0, 0x0F0F0F0F) }, 0xFFFFFFFF);
  assert_eq!(unsafe { sub(0xF0F0F0F0, 0xF0F0F0F0) }, 0);

  // u32 wrap-around (built with overflow-checks=no)
  assert_eq!(unsafe { add(u32::MAX, 1) }, 0);
  assert_eq!(unsafe { add(u32::MAX, u32::MAX) }, u32::MAX - 1);
  assert_eq!(unsafe { sub(0, 1) }, u32::MAX);
  assert_eq!(unsafe { sub(0, u32::MAX) }, 1);
}
