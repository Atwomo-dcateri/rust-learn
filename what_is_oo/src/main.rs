use std::string;

use gui::Draw;
use what_is_oo::{Screen, SelectBox, Button};
fn main() {
    let screen = Screen{
        componets:vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button{
                width: 50,
                height:10,
                String::from("OK"),
            }),
        ],
    };

    screen.run();
}