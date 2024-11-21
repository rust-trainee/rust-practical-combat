/// # 返回错误信息
use rand::prelude::*;

/// 辅助函数让我们偶尔触发错误
fn one_in(denominator: u32) -> bool {
    // thread_rng 创建一个线程局部随机数生成器，gen_ratio(n, m), 会以n/m的概率来返回一个布尔值。
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
        }
    }
    fn new_with_data(name: &str, data: &[u8]) -> Self {
        Self {
            name: String::from(name),
            data: data.into(),
        }
    }
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        save_to.append(&mut tmp);
        Ok(read_len)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}
fn main() {
    let mut f4 = File::new_with_data("f4.txt", &[114, 117, 115, 116, 33]);

    let mut buffer = vec![];

    f4 = open(f4).unwrap();
    let f4_len = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_len);
    println!("{}", text);
}
