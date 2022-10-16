use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // 새 변수 선언 
                                   // 러스트는 기본적으로 immutable. mut 은 mutable 선언
    io::stdin()
        .read_line(&mut guess)     //read_line은 콘솔입력 뿐 아니라 result type의 값도 return 한다.
        .expect("Failed to read line"); //입력 실패시의 err메세지를 지정한다.(result type 참조)
                                        //그리고 read_line 쓰고 expect 지정 안할 시 warring 뜸
    println!("You guessed: {guess}");
}
