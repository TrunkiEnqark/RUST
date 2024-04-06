fn main() {
    let a: usize = 121323243453;
    println!("{}", std::mem::size_of_val(&a));
}