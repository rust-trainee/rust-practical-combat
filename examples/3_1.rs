#![allow(unused_variables)] // 放宽编译器警告
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

//  ! 返回类型也叫Never类型 告知 Rust 编译器，次函数用不返回
#[allow(dead_code)] // 放宽未使用函数编译器警告
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
}
