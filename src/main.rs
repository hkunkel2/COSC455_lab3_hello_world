mod chapter2_challenge;
mod chapter4_challenge;

fn main() {
    // chapter2_challenge::main();
    // chapter4_challenge::main();

    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letters is {}", letters[count]);
        count += 1;
    }
}