trait Hello {
    fn hello(&self);
}

impl<T: Copy> Hello for T {
    fn hello(&self) {
        println!("Hello, world!");
    }
}

fn main() {
    2.hello();
    'x'.hello();
    false.hello();
    "Bob".hello();
    1.2355.hello();
}
