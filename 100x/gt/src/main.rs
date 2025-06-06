struct Rect<T> {   // T -> trait
    width : T,
    height : T
}

// struct Rect {
//     width : u8,
//     height : u8
// }

impl<T : std::ops::Mul<Output = T>> Rect<T> {
    fn area(self) -> T {
        self.width * self.height
    }

    fn say() {
        println!("This is rectangle");
    }
}


// impl Rect {
//     fn area(&self) -> u8 {
//         self.width * self.height
//     }

//     fn say() {
//         println!("This is rectangle");
//     }
// }


fn main() {
    let r = Rect::<u8> {
        width : 25,
        height : 10
    };

    // let r = Rect {
    //     width : 25,
    //     height : 10
    // };

    Rect::<u8>::say();
    // Rect::say();
    println!("The area of rectangle is : {}" , r.area());
    // println!("The area of rectangle is : {}" , find_area(r));
}

// fn find_area(r : Rect) -> u8{
//     r.width * r.height
// }