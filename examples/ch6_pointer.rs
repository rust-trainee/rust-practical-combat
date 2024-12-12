use std::mem::size_of;
/// # 内存
///
/// Option<T> 类型使用了空指针优化,来确保 Option<T> 在编译后的二进制文件中占用0 字节. 它的变体 None
/// 是由一个空指针来表示的(一个指向无效内存的指针),这就让它的变体 Some(T)不需要额外的间接访问.
///
/// 引用\ 指针和地址有什么区别?
/// - 内存地址: 内存地址通常简称为地址,一个内存地址是一个数字,并且恰好指向了内存中的一个字节.
/// 内存地址是由汇编语言提供的一种抽象概念.
/// - 指针有时又叫做原始指针,一个指针是一个内存地址,并且指向了某个类型的值.
/// 指针是由高级语言提供的一种抽象概念.
/// - 引用是指针,在动态大小类型的情况下,一个应用是一个指针以及一个额外保证的整数.引用是 Rust
/// 提供的一种抽象概念.
///
///
/// Rust引用的优点:
/// - 引用总是指向有效的数据: 基于所有权和借用生命周期
/// - 引用被正确地对齐为 usize 地倍数: 由于技术的原因, 当要求 CPU 获取未对齐的内存时, CPU
/// 的行为会变的不稳定. Rust 的类型实际上包含填充的字节, 填充为 usize 的倍数.
/// - 引用能够为动态大小的类型提供这样的保证: 对于在内存中没有固定宽度的类型, Rust
/// 会确保在内部指针的旁边保留一个长度. 这样一来, Rust 就可以确保应用程序在使用该类型的时候,
/// 永远不会超出该类型所在的内存区域.
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 101];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    // 使用引用来模拟指针

    // let a = 42;
    // let b = &B;
    // let c = &C;
    // println!("a: {}, b: {:p}, c: {:p}", a, b, c);

    // 通过几种不同的类型, 来对比引用和 Box<T>
    // let a: usize = 42;
    // let b: &[u8; 10] = &B;
    // let c: Box<[u8]> = Box::new(C);
    //
    // println!("a (an unsigned inter): ");
    // println!("  location: {:p}", &a);
    // println!("  size: {:?} bytes", size_of::<usize>());
    // println!("  value: {:?}", a);
    //
    // println!("b (an unsigned inter): ");
    // println!("  location: {:p}", &b);
    // println!("  size: {:?} bytes", size_of::<&[u8; 10]>());
    // println!("  value: {:?}", b);
    //
    // println!("c (an unsigned inter): ");
    // println!("  location: {:p}", &c);
    // println!("  size: {:?} bytes", size_of::<Box<[u8]>>());
    // println!("  value: {:?}", c);
    //
    // println!("B (an unsigned inter): ");
    // println!("  location: {:p}", &B);
    // println!("  size: {:?} bytes", size_of::<[u8; 10]>());
    // println!("  value: {:?}", B);
    //
    // println!("C (an unsigned inter): ");
    // println!("  location: {:p}", &C);
    // println!("  size: {:?} bytes", size_of::<[u8; 11]>());
    // println!("  value: {:?}", C);

    // 输出来自外部来源的字符串
    //
    // 一种智能指针类型, 能够从其指针位置读取数据而无需先复制它.
    // 写时复制, 适用于读多写少的场景
    use std::borrow::Cow;
    // CStr 是一个类似于 C 字符串的类型, 它让 Rust 能够读取以0 作为结束标识的字符串.
    use std::ffi::CStr;
    // c_char 是 Rust 中i8 类型的别名, 但对特定的平台, 它可能会存在一些细微的差别.
    use std::os::raw::c_char;

    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c,);
}
