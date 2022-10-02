// 関数の定義 (d1)
fn add(x: i32, y: i32) -> i32 {
    // return x + y; とも書けますが
    // 通常は以下のように書きます
    x + y
}

fn main() {
    // 関数の呼び出し (d2)
    let added = add(10, 20);
    println!("{}", added);
    //=> 30

    // クロージャ (d3)
    let z = 20;
    let add_z = |x: i32| x + z;
    println!("{}", add_z(10));
    //=> 30
}
