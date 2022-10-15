fn main() {
    let x = 100;
    let y = 50;

    if x == y {
        // (d1)
        println!("same value!");
    }

    // if式の評価結果を変数に束縛する(d2)
    let _z = if x != y { 500 } else { 300 };
}
