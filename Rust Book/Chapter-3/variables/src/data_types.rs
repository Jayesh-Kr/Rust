fn main() {
    // Data types -
    let a : u32 = 625;
    let b : i32 = -8448;
    
    // We can also write like this
    let a = 625u32;
    let b = -92898i32;

    // Cool feautres - To make number more readable we can use _
    let num = 1_00_000;
    let num2 = 65_u32;
    
    // we can also write numbers in hex,oct,etc.
    let hex = 0xff;
    let oct = 0o77;
    let binary = 0b0111011;


    // Floating numbers - f32 & f64 (default). It has only signed numbers
    let f : f32 = 566.559;
    let f2 : f64 = 88.5288;

    //! For divison
    let div1 = 4 / 2; // 2
    let div2 = 5 / 2; // 2
    let div3 : f64 = 5_f32 / 2_f32; // 2.5 
    /// or 
    let div3 = 5.0 / 2.0;


    // Boolean
    let a_bool : bool = true;
    let a_bool = false;

    // Character ''
    let c : char = 'A';
    let str = "String";

    // Tuples
    let tup : (i32,f64,char) = (64,96.54,'B');
    // Destructing tuple
    let (x,y,z) = tup;
    // OR
    let abcd = tup.0;
    let abcd = tup.1;
    let abcd = tup.2;
    // Unit - Empty tuple
    let cd = ();


    // Arrays
    let arr : [i64;5] = [1,2,3,4,5];
    //!
    let arr2 : [i32;5] = [10;5] // [10,10,10,10,10]
    // Accessing array elements
    let num1 = arr[0];
}