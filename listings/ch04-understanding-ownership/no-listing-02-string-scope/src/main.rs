fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s는 이 지점부터 유효합니다

        // s를 가지고 무언가 합니다
    }                                  // 이 스코프가 종료되었고, s는 더 이상
                                       // 유효하지 않습니다.
    // ANCHOR_END: here
}
