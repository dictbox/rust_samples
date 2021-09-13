
fn main() {

    let a = format!("Hellow,{}!","world");

    //输出到stdout
    println!("{}",a);

    //向右padding width个字符
    println!("{number:>width$}",number=1,width=6);
    //向右padding width个字符，并填充0
    println!("{number:0>width$}",number=1,width=6);


    //格式化为二进制
    println!("{:b}", 2);

    //使用索引，重复输出
    println!("{0}-{1}-{0}",1,2);

    //输出到stderr
    eprintln!("{0}-{1}-{0}",1,2);
}
