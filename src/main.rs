#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::io::{Stdin, stdin, stdout, Write};
use std::thread::Thread;

mod dir;
mod ptr;

fn main() {
    println!("Hello, world!");
    let stdin = stdin();
    let mut readLine = String::new();
    stdin.read_line(&mut readLine);

    let mut stdout = stdout();
    stdout.write(readLine.as_bytes());

    let i = 1;
    let j = 1;
    for i in 1..10 {
        for j in i..10 {
            print!("{} * {} = {};  ",i,j,i * j);
        }
        println!("");
    }

}
