use std::{ffi::CStr, str::FromStr};
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
