// ==========================================
// Type Definitions (Matching your backend)
// ==========================================

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: u8,
    pub age: u8,
}

pub struct Zst;

#[repr(C)]
pub struct BigStruct {
    pub data: [u64; 8],
}

#[repr(C)]
pub struct MixedTypes {
    pub integer: i32,
    pub floating: f64,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// ==========================================
// Linker implementation for the C ABI test
// ==========================================
// Your backend's `test_c_abi_call` expects to link against this function.
// We implement it here so standard `rustc` can provide it to the linker.
#[no_mangle]
pub extern "C" fn external_c_computation(p: Point3D, scale: f64) -> f64 {
    (p.x + p.y + p.z) * scale
}

// ==========================================
// Extern Declarations (Your Backend's outputs)
// ==========================================

extern "Rust" {
    fn fib(n: u64) -> u64;

    fn editor(user: User) -> User;
    fn user() -> User;

    fn many_args(a: i8, b: i16, c: i32, d: i64, e: f32, f: f64, g: u8, h: u64, i: i64) -> i64;

    fn pass_zst(zst1: Zst, val: i64, zst2: Zst) -> i64;

    fn return_big_struct(seed: u64) -> BigStruct;
    fn consume_big_struct(big: BigStruct) -> u64;

    fn split_registers(mixed: MixedTypes) -> f64;

    fn min_max(a: i64, b: i64) -> (i64, i64);
}

extern "C" {
    fn test_c_abi_call() -> f64;
    fn c_abi_point_transformer(p: Point3D) -> Point3D;
}

// ==========================================
// The Test Runner
// ==========================================

fn main() {
    unsafe {
        // 1. Primitive ABI (Existing tests)
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 2);
        assert_eq!(fib(3), 3);
        assert_eq!(fib(4), 5);
        assert_eq!(fib(5), 8);
        assert_eq!(fib(11), 144);

        // 2. Small Structs (Returned in registers or combined)
        let u_default = user();
        assert_eq!(u_default, User { id: 2, age: 2 });

        let u_edited = editor(User { id: 42, age: 100 });
        assert_eq!(u_edited, User { id: 43, age: 100 });

        // 3. Register Exhaustion (Forces stack usage)
        // a:1 + b:2 + c:3 + d:4 + e:5.0 + f:6.0 + g:7 + h:8 + i:9 = 45
        let sum = many_args(1, 2, 3, 4, 5.0, 6.0, 7, 8, 9);
        assert_eq!(sum, 45);

        // 4. Zero-Sized Types (Should not misalign the real arguments)
        let zst_val = pass_zst(Zst, 777, Zst);
        assert_eq!(zst_val, 777);

        // 5. Large Structs (Testing hidden `sret` pointer allocation)
        let big = return_big_struct(10);
        assert_eq!(big.data, [10, 11, 12, 13, 14, 15, 16, 17]);

        let sum_ends = consume_big_struct(big);
        assert_eq!(sum_ends, 10 + 17); // 27

        // 6. Heterogeneous Structs (Testing GPR/XMM split)
        let mixed = MixedTypes { integer: 10, floating: 2.5 };
        let float_sum = split_registers(mixed);
        assert_eq!(float_sum, 12.5);

        // 7. Extern "C" ABI (Calling out of custom backend into standard rust)
        // Inside custom backend: Point3D(1.5, 2.5, 3.5), scale: 2.0
        // (1.5 + 2.5 + 3.5) * 2.0 = 7.5 * 2.0 = 15.0
        let c_result = test_c_abi_call();
        assert_eq!(c_result, 15.0);

        // 8. Extern "C" byval passing
        let p_in = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p_out = c_abi_point_transformer(p_in);
        assert_eq!(p_out, Point3D { x: 2.0, y: 4.0, z: 6.0 });

        // 9. Tuple returns
        assert_eq!(min_max(10, 5), (5, 10));
        assert_eq!(min_max(2, 8), (2, 8));

        println!("All ABI edge case tests passed!");
    }
}