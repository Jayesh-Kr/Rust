fn main() {
    // Shadowing in rust
    // let mut a: i32 = 5;
    // a = true;   //? We can't do like this

    let a: i32= 5;
    let a:bool = true;
    println!("{a}");

    let x = 5;
    let x = x + 1;
    {
        // Shadowing
        let x = x * 5;
        println!("The value of x is {x}"); // 30
    }
    println!("The value of x is {x}"); // 6
}