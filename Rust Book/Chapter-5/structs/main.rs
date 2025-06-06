struct User {
    email : String,
    name : String,
    active : bool
}

// Struct Tuple
struct Color (u32,u32,u32);

fn main() {
    let user1 : User = User {
        email : String::from("jk@rust.com"),
        active : true,
        name : String::from("Jay")
    };

    let mut user2 = User {
        email : String::from("abcd@gmial.com"),
        active : false,
        name : String::from("Tillu")
    };
    // Takes ownership..
    let user3 = User {
        email : String::from("abcd@gmail.com"),
        ..user2
    };
    user2.name = String::from("Tillu Badmosh");
    println!("{}",user1.name);

    let color : Color = Color(255,255,0);
    print_color(color);
}

fn print_color(color:Color) {
    let Color(R,G,B) = color;
    println!("R = {R}, G = {G}, B = {B}");
}