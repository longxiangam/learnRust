
use std::{ffi::{CStr, c_uchar}, str::FromStr};
use std::io::stdin;

use libc::{STDIN_FILENO, c_char, BUFSIZ, c_void, STDOUT_FILENO, c_uint};
pub fn print_dir(path: &str) {
    println!("路径：{}", path);
    let uid: u32;
    uid = unsafe { libc::getuid() };
    println!("{}", uid);

    let str = String::from_str(path).unwrap();
    let ptr = str.as_ptr();

    let dir = unsafe { libc::opendir(ptr as *const i8) };

    let mut sub_dir = unsafe { libc::readdir(dir) };

    while !sub_dir.is_null() {
        let entry = unsafe { &*sub_dir };
        let name = unsafe { CStr::from_ptr(entry.d_name.as_ptr()).to_string_lossy() };
        println!("Name: {}", name);
        sub_dir = unsafe { libc::readdir(dir) };
    }
    unsafe {
        libc::closedir(dir);
    }
}


pub fn read_file(){
    let mut _buf:[u32 ; BUFSIZ as usize]  = [0;BUFSIZ as usize];

    let n = unsafe {
        libc::read( STDIN_FILENO, _buf.as_ptr() as *mut c_void, (BUFSIZ  as c_uint).try_into().unwrap())
    };
    if n > 0{

        unsafe {
            let mut input_str = CStr::from_ptr((_buf.as_ptr() as *const c_char).try_into().unwrap()).to_string_lossy();
            let mut input_str = input_str.to_string();
            input_str = input_str.replace("\n","") +"456";
            println!("输入：{}",input_str);
        };

        unsafe{
            libc::write(STDOUT_FILENO, _buf.as_ptr() as *mut c_void, (n  as c_uint).try_into().unwrap());
        }
    }
}