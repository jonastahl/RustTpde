

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