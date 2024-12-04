fn main() {
    // # for循环：迭代的中心支柱
    //  在for循环中是遍历，默认会进行所有权语义-move，因为for循环接口，实际上被编译器扩展成了方法的调用。
    let v = vec![1,2,3];
    for item in v { // move
        println!("item: {item}");
    }
    // println!("{:?}", v); // error

    // # while循环：直到循环条件改变了循环的状态
    //  示例：通过自增计数器来测试你的计算机有多快
    use std::time::{Duration, Instant};

    let mut count = 0;
    //  创建一个Duran实例，表示1个1s的时间间隔
    let time_limit = Duration::new(1, 0);
    //  访问系统的时钟时间。
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

}
