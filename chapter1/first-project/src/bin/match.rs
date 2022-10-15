fn main() {
    let i = 5;

    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=10 => println!("four to ten"),
        _ => println!("other"),
    }

    let is_zero_str = match i {
        // (d1)
        0 => "zero",
        _ => "not zero",
    };
    println!("{}", is_zero_str); // not zeroが出力される
}
