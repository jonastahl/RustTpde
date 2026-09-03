
extern "Rust" {
    fn give_me_str() -> &'static str;
    fn more_string() -> &'static str;
}

fn main() {
    let str = unsafe { give_me_str() };
    assert_eq!(str, "Hello world!");

    let str2 = unsafe { more_string() };
    assert_eq!(str2, "Hallo Welt!");
}