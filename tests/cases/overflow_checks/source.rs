

#[no_mangle]
fn addu(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
fn addi(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
fn subu(a: u32, b: u32) -> u32 {
    a - b
}

#[no_mangle]
fn subi(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
fn mulu(a: u32, b: u32) -> u32 {
    a * b
}

#[no_mangle]
fn muli(a: i32, b: i32) -> i32 {
    a * b
}