
pub struct User {
    pub id: u64,
    pub age: u8,
}

#[no_mangle]
fn find_user_by_age(age: u32) -> User {
    if age > 100 {
        User {
            id: 0,
            age: 0,
        }
    } else {
        User {
            id: 1,
            age: 1,
        }
    }
}