fn main() {
    // 所有権の移動(d1)
    {
        let x = String::from("hello");
        let y = x; // ここでムーブ

        // xにあった値の所有権はyに移動済みなのでもうxは使えない
        //println!("{}", x);
        println!("y:{}", y);
        // yもここで解放される
    }

    // 借用(d2)
    {
        let z = String::from("hello");
        {
            let w = &z; // ここで借用

            //let s = z; // (d3)
            println!("w:{}", w);
            // wはここで解放される
        }
        // 上のブロックではzを借用していただけなのでzはまだ使える
        println!("z:{}", z);
        // zはここで解放される
    }
}
