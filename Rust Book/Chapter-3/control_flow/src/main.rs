fn main() {
    // if , else if and else - These are expression i.e. they return something.
    let a = 3;
    if a == 3 {
        println!("This number is 3");
    } else {
        println!("This number is not 3");
    }

    // ?
    let y = if a > 0 { "Number is positive"} else {"Number is negative"};
    println!("{y}");


    // Loops
    let mut num = 0;
    loop {
        if num == 10 {
            break;
        }
        println!("{num}");
        num = num + 1;
    }


    //? Something very interesting ......
    // We can even return after breaking...
    let z = loop {
        // num = num + 1;
    if a == 3 {
        break 70;
    }
};
    println!("The value of z is : {z}");


    // For iterating array
    let arr = [1,2,3,4,5,6,7];
    for x in arr {
        println!("The element in arr is = {} ", x );
    }

    //? To print a range of number
    // To include 10 - 0..=10
    // To reverse - for x in (0..10).rev()
    for x in 0..10 {
        println!("The number is = {x}");
    }
}
