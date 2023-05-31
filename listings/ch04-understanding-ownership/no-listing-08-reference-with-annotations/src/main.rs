fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    s.len()
} // 여기서 s가 스코프 밖으로 벗어납니다. 하지만 참조하는 것을 소유하고 있진 않으므로,
  // 버려지지는 않습니다.
// ANCHOR_END: here
