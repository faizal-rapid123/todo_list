use std::io;
pub fn c_user()->Vec<String>
{
  let mut username = String::new();
  println!("enter you new username");
  io::stdin().read_line(&mut username).expect("");
  let username:String = username.trim().parse().unwrap();

  let mut password = String::new();
  println!("enter you new strong password");
  io::stdin().read_line(&mut password).expect("");
  let password:String = password.trim().parse().unwrap();
  
  let userdata = vec![username,password];
  userdata


}