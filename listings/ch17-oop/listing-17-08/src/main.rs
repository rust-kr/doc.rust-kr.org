// ANCHOR: here
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 실제로 선택 상자를 그리는 코드
    }
}
// ANCHOR_END: here

fn main() {}
