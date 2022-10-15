fn main() {
    // 文字列は、大きく&str型とString型がある (d1)
    let str_slice: &str = "world";
    let _string: String = String::from(str_slice);
    let _string_format: String =
        format!("Hello, {}", str_slice);

    // 要素数3の配列（固定長配列）(d2)
    let mut array: [i32; 3] = [1, 2, 3];
    array[0] = 10; // mutならば要素の値を変更できる

    // array.push(10); // mutでも要素の追加はできない

    // 要素数3のベクタ（可変長配列）(d3)
    let mut vec: Vec<i32> = vec![1, 2, 3]; // (d4)
    vec[0] = 10; // mutならば要素の値を変更できる
    vec.push(10); // mutならば要素を追加できる
}
