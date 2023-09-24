fn main() {
    let number = 6;

    if number > 5 {
        println!("> 5");
    } else {
        println!("< 5");
    };

    let condition = true;

    let var = if condition { 7 } else { 8 };

    println!("var {var}");
}