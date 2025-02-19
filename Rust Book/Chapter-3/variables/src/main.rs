const P : i32 = 3;
fn main() {
    // We can't use keywords as variables of fn names.

    // Difference btw let and const -
    // --> const can be defined anywhere, (not necessary to use within a scope) , not true for let
    // --> const can't be mutable, i.e. we can't use ( const mut a = 5;)
    // --> while using const we have to give the type to the variable
    const ABC : i32 = 5;
    // --> we can't initialize value to const variables during runtime
    // eg = const a : i32 = 5*9*age;

    //? cosnt variable must to UpperCase
    println!("{ABC}");
    println!("{P}");
    println!("Hello, world!");
}
