//we are going to use struct and enum
enum IP{
    v4,
    v6
}
struct ipaddress{
    kind:IP,
    ip:String
}
fn main()
{
    let localhost = ipaddress{
        kind:IP::v4,
        ip:String::from("hello world")
    };
    print_ip_Details(localhost);
}
fn print_ip_Details(loc:ipaddress){
    println!("This is a normal function");
    let ip_version = match loc.kind{
        IP::v4 => "IPV4",
        IP::v6 => "IPV"
    };
    println!("{} {}",loc.ip,ip_version);
}
