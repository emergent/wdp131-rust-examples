struct Fruit {
    // (d1)
    name: String,
}

impl Fruit {
    // (d2)
    fn get_name(&self) -> &str {
        &self.name
    }
}

struct Rectangle(i32, i32); // (d3)

impl Rectangle {
    fn calc_area(&self) -> i32 {
        self.0 * self.1 // (d4)
    }
}

struct Unit; // (d5)

fn main() {
    // 定義した構造体の利用
    let banana = Fruit {
        name: String::from("Banana"),
    };
    println!("{}", banana.get_name()); // Banana

    let rect = Rectangle(10, 20);
    println!("{}", rect.calc_area()); // 200

    let _unit = Unit;
}
