fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    let r3 = &mut s; // 큰 문제

    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
