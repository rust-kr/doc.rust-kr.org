fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // 여기서 r1이 스코프 밖으로 벗어나며, 따라서 아무 문제없이 새 참조자를 만들 수 있습니다.

    let r2 = &mut s;
    // ANCHOR_END: here
}
