/// 实现奇偶校验位检查
///
///
// 获取一个字节切片作为参数 bytes, 并返回一个单字节作为输出.
// 此函数可以很容易地返回一个布尔值,但是在这里返回
// u8,可以让这个返回结果在之后能够移位到某个期望的位置上.
fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

    for byte in bytes {
        // Rust 中所有的整数类型都有 count_ones 方法和 count_zeros 方法
        // count_ones: 返回整数二进制中1 的个数
        // count_zeros: 返回整数二进制中0 的个数
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }
    (n_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {:?}", abc);
    println!("output: {:08x}", parity_bit(abc));

    let abcd = b"abcd";
    println!("input: {:?}", abcd);
    println!("output: {:08x}", parity_bit(abcd));
}
