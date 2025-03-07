///
/// 问题描述：小明所在的学校有五个补习班，政治补习班每隔一天补习一次，绘画补习班每隔2天补习一次，
/// 英语补习班每隔3天补习一次，数学补习班每隔4天补习一次，体育补习班每个5天补习一次。
/// 现在请问：请问从1月1号开始后，一季度（二月28天）5个补习班一共有多少次同时出现在学校补习，
/// 有多少天都没有出现在学校。
///
fn main() {
    //一季度共有90天，用一个数组来记录
    let mut days = [0; 90];
    //补习班1mod/2=1 ,补习班2 mod/3=1 补习班3 mod/4=1,补习班4 mod/5=1  补习班5 mod/6=1
    //但考虑到数组下标是从0开始的，所以上面的mod值都改为0.
    //使用二进制位运算来标记是否来学校补习：1,10(2),100(4),1000(8),10000(16)
    //则全部来的时候值为11111，都不来的时候值为0

    for i in 0..90 {
        if i % 2 == 0 { days[i] = days[i] | 1; }
        if i % 3 == 0 { days[i] = days[i] | 2; }
        if i % 4 == 0 { days[i] = days[i] | 4; }
        if i % 5 == 0 { days[i] = days[i] | 8; }
        if i % 6 == 0 { days[i] = days[i] | 16; }
    }

    let mut all_days = vec![];
    let mut nobody_days = vec![];

    //统计来的天数
    for i in days.iter().enumerate() {
        if *i.1 == 0b_11111 {
            all_days.push((i.0 + 1).to_string()); //方便打印，这里直接转化为字符串了
        } else if *i.1 == 0 {
            nobody_days.push((i.0 + 1).to_string());
        }
    }

    println!("一季度全部到校次数是：{}，分别在第{}天", all_days.len(), all_days.join("、"));
    println!("一季度全未到校次数是：{}，分别在第{}天", nobody_days.len(), nobody_days.join("、"));
}
