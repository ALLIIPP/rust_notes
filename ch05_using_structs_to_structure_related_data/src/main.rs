
struct Color(i32,i32,i32);

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect1:&Rectangle)->bool {
        self.width > rect1.width && self.height > rect1.height
    }
    fn cube(size:u32)-> Rectangle{
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let mut user1 = User{
        email: String::from("example@email.com"),
        username: String::from("example user"),
        sign_in_count: 5,
        active: true
    };

    user1.active = false;

    let user2 = User{
        email: String::from("another_example@email.com"),
        username: String::from("another example"),
        ..user1
    };


    let red = Color(255,0,0);
    let Color(r,g,b) = red;

    println!("red:{}, green:{}, blue:{}", r,g,b);

    println!("user1 active: {}",user1.active);
    println!("user2 active: {}",user2.active);

    println!("{:#?}", user1);

    let rec1 = Rectangle{width: 10, height: 20};
    let rec2 = Rectangle{width: 100, height: 100};

    println!("can r1 hold r2: {}", rec1.can_hold(&rec2));
    println!("can r2 hold r1: {}", rec2.can_hold(&rec1));
    
    let cube = Rectangle::cube(25);
    println!("{:#?}", cube);
}
