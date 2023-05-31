fn main() {
    // ANCHOR: here
    {                      // s는 아직 선언되지 않아서 여기서는 유효하지 않습니다
        let s = "hello";   // 이 지점부터 s가 유효합니다

        // s로 어떤 작업을 합니다
    }                      // 이 스코프가 종료되었고, s가 더 이상 유효하지 않습니다
    // ANCHOR_END: here
}