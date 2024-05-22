use std::io;

fn main() {
    println!("Fibonacci Challenge");
    println!("------------------------");

    let mut input = String::new();

    println!("Please insert your number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u8 = input.trim().parse().expect("Please insert a number");

    let mut a = 0;
    let mut b = 0;

    let nth = loop {
        let c = a;

        if a == 0 {
            a += 1;
        } else if b == 0 {
            b += 1;
        } else {
            a = a + b;
            b = c;
        };

        if a > input {
            break c;
        }
    };

    println!("Output: {nth}");
}
