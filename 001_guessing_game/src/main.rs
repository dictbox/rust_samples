use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("猜数字游戏");
    println!("游戏说明：系统会随机生成一个1~100之间的整数，如果猜对了就获取胜利");
    let target:u32 = rand::thread_rng().gen_range(1..101);

    let mut str_number = String::new();

    loop {
        println!("请输入一个数字：");
        io::stdin().read_line(&mut str_number).expect("读取数字异常");

        let guess_number: u32 = match str_number.trim().parse() {
            Ok(num) => num,
            _ => continue,
        };

        match guess_number.cmp(&target) {
            Ordering::Less => { println!("小了"); }
            Ordering::Equal => {
                println!("恭喜，猜对了！");
                println!("按任意键退出游戏！");
                io::stdin().read_line(&mut str_number).unwrap();
                break;
            }
            Ordering::Greater => { println!("大了") }
        }

        str_number.clear();
    }
}
