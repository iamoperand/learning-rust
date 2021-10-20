enum Color {
    Green,
    Yellow,
    Red,
    Custom { red: u8, green: u8, blue: u8 },
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        return Color::Custom {
            red: r,
            green: g,
            blue: b,
        };
    }

    fn get_red_contribution(self) -> u8 {
        match self {
            Color::Custom { red, .. } => red,
            _ => 0,
        }
    }
}

#[allow(unused_variables)]
pub fn main() {
    let current_color = Color::Yellow;

    let color_meaning = match current_color {
        Color::Green => println!("Go!"),
        Color::Red => println!("Stop!"),
        Color::Yellow => println!("Slow down!"),
        _ => println!("Custom color!"),
    };

    let new_color = Color::new(255, 0, 0);

    let custom_color = Color::Custom {
        red: 4,
        green: 0,
        blue: 0,
    };
    let red_component = custom_color.get_red_contribution();
    println!("red component is {}", red_component);

    let email_str = "hello";
    let email: Option<String> = Option::Some(email_str.to_string());
    let email: Option<String> = Option::None;

    let success_response: Result<i64, String> = Result::Ok(123);
    let error_response: Result<i64, String> = Result::Err("some error occurred".to_string());
}
