fn main() {
    let a = String::from("Hello");
    let largest_str;
    {
        let b = String::from("World!");
         largest_str = biggest(&a, &b);
         print!(" The largest string is : {largest_str}");
    }
    // print!(" The largest string is : {largest_str}");  ❌
    // Result can be used within the smallest lifetime of the input
}

fn biggest<'x>(a : &'x String, b : &'x String) -> &'x String {
    println!("Length of a is : {} and Length of b is : {}" , a.len() , b.len());
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


// fn main() {
//     let a = "Hello";  // have lifetime of &'static
//     let largest_str;
//     {
//         let b = "World!"; 
//         largest_str = biggest(&a, &b);      // Here no error bcz str are stored in binary 💡
//     }
//     print!(" The largest string is : {largest_str}");
// }

// fn biggest<'a>(a : &'a str, b : &'a str) -> &'a str {
//     println!("Length of a is : {} and Length of b is : {}" , a.len() , b.len());
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }











// fn main() {
//     let a = String::from("Hello");
//     let b = String::from("World!");

//     let largest_str = biggest(a,b);
//     print!(" The largest string is : {largest_str}");
// }

// fn biggest(a : String, b : String) -> String {
//     println!("Length of a is : {} and Length of b is : {}" , a.len() , b.len());
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }