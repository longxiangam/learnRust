pub fn test_ptr(){
    //可变指针
    let mut a = String::from("123");
    let aa = 1;

    let c = &aa as *const i32;
    unsafe {
        let b = &mut a as *mut String;
        *b = String::from("456");
        println!("{}",*b);
    }

    println!("{}",c as i32);
}
