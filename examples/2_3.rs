fn main() {

    // # 浮点数精度
    let x = (-42.0f32).sqrt();
    println!("{}", x == x);
    println!("{}", x.is_nan());
    // 如果此数字既不是无穷大也不是NaN，则返回true。
    println!("{}", x.is_finite());

    let x: f32 = 1.0 / 0.0;
    println!("{}", x);
    println!("{}", x.is_finite());

    // # 有理数、复数和其他类型
    //  Rust并没有支持上述类型，可以借助于num crate来实现
    use num::complex::Complex;
    let a = Complex {
        re: 2.1,
        im: -1.2
    };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}