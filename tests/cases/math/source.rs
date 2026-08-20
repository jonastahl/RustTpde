// Addition and subtraction for every integer width the backend supports,
// signed and unsigned. Compiled with `overflow-checks=no`, so each of these
// is a wrapping operation.

#[no_mangle]
pub extern "C" fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_i8(a: i8, b: i8) -> i8 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_i16(a: i16, b: i16) -> i16 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_i16(a: i16, b: i16) -> i16 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_i32(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_i64(a: i64, b: i64) -> i64 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_u8(a: u8, b: u8) -> u8 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_u8(a: u8, b: u8) -> u8 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_u16(a: u16, b: u16) -> u16 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_u16(a: u16, b: u16) -> u16 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_u32(a: u32, b: u32) -> u32 {
    a - b
}

#[no_mangle]
pub extern "C" fn add_u64(a: u64, b: u64) -> u64 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_u64(a: u64, b: u64) -> u64 {
    a - b
}

// Chained arithmetic: the intermediate result needs a slot of its own.
#[no_mangle]
pub extern "C" fn add3_u32(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}

// Mixed add/sub, with both arguments used twice. Equals `2 * b` for all inputs.
#[no_mangle]
pub extern "C" fn add_sub_u32(a: u32, b: u32) -> u32 {
    (a + b) - (a - b)
}

// Same, at the narrowest width, where a lost truncation is most visible.
#[no_mangle]
pub extern "C" fn add_sub_u8(a: u8, b: u8) -> u8 {
    (a + b) - (a - b)
}

// More arguments than the SysV C ABI has argument registers (6), so the last
// two arrive on the stack.
#[no_mangle]
pub extern "C" fn add8_args_u32(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    a + b + c + d + e + f + g + h
}
