use std::io;
pub fn login()->String
{
    let mut ans = String::new();
  loop{
    println!("WELCOME TO THE TO DO LIST");
    println!("Already a user? Y or N");
     ans = String::new();
    io::stdin().read_line(&mut ans).expect("");
    let ans:String = ans.trim().parse().unwrap();
    let ans = match ans.as_str()
    {
      "Y"=>"Y",
      "N"=>"N",
      _ => "unexpected"
    };
    if ans == "Y" || ans =="N"
    {
      break;
    }
    else {
      println!("unexpected input,please enter Y or N ");
        
    }

}
return ans;






}