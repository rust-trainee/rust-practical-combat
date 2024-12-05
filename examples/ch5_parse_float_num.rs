/// 解构一个浮点数的值

const BIAS: i32 = 123;

const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);

    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field | as bits | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1; // 使用移位操作一出不需要的31 位数据,只保留符号位.

    let exponent = (bits >> 23) & 0xff; // 先移除23个不需要的位数据,然后使用逻辑与掩码操作过滤高位的数据.
    let fraction = bits & 0x3fffff; // 使用一个与掩码操作, 只保留23 个最低有效位.

    (sign, exponent, fraction) // 尾数部分在这里叫做 fraction(分数),执行了解码操作以后,才管这部分数据叫做尾数
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // 把符号位转换成1.0 或者 -1.0 .这里-1.0_f32
    // 需要用括号括起来,用于表明运算的优先级,这是因为方法调用的优先级高于单目减号运算符的.
    let signed_1 = (-1.0_f32).powf(sign as f32);

    // 指数必须先转为 i32 , 因为减去 BIAS
    // 以后的结果有可能是负数.接下来,还需要把它转为f32,这样才能把它用于指数幂的运算中
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    // 解码尾数
    for i in 0..23 {
        let mark = 1 << i;
        let one_at_bit_i = fraction & mark;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
