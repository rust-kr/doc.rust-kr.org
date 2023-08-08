fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다

    println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다
                       // ANCHOR_END: here
}
