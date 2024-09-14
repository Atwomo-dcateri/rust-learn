pub enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
} 
pub enum Color {

    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}