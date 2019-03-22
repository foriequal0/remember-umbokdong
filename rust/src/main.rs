#![allow(dead_code)]

mod ubd;
use ubd::Ubd;

fn main() {
    let mut ubd = Ubd::new();
    ubd.push("엄복동");

    // ubd.push("엄복동");
    // => "엄복동 하나만 기억해주세요"

    // ubd.push("정지훈");
    // => "엄복동 하나만 기억해주세요"

    for ubd in ubd.into_iter() {
        println!("{}", ubd);
    }
}
