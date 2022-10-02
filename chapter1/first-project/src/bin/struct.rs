// (d1)
struct Fruit {
    name: String,
}

// (d2)
impl Fruit {
    fn get_name(&self) -> &str {
        &self.name
    }
}

// (d3)
struct Rectangle(i32, i32);

impl Rectangle {
    fn calc_area(&self) -> i32 {
        self.0 * self.1 // (d4)
    }
}

// d5）
struct Unit;

fn main() {
    // Fruit構造体の利用
    let banana = Fruit {
        name: String::from("Banana"),
    };
    println!("{}", banana.get_name());
    //=> Banana

    // Rectangle構造体の利用
    let rect = Rectangle(10, 20);
    println!("{}", rect.calc_area());
    //=> 200

    // Unit構造体の利用
    let _unit = Unit;
}
