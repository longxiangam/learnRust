
use std::io;
use rand::seq::SliceRandom;
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


pub fn four_color() {
    let colors = ["红色", "绿色", "蓝色", "黄色", "黑色", "白色"];
    let mut rng = rand::thread_rng();
    let answer: Vec<_> = colors.choose_multiple(&mut rng, 4).collect();

    println!("颜色列表：{}",colors.join(","));
    println!("我选了四种颜色，请你猜一下它们是什么颜色，顺序很重要！");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("无法读取输入");

        let guess: Vec<_> = guess.trim().split_whitespace().collect();
        if guess.len() != 4 {
            println!("请猜四种颜色，用空格分隔。");
            continue;
        }

        let mut correct_color_count = 0;
        let mut correct_position_count = 0;
        for (i, &color) in guess.iter().enumerate() {
            if answer.contains(&&color) {
                correct_color_count += 1;
            }
            if *answer[i] == color {
                correct_position_count += 1;
            }
        }

        println!("你猜对了 {} 种颜色，其中 {} 种颜色的位置也猜对了。",
                 correct_color_count, correct_position_count);

        if correct_position_count == 4 {
            println!("恭喜你，你猜对了！");
            break;
        }
    }
}