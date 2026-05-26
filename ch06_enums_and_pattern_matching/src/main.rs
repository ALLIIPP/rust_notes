
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {
    let my_ip = IpAddr::V4(127,0,0,1);
    let other_ip = IpAddr::V6(String::from("2001::1"));

    route(&my_ip);
    route(&other_ip);

    let loopback = String::from("2001::1");
    if let IpAddr::V6(loopback) = other_ip {
        println!("its an ipv6 loopback address");
    }

}

fn route(ip_addr: &IpAddr){

    let address = match ip_addr {
        IpAddr::V4(u1, u2, u3, u4) => &format!("{},{},{},{}",u1,u2,u3,u4)[0..],
        IpAddr::V6(s) => s
    };

    println!("routing... {}", address);
}
