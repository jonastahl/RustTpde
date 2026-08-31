

#[no_mangle]
fn fib(n: u64) -> u64 {
    if n <= 2 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

pub struct User {
    pub id: u64,
    pub age: u64
}


#[no_mangle]
pub fn func(a: i32, b: i32, c: i32, d: i32, e: i32, u: User, f: i32) -> User {
    u
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