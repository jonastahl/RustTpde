
extern "Rust" {
    fn give_me_str() -> &str;
}

fn main() {
    let str = give_me_str();
    assert_eq!(str, "Hello world!");
}