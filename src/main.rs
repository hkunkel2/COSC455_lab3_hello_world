mod chapter2_challenge;
mod chapter4_challenge;

fn main() {
    // chapter2_challenge::main();
    // chapter4_challenge::main();

    let message = ['h','e','l','l','o'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number)
    }
}