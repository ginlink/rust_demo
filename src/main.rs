// fn main() {
//     println!("Hello, world!");
// }

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn struct_demo() {
    let rect = Rect {
        width: 3,
        height: 5,
    };

    println!("{:?}", rect);
    println!("{}", rect.area());
}

fn main() {
    struct_demo();
}
