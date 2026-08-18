
struct User {
    id: u64,
    age: u8,
}

#[no_mangle]
fn increase_age(user: User) -> User {
    User {
        id: user.id,
        age: user.age + 1,
    }
}

// #[no_mangle]
// fn find_user(age: u32) -> User {
//     let u: User;
//     if age > 100 {
//         u = User {
//             id: 0,
//             age: 0,
//         }
//     } else {
//         u = User {
//             id: 1,
//             age: 1,
//         }
//     }
//     return u;
// }
