// mod file;
// mod v2ray;
// use file::File;
// use v2ray::V2ray;

// #[tokio::main]
//struct HA {}
//let buf = File::to_buf("./config.json").await;
///let v2ray = V2ray::from_buf(&buf[..]);
//V2ray::add_client(v2ray, client)
fn greet(target: String) {
    println!("Hello, {}", target);
}
fn main() {
    let greeting = "Hello, world!";
    greet(greeting.to_string());
}
