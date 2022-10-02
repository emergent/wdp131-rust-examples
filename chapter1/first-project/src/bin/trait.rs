trait Greeter {
    fn greet(&self);
}

struct Person(String);

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, I am {}!", self.0);
    }
}

struct JapanesePerson(String, i32);

impl Greeter for JapanesePerson {
    fn greet(&self) {
        println!(
            "こんにちは、私は{}です。{}歳です。",
            self.0, self.1
        );
    }
}

fn main() {
    let person = Person("John".into());
    person.greet();
    let japanese = JapanesePerson("花子".into(), 20);
    japanese.greet();
}
