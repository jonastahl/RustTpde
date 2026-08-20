
pub struct User {
    pub id: u64,
    pub age: u8,
}

#[no_mangle]
fn find_user(age: u32) -> User {
    let u: User;
    if age > 100 {
        u = User {
            id: 0xF0F0F0F0F0F0F0F0,
            age: 0xF1,
        }
    } else {
        u = User {
            id: 0x0F0F0F0F0F0F0F0F,
            age: 0xF2,
        }
    }
    return u;
}