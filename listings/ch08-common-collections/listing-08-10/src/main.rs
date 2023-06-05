fn main() {
    // ANCHOR: here
    {
        let v = vec![1, 2, 3, 4];

        // v를 가지고 작업하기
    } // <- 여기서 v가 스코프 밖으로 벗어나고 해제됩니다
      // ANCHOR_END: here
}
