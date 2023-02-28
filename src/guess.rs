

use rand::Rng;

pub fn start(){
    let mut colors = [-1 ; 4];
    
    for i in 0..4 {
        let mut temp_v = 0 ;
        while colors.contains(&temp_v) {
           temp_v =  rand::thread_rng().gen_range(0..4);
        }
        colors[i] = temp_v;
        println!("{}",temp_v);

    }

    
    
}