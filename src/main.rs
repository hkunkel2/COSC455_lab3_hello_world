mod chatper2_challenge;

fn main() {
    // chatper2_challenge::main();
    say_hello();
    say_a_number(23);

    let x = 1;
    let y = 2;

    say_a_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello() {
    println!("Hello!")
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_a_sum(num1: u8, num2: u8) {
    println!("sum is {}", num1 + num2);
}