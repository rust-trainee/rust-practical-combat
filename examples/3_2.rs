#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // 创建一个副本，因为save_to.append会清空掉输入的Vec<u8>
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    // 预留容量
    // save_to.reserve(read_length);
    // 将tmp中的元素移动到save_to中，会清空tmp
    // append内部会调reserve来预留容量
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer = vec![];
    open(&mut f1);
    let f1_length = read(&f1, &mut buffer);
    close(&mut f1);

    // 把 Vec<u8>转换成 String，任何无效字符都会被替换成乱码
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text)
}
