/// 从一个 u8 中产生一个在[0, 1]区间的f32

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;

    let large_n = (n as u32) << 15; // 先将输入字节 n 对齐到32 位,然后通过左移15 位的操作让数值变大

    let f32_bits = base | large_n; // 使用按位或操作,把上一步转换后的输入字节合并到基础字节数据
                                   // base 中.

    let m = f32::from_bits(f32_bits);

    2.0 * (m - 0.5)
}
fn main() {
    println!("rand f32: {}", mock_rand(0xff));
    println!("rand f32: {}", mock_rand(0x7f));
    println!("rand f32: {}", mock_rand(0x00));
}
