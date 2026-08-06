
#[no_mangle]
pub fn simplebranch(a: u32) -> u32 {
    let b;
    if a > 100 {
        b = 1;
    } else {
        b = 0;
    }
    return b;
}
