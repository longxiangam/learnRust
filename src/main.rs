#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::io::{Stdin, stdin, stdout, Write};
use std::thread::Thread;

#[cfg(not(target_os = "windows"))]
mod dir;
mod ptr;
mod guess;


fn main() {

   /* dir::print_dir("/");
    dir::read_file();*/

/*    let i = 1;
    let j = 1;
    for i in 1..10 {
        for j in i..10 {
            print!("{} * {} = {};  ",i,j,i * j);
        }
        println!("");
    }*/


/*    ptr::test_ptr();

    guess::start();*/
    guess::four_color();
}
