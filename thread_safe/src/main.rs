use std::{thread, sync::{Arc, Mutex}};


fn main(){
   //arc_mutex();

   let a = "123".to_string();
   let b = &a;
   let mut c = &b;
   
}





// 使用管道通信
fn learn_channel(){

}


// 多线程操作同一数据
fn arc_mutex() {
    let duration = std::time::Duration::from_micros(1);

    let vec = Arc::new( Mutex::new( Vec::new()));
    
    let vec_1 = vec.clone();
    let vec_2 = vec.clone();
    let join_1 = thread::spawn(move ||{
        for i in 0..10000 {
            let mut temp = vec_1.lock().unwrap();
            if(!temp.contains(&i)){
                temp.push(i);
                thread::sleep(duration);
            }
        }
    });
    let join_2 = thread::spawn(move ||{
        for i in 0..10000 {
            let mut temp = vec_2.lock().unwrap();
            if(!temp.contains(&i)){
                temp.push(i);
                thread::sleep(duration);
            }
        }
    });

    join_1.join().expect("fail1");
    join_2.join().expect("fail2");

    let mut prev = -1;
    for v in vec.lock().unwrap().iter()  {
        if v <= &prev {
            panic!("{},{}顺序不正确",prev,v);
        }
        prev = *v;

    }

    println!("{:?}",vec.lock().unwrap());
 
}





