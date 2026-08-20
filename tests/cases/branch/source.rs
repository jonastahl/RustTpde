
#[no_mangle]
pub extern "C" fn simplebranch(a: u32) -> u32 {
    let b;
    if a > 100 {
        b = 0xF0F0F0F0;
    } else {
        b = 0x0F0F0F0F;;
    }
    return b;
}
