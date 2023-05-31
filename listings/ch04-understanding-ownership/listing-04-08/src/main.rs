fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 값 5를 받습니다

    s.clear(); // 이 코드는 String을 비워서 ""으로 만듭니다

    // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는
    // 문자열은 더 이상 없습니다. word는 이제 전혀 유효하지 않습니다!
}
// ANCHOR_END: here
