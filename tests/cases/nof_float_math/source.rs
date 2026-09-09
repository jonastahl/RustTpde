// Floating-point arithmetic for the two widths the backend supports. Nothing
// here can overflow in the integer sense, so the `nof_` prefix only keeps this
// case off the overflow-check path -- the results are plain IEEE-754.

// The five arithmetic operators, at both widths.
#[no_mangle]
pub fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

#[no_mangle]
pub fn sub_f32(a: f32, b: f32) -> f32 {
    a - b
}

#[no_mangle]
pub fn mul_f32(a: f32, b: f32) -> f32 {
    a * b
}

#[no_mangle]
pub fn div_f32(a: f32, b: f32) -> f32 {
    a / b
}

#[no_mangle]
pub fn rem_f32(a: f32, b: f32) -> f32 {
    a % b
}

#[no_mangle]
pub fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

#[no_mangle]
pub fn sub_f64(a: f64, b: f64) -> f64 {
    a - b
}

#[no_mangle]
pub fn mul_f64(a: f64, b: f64) -> f64 {
    a * b
}

#[no_mangle]
pub fn div_f64(a: f64, b: f64) -> f64 {
    a / b
}

#[no_mangle]
pub fn rem_f64(a: f64, b: f64) -> f64 {
    a % b
}

// Negation, which flips the sign bit and so is defined on zeros and NaNs too.
#[no_mangle]
pub fn neg_f32(a: f32) -> f32 {
    -a
}

#[no_mangle]
pub fn neg_f64(a: f64) -> f64 {
    -a
}

// Comparisons. These are the ordered predicates, so every one of them is
// false when either operand is NaN -- including `le` and `ge`, which are not
// the negations of `gt` and `lt`.
#[no_mangle]
pub fn cmp_eq_f32(a: f32, b: f32) -> bool {
    a == b
}

#[no_mangle]
pub fn cmp_ne_f32(a: f32, b: f32) -> bool {
    a != b
}

#[no_mangle]
pub fn cmp_lt_f32(a: f32, b: f32) -> bool {
    a < b
}

#[no_mangle]
pub fn cmp_le_f32(a: f32, b: f32) -> bool {
    a <= b
}

#[no_mangle]
pub fn cmp_gt_f32(a: f32, b: f32) -> bool {
    a > b
}

#[no_mangle]
pub fn cmp_ge_f32(a: f32, b: f32) -> bool {
    a >= b
}

#[no_mangle]
pub fn cmp_eq_f64(a: f64, b: f64) -> bool {
    a == b
}

#[no_mangle]
pub fn cmp_ne_f64(a: f64, b: f64) -> bool {
    a != b
}

#[no_mangle]
pub fn cmp_lt_f64(a: f64, b: f64) -> bool {
    a < b
}

#[no_mangle]
pub fn cmp_le_f64(a: f64, b: f64) -> bool {
    a <= b
}

#[no_mangle]
pub fn cmp_gt_f64(a: f64, b: f64) -> bool {
    a > b
}

#[no_mangle]
pub fn cmp_ge_f64(a: f64, b: f64) -> bool {
    a >= b
}

// ==========================================
// Fused F32 Comparisons + Branches / Selects
// ==========================================

#[no_mangle]
pub fn branch_eq_f32(a: f32, b: f32) -> i32 {
    if a == b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_ne_f32(a: f32, b: f32) -> i32 {
    if a != b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_lt_f32(a: f32, b: f32) -> i32 {
    if a < b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_le_f32(a: f32, b: f32) -> i32 {
    if a <= b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_gt_f32(a: f32, b: f32) -> i32 {
    if a > b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_ge_f32(a: f32, b: f32) -> i32 {
    if a >= b { 10 } else { 20 }
}

// ==========================================
// Fused F64 Comparisons + Branches / Selects
// ==========================================

#[no_mangle]
pub fn branch_eq_f64(a: f64, b: f64) -> i32 {
    if a == b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_ne_f64(a: f64, b: f64) -> i32 {
    if a != b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_lt_f64(a: f64, b: f64) -> i32 {
    if a < b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_le_f64(a: f64, b: f64) -> i32 {
    if a <= b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_gt_f64(a: f64, b: f64) -> i32 {
    if a > b { 10 } else { 20 }
}

#[no_mangle]
pub fn branch_ge_f64(a: f64, b: f64) -> i32 {
    if a >= b { 10 } else { 20 }
}

// Casts between the two float widths. Widening is exact; narrowing rounds,
// and saturates to infinity once the value leaves the f32 range.
#[no_mangle]
pub fn f32_to_f64(a: f32) -> f64 {
    a as f64
}

#[no_mangle]
pub fn f64_to_f32(a: f64) -> f32 {
    a as f32
}

// Casts between floats and integers. Rust defines these as saturating: a
// value past the integer's range clamps to `MIN` or `MAX`, and NaN becomes 0.
// That is not what the bare hardware instruction does, so the backend has to
// emit the clamping itself.
#[no_mangle]
pub fn f32_to_i32(a: f32) -> i32 {
    a as i32
}

#[no_mangle]
pub fn f32_to_i64(a: f32) -> i64 {
    a as i64
}

#[no_mangle]
pub fn f32_to_u32(a: f32) -> u32 {
    a as u32
}

#[no_mangle]
pub fn f32_to_u64(a: f32) -> u64 {
    a as u64
}

#[no_mangle]
pub fn f64_to_i32(a: f64) -> i32 {
    a as i32
}

#[no_mangle]
pub fn f64_to_i64(a: f64) -> i64 {
    a as i64
}

#[no_mangle]
pub fn f64_to_u32(a: f64) -> u32 {
    a as u32
}

#[no_mangle]
pub fn f64_to_u64(a: f64) -> u64 {
    a as u64
}

#[no_mangle]
pub fn i32_to_f32(a: i32) -> f32 {
    a as f32
}

#[no_mangle]
pub fn i32_to_f64(a: i32) -> f64 {
    a as f64
}

#[no_mangle]
pub fn i64_to_f32(a: i64) -> f32 {
    a as f32
}

#[no_mangle]
pub fn i64_to_f64(a: i64) -> f64 {
    a as f64
}

#[no_mangle]
pub fn u32_to_f32(a: u32) -> f32 {
    a as f32
}

#[no_mangle]
pub fn u32_to_f64(a: u32) -> f64 {
    a as f64
}

#[no_mangle]
pub fn u64_to_f32(a: u64) -> f32 {
    a as f32
}

#[no_mangle]
pub fn u64_to_f64(a: u64) -> f64 {
    a as f64
}

// Chained arithmetic, where each intermediate has to round before the next
// operation sees it. `a * b + c` must NOT be contracted into a single fused
// multiply-add: fusing it would skip a rounding step and change the result.
#[no_mangle]
pub fn mul_add_f64(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}

#[no_mangle]
pub fn mul_add_f32(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

// Operator precedence: `a + b * c`, not `(a + b) * c`.
#[no_mangle]
pub fn add_mul_f64(a: f64, b: f64, c: f64) -> f64 {
    a + b * c
}

// Left-associative, and not reassociated: for these operands the two
// groupings give different answers.
#[no_mangle]
pub fn add3_f64(a: f64, b: f64, c: f64) -> f64 {
    a + b + c
}

// Mixed add/sub with both arguments used twice. Unlike the integer case this
// is not exactly `2 * b`, since each half rounds separately.
#[no_mangle]
pub fn add_sub_f64(a: f64, b: f64) -> f64 {
    (a + b) - (a - b)
}

// More float arguments than the SysV C ABI has SSE argument registers (8), so
// the last one arrives on the stack.
#[no_mangle]
pub fn add9_args_f64(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
) -> f64 {
    a + b + c + d + e + f + g + h + i
}

// Integer and float arguments are counted against separate register classes,
// so these all stay in registers despite there being ten of them.
#[no_mangle]
pub fn mixed_args(a: f64, b: u64, c: f64, d: u64, e: f64, f: u64) -> f64 {
    (a + c + e) * ((b + d + f) as f64)
}
