fn main() {
    println!("{}", div(4, 2));
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[test] // (d1)
fn div_test() {
    assert_eq!(div(10, 3), 3);
}

#[test]
#[should_panic] // (d2)
fn div_panic_test() {
    div(2, 0);
}
