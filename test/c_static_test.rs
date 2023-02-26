#[test_case]
fn c_static_test() {
    #[link(name = "say", kind = "static")]
    extern "C" {
        pub fn say_hello();
    }
    unsafe {
        say_hello();
    }
}
