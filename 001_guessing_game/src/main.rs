use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字游戏");
    println!("游戏说明：系统会随机生成一个1~100之间的整数，如果猜对了就获取胜利");

    //声明一个不可变变量，用来存放随机生成的目标数字
    let target: u32 = rand::thread_rng().gen_range(1..101);

    //声明一个可变变量，用来接收用户的输入
    let mut str_number = String::new();

    loop {
        println!("请输入一个数字：");
        io::stdin() //获取标准输入流
            .read_line(&mut str_number) //从标准输入读取一行数据，并将其存入到str_number
            .expect("读取数字异常");

        //使用match来返回一个有效值，
        //注：continue或return会跳出当前循环，同时也跳过了赋值，因此不会引发编译错误。
        let guess_number: u32 = match str_number.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("无效输入，请重新输入!");
                str_number.clear();
                continue;
            }
        };

        match guess_number.cmp(&target) {
            Ordering::Less => {
                println!("小了");
            }
            Ordering::Equal => {
                println!("恭喜，猜对了！");
                println!("按任意键退出游戏！");
                io::stdin().read_line(&mut str_number).unwrap();
                break;
            }
            Ordering::Greater => {
                println!("大了")
            }
        }

        str_number.clear();
    }
}
