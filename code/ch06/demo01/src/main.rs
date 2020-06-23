fn main() {
    println!("Hello, world!");
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // println!("home.king: {}", home.kind);
    println!("home.addresss: {}", home.address);
    // println!("{}", home.kind);
    // println!("{}", home.kind);
    println!("loopback.addresss: {}", loopback.address);

}

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
