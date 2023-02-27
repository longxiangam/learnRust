

use rand::Rng;

pub fn start(){
    let mut colors = [-1 ; 4];
    
    for mut o in colors  {
        let mut temp_v = -1 ;

        while colors.contains(&temp_v) {
           temp_v =  rand::thread_rng().gen_range(0..3);
        }

        o = temp_v;
        println!("{}",o);
    }
}