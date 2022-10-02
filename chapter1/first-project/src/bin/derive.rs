#[derive(Debug)]
struct Hours(u32);

fn main() {
    let h = Hours(5);
    println!("{:?}", h); //=> Hours(5)
}
