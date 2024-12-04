/// # 将浮点数的位元串解释为整数
fn main() {
    let a: f32 = 42.42;
    // 类型转换：在不影响任何底层的位数据的情况下，将f32类型转为u32类型
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    // 把42.42_f32所产生的位模式视为一个十进制整数。
    println!("{}", frankentype);

    // {:032b} 表示将其格式化为一个二进制的值，这是借助std::fmt::Binary 这个trait来实现，并且要占用32位，位数不足则再其左侧补零。
    println!("{:032b}", frankentype);

    // 再将其转换为 f32 类型
    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
    //单个字节内部的布局顺序的偏好叫做位编号或者位端序
}
