struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main(){
    let user1 = User{
        email : String::from("sabyasachibanerjee415@gmail.com"),
        username : String::from("Sabyasachi Banerjee"),
        active  : true,
        sign_in_count : 45
    };
    println!("{}",user1.active);
    let mut abc:User = create_user("dummy_username".to_string(),"dummy_email".to_string());
    println!("{}",abc.email);
   //trying to modify any props of abc
    abc.email = "fake_override".to_string();
    println!("{}",abc.email);
}
fn create_user(username:String,email:String)-> User{
    let returned_User = User{
        username,
        email,
        active:false,
        sign_in_count:4
    };
    return  returned_User;
}
