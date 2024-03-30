fn main() {
    let s1 = String::from("hello");
    let bytes = s1.as_bytes();
    for &byte in bytes{
        println!("{}",byte);
    }
}
