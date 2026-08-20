
pub struct User {
    pub id: u64,
    pub age: u8,
}

#[no_mangle]
fn find_user_by_age(age: u32) -> User {
    if age > 100 {
        User {
            id: 0xF0F0F0F0F0F0F0F0,
            age: 0xF1,
        }
    } else {
        User {
            id: 0x0F0F0F0F0F0F0F0F,
            age: 0xF2,
        }
    }
}