extern "C" {
    fn say_hello();
}

fn main() {
    unsafe {
        say_hello();
    }
}

