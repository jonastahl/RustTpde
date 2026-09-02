
#[no_mangle]
static HELLO_WORLD: &str = "Hello world!";

#[no_mangle]
fn give_me_str() -> &'static str {
    HELLO_WORLD
}

#[no_mangle]
fn more_string() -> &'static str {
    "Hallo Welt!"
}