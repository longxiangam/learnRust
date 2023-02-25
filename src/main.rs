#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
mod dir;

fn main() {
    println!("Hello, world!");
    //dir::print_dir("/");
    //dir::read_file();

    let a = 1;
    let aa = 1;
    let b = &a as *const i32;
    let c = &aa as *const i32;
    println!("{}",b as i32);
    println!("{}",c as i32);
}
