fn main() {
    // println!("Hello, world!");
    // print_sum(5,5)
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}
