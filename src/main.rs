mod chatper2_challenge;

fn main() {
    // chatper2_challenge::main();
    let result = square(13);
    println!("result is {:?}",result)

}

fn square(x: i32) -> (i32, i32) {
    println!("sqauring {}", x);
    (x, x * x)
}