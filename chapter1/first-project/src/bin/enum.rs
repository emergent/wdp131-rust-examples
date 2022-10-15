enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

impl Color {
    fn color_code(&self) -> String {
        match *self {
            Color::Red => String::from("#ff0000"),
            Color::Green => String::from("#00ff00"),
            Color::Blue => String::from("#0000ff"),
            Color::Custom(r, g, b) => {
                // (d1)
                format!(
                    // 2桁の16進数での出力指定
                    "#{:02x}{:02x}{:02x}",
                    r, g, b
                )
            }
        }
    }
}

fn main() {
    let color = Color::Blue;
    println!("{}", color.color_code()); // #0000ff

    let color = Color::Custom(10, 123, 255);
    println!("{}", color.color_code()); // #0a7bff
}
