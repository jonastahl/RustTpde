

#[no_mangle]
fn fib(n: u64) -> u64 {
    if n <= 2 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

pub struct User {
    pub id: u8,
    pub age: u8
}


#[no_mangle]
pub fn editor(user: User) -> User {
    User {
        id: user.id + 1,
        age: user.age
    }
}

#[no_mangle]
pub fn user() -> User {
    editor(User {
        id: 1,
        age: 2
    })
}

// ==========================================
// 2. Register Exhaustion (Many Arguments)
// ==========================================
// Most ABIs (like System V AMD64) pass the first ~6 arguments in registers.
// This function forces the backend to push the remaining arguments to the stack.

#[no_mangle]
pub fn many_args(
    a: i8, b: i16, c: i32, d: i64,
    e: f32, f: f64, g: u8, h: u64, i: i64
) -> i64 {
    a as i64 + b as i64 + c as i64 + d + e as i64 + f as i64 + g as i64 + h as i64 + i
}

// ==========================================
// 3. Zero-Sized Types (ZST)
// ==========================================
// ZSTs take up no space and should not consume a register or stack slot in the ABI.
// If your backend isn't careful, it might misalign the `val` argument.

pub struct Zst;

#[no_mangle]
pub fn pass_zst(_zst1: Zst, val: i64, _zst2: Zst) -> i64 {
    val
}

// ==========================================
// 4. Large Structs (Hidden `sret` Pointer)
// ==========================================
// Structs that are too large to fit in return registers are usually handled
// by the caller allocating stack space and passing a hidden pointer as the first arg.

#[repr(C)]
pub struct BigStruct {
    pub data: [u64; 8],
}

#[no_mangle]
pub fn return_big_struct(seed: u64) -> BigStruct {
    BigStruct {
        data: [seed, seed+1, seed+2, seed+3, seed+4, seed+5, seed+6, seed+7],
    }
}

#[no_mangle]
pub fn consume_big_struct(big: BigStruct) -> u64 {
    big.data[0] + big.data[7]
}

// ==========================================
// 5. Heterogeneous Structs (Int/Float Splitting)
// ==========================================
// In ABIs like SysV, structs with a mix of floats and ints might be split
// across integer (GPR) and floating-point (XMM) registers.

#[repr(C)]
pub struct MixedTypes {
    pub integer: i32,
    pub floating: f64,
}

#[no_mangle]
pub fn split_registers(mixed: MixedTypes) -> f64 {
    (mixed.integer as f64) + mixed.floating
}

// ==========================================
// 6. C ABI & byval Arguments
// ==========================================
// Using extern "C" forces the C ABI. Passing a struct by value in C
// often translates to `byval` pointers in IRs like LLVM or specific
// stack copies in custom backends.

#[repr(C)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

extern "C" {
    // A dummy external C function. Your backend should emit a call to this,
    // handling the Point3D argument according to standard C ABI (often byval/stack).
    fn external_c_computation(p: Point3D, scale: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn test_c_abi_call() -> f64 {
    let p = Point3D { x: 1.5, y: 2.5, z: 3.5 };
    unsafe {
        // Calls the extern function
        external_c_computation(p, 2.0)
    }
}

// Tests receiving a struct byval via the C ABI and returning it.
#[no_mangle]
pub extern "C" fn c_abi_point_transformer(p: Point3D) -> Point3D {
    Point3D {
        x: p.x * 2.0,
        y: p.y * 2.0,
        z: p.z * 2.0,
    }
}

// ==========================================
// 7. Tuple / Multiple Returns
// ==========================================
// Rust natively returns tuples in a specific way.
// A small tuple might fit in two registers (e.g. RAX and RDX on x86_64).

#[no_mangle]
pub fn min_max(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
