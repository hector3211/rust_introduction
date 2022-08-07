enum IpAddKind {
    v4(u8, u8, u8, u8),
    v6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_func() {
        println!("some_func");
    }
}
struct IpAddr {
    kind: IpAddKind,
    address: String,
}
fn main() {
    let four = IpAddKind::v4;
    let sixe = IpAddKind::v6;
    let localhost = IpAddKind::v4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddKind) {}
