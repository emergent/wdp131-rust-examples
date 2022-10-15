fn main() {
    for i in 0..10 {
        println!("in for-loop: {}", i);
    }

    let mut count = 0;
    while count < 10 {
        count += 1; // count++ のような表記はできない
    }

    loop {
        count -= 1;
        if count == 0 {
            break;
        }
    }
}
