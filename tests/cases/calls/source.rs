

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

pub fn editor(user: User) -> User {
    User {
        id: user.id,
        age: user.age
    }
}

pub fn dummy() {
    println!("Hello, world!");
}

pub fn user() -> User {
    dummy();
    editor(User {
        id: 1,
        age: 2
    })
}