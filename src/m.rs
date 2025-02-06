mod server;
use server::server;


fn main() {
    server();
    let a = 4;
    let b = a + 2;

    println!("{a} + {b}");
    println!("Hello, world!");
}
