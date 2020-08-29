#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width : size,
            height: size,
        }
    }

    fn can_hold(&self ,other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}





fn main (){

    let rec1 = Rectangle {
        width:32,
        height:50,
    };
    let rec2 = Rectangle {
        width:10,
        height:10,
    };
    let rec3 = Rectangle {
        width:322,
        height:250,
    };

    println!("{}",rec1.can_hold(&rec2));


fn area(Dimension : (i32,i32)) -> i32 {
    Dimension.0 * Dimension.1
}
}