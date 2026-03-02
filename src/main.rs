mod chapter2_challenge;
mod chapter4_challenge;

fn main() {
    // chapter2_challenge::main();
    // chapter4_challenge::main();

    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y")
    } else {
        if x == y {
            println!("x is equal to y")
        } else {
            println!("x is less than to y")
        }
    }

    if x > y {
        println!("x is greater than y")
    } else if x == y {
            println!("x is equal to y")
    } else {
            println!("x is less than to y")
    }
}