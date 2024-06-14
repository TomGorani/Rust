use std::io;

fn main() {
    println!("Hello, world!");

    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("입력 실패");

    println!("당신의 선택은!? {decision}");

}
