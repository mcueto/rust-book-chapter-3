fn main() {
    let new_value = sum_values(five(), twenty());
    print_value(new_value);
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn twenty() -> i32 {
    return 20;
}

fn sum_values(value_1: i32, value_2: i32) -> i32 {
    return value_1 + value_2;
}
