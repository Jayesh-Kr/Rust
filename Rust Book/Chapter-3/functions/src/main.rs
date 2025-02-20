fn main() {
    println!("Hello, world!");
    my_function();
    add(5,6);
    expression_vs_statement();
    println!("The value of sum is : {}",return_sum(10,20));
}

fn my_function() {
    println!("hello world");
}

// Fn with parameter
fn add(a:u32 , b:u32) {
    println!("The sum of parameters is {}",(a+b));
}

// ! Imporant expression vs statement. A expression returns a result while a statement does not. Calling a fnc is expression 

fn expression_vs_statement() {
    // expression 
    let y = {
        let x = 5;
        x + 1  // Expression
    };
    println!("The value of y is {y}");
}

// ? Returning from the function

fn return_sum(a:i32,b:i32) -> i32{
    // return a+b;
    // Or
    a + b
}