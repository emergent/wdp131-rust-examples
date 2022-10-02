fn main() {
    // (d1)
    for i in 0..10 {
        println!("in for-loop: {}", i);
    }

    // (d2)
    let mut count = 0;
    while count < 10 {
        count += 1; // count++ のような表記はできません
    }

    // (d3)
    loop {
        count -= 1;
        // (d4)
        if count == 0 {
            break;
        }
    }
}
