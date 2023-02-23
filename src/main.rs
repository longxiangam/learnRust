
fn main() {
    println!("Hello, world!");
    let uid:u32;
    unsafe {
        uid = libc::getuid();
    };

    println!("{}",uid);
}
