
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