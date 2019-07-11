fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // println!("This tuple has data: {}", tup); // this doesnt work

    // this works
    let (x, y, z) = tup;
    println!("This x on tuple has data: {}", x);
}
