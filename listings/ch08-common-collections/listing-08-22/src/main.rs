fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점부터 유효하지 않습니다.
    // 사용을 시도해보고 무슨 컴파일러 에러가 발생하는 알아보세요!
    // ANCHOR_END: here
}
