use std::io; // import std.io; 처럼 생각하자.
use std::cmp::Ordering; // 
use rand::Rng; // Rng 는 난수 생성기를 구현한 메서드를 정의한 트레이트(trait. 10장에서 소개)이다.
            // 해당 메서드들을 이용하기 위해서는 반드시 스코프 내에 있어야 한다.

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100); // rand 가 머이리 어려워 ㅅㅂ

    // 랜덤 숫자를 생성하기 위해서 rand 를 써야하는데 표준 라이브러리에는 없다.
    // 그래서 rand 크레이트를 호출해야한다.
    // Crate 는 러스트 코드 파일들의 모음이다. 기억하자.
    // 추가하는 방법은 Cargo.toml의 dependencies 섹션 아래에 추가.
    // 난수 표현식은 (start..=end) 이다.

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    io::stdin().read_line(&mut guess) // std 안에 io 안에 stdin 함수의 read_line 메소드
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");    
    // 엥? guess 는 이미 선언했었는데?
    // -> 러스트는 새로운 값으로 가리는 (shadow) 것을 허용한다.
    // 섀도잉은 guess_str, guess 두 개의 고유한 변수를 만드는 것보다 guess 라는 변수를 재사용하도록
    // 해준다.

    println!("You guessed: {}", guess); // {}는 자리표시자. {guess} 로 써도 되고
    // 위의 코드처럼 printf 처럼 써도 된다.

    match guess.cmp(&secret_number) { // 에러가 나는 이유: 일치하지 않는 타입이 있다는걸 알려주는 것.
        // 변수 자료형 몇몇개들은 각각 1과 100 사이의 값을 가질 수 있다 i32는 32비트 정수
        // u32 는 unsigned 32비트 정수, i64는 64비트의 정수
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"), // Ordering 은 열거형
    }


    // 

}
