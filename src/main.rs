mod chapter2_challenge;
mod chapter4_challenge;

fn main() {
    // chapter2_challenge::main();
    // chapter4_challenge::main();

    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};

    // if make_x_odd {
    //     x = 1;
    // } else {
    //     x = 2
    // }

    println!("x is {}", x);
}