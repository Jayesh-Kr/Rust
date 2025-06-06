struct Rectangle {
    width : u32,
    height : u32
}

// To relate a particular functions with the struct we use method syntax. So // ? From this there is a diff in fn and method
impl Rectangle {
    // fn calculate_area(a : &Self) -> u32 {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
    fn print_extra(&self,e : u32) {
        println!("The value of extra is = {e}");
    }
    // ! Important
    fn create_sq(side:u32) -> Self {
        Rectangle {
            width : side,
            height : side
        }
    }
}
fn main() {
    let rec = Rectangle {
        width : 50,
        height : 40
    };
    // let area = calculate_area(&rec);
    // println!("The area of rectangle is : {area}");
    println!("The area of rectangle is : {}",rec.calculate_area());
    rec.print_extra(54);

    let square : Rectangle = Rectangle::create_sq(5);
}

fn calculate_area(rectangle : &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}