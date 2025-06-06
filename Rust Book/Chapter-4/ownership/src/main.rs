fn main() {
    let str = String::from("Hello world!");
    let new_str = str;
    // println!("{str}"); // We can't access the value of str, bcz now the new owner is new_str.
    println!("{new_str}");
    // Ownership comes when the data is stored in the heap.
    
    // Transfer ownwership
    // transfer_ownership(new_str);

    // println!("{new_str}"); // Here we can't access the value of new_str. To access it we can do two things.
    // transfer_ownership(new_str.clone());
    // ? OR return the string from the fnc

    let (size , new_str) = transfer_ownership(new_str);
    println!("The length of the string is : {size} and string is : {new_str}");

    // To solve this issue of re ownership we use concept of referencing....
    // Referencing -->

    let mut ss = String::from("Bye");
    let s2 = &ss;
    let s3 = &ss;
    println!("{s2}");
    println!("{s3}");
    println!("{ss}");
    let len = change_str(&mut ss);
    println!("{ss}");
    println!("{len}");
}

fn transfer_ownership(s : String) -> (usize,String) {
    println!("The value of s is : {s}");
    (s.len(),s)
}

fn change_str(s : &mut String) -> usize {
    s.push_str(" Bye");
    s.len()
}