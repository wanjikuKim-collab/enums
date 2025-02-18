#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    v6([u8; 16]),
}

impl IpAddrKind {
    fn some_function(&self){
        println!("I did this");
    }    
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::v6([
    0x20, 0x01, 0x00, 0x00,
    0x32, 0x3B, 0xDf, 0xE1,
    0x00, 0x63, 0x00, 0x00,
    0x00, 0xFE, 0xFB, 0xFB
    ]);

    println!("{:#?},{:?}", six, six.some_function());
}