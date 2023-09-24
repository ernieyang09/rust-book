fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("scope {x}");
        // scope 12
    }

    println!("outside scope {x}");
    // outside scope 6

}