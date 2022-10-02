// (d1)
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// (d2）
impl Color {
    fn get_color(&self) -> String {
        match self {
            Color::Red => String::from("#ff0000"),
            Color::Green => String::from("#00ff00"),
            Color::Blue => String::from("#0000ff"),
            // (d3)
            Color::Custom(r, g, b) => {
                format!(
                    // 2桁の16進数で出力するための指定
                    "#{:02x}{:02x}{:02x}",
                    r, g, b
                )
            }
        }
    }
}

fn main() {
    // 列挙型の利用
    let color = Color::Blue;
    println!("{}", color.get_color());
    //=> #0000ff

    let color = Color::Custom(10, 123, 255);
    println!("{}", color.get_color());
    //=> #0a7bff
}
