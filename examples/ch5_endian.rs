/// # 理解字节序
/// 组成整数的各个字节的排列布局选择（从左到右、从右到左）被称为 CPU的字节序。
///
/// 有时候把一个可执行文件从一台计算机复制到另外一台上后不能正常运行，字节序不同是可能的原因之一。
///
/// ## 大端序、小端序
/// 比如 123 整数， 以常规方式书写，123就书写成123，这种被称为以大端序的格式书写。
/// 如果反转这个顺序，将123书写成321，那么这就是小端序的格式。
/// 目前整数基本是以小端序的格式来存储。

fn formated_arr_item(arr: [u8; 4]) -> String {
    let mut str = String::from("[");
    for i in arr {
        str.push_str(&format!("{:08b}", i));
        str.push_str(",");
    }
    str.pop();
    str.push_str("]");
    str
}
fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };

    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    // 字节序是从右往左排列，是小端序的格式
    println!("a: {:032b}, {:?}", a, formated_arr_item(big_endian));
    // a: 11011101110011001011101110101010, "[10101010,10111011,11001100,11011101]"

    println!("b: {:032b}, {:?}", b, formated_arr_item(little_endian));
    // b: 10101010101110111100110011011101, "[11011101,11001100,10111011,10101010]"
    println!("{} vs {}", a, b);
}
