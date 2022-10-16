use std::process::Command;

fn main(){
    Command::new("echo")
            .arg("hello world")  //옵션 또한 인자로 줄 수 있다.
            .spawn()
            .expect("echo command faild");
}

