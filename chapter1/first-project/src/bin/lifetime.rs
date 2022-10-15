struct Greet<'a> {
    word: &'a str,
}

impl<'a> Greet<'a> {
    fn say(&self) {
        println!("{}", self.word);
    }
}

fn main() {
    let hello: &str = "Hello!";
    {
        let greet = Greet { word: hello };
        greet.say();
    } // greetはここで解放される

    println!("{}", hello);
} // helloはここで解放される
