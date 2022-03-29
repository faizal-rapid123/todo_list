 use std::collections::HashMap;
 mod Login;
 mod Create_user;
 mod Show;
 mod Add;
 mod Remove;

 use std::io;
 struct _Usernotes // implemet this structure later
 {
     user_name:String,
     notes:Vec<String>
 }
 fn main()
 
 {
    loop{
     let mut userpass = HashMap::new();
     let mut userdata:HashMap<String,_Usernotes > = HashMap::new();
     userpass.insert("username".to_string(),"password".to_string());
     let ans = Login::login();
     let mut  UserName = String::new();
     
     
     let ans = ans.trim();
     if ans == "N"
     {
        let user_data =  Create_user::c_user();
        userpass.insert(user_data[0].clone(),user_data[1].clone());
        let arr:Vec<String>  = Vec::new();
        userdata.insert(user_data[0].clone(),_Usernotes{user_name:user_data[0].clone(),notes:arr});
        UserName = user_data[0].clone();
     }
     else 
     {
         loop
              {
                      let mut correct =  false;
                      let mut username = String::new();
                      let mut pass = String::new();
                      println!("plese enter your existing username");
                      io::stdin().read_line(&mut username).expect(" ");
                      let username:String =  username.trim().parse().unwrap();
                      println!("plese enter your existing password for {} aaa",username);
                      io::stdin().read_line(&mut pass).expect(" ");
                      let pass:String =  pass.trim().parse().unwrap();
                      if userpass.contains_key( &username) == true
                             {
                                let passvalue = userpass.get(&username).unwrap();
                                         if passvalue == &pass 
                                                  {
                                                        correct = true;
                                                  }
                                         else {
                                                        correct = false;
                                                  }
                             }
                    if correct == true { println!("welcome back {}",&username);UserName = username;break; }
                    let mut tri = String::new();
                    println!("username or password is wrong");
                    println!("press R to retry or press anything to exit");
                    io::stdin().read_line(&mut tri).expect(" ");
                    let tri:String = tri.trim().parse().unwrap();
                    if tri.as_str() == "R" { continue;}
                    else { panic!("you have exited the program"); }

        
            

     }
}
let data = userdata.get_mut(&UserName).unwrap();
if data.notes.len() == 0
     {
            println!(" wow!!seems your list is empty");
     }
     {
         Show::show(&mut data.notes);
     }
     let mut apple =0; 
 loop{
            println!("enter queries 'show', 'remove', 'add','exit','login_page'");
            let mut query = String::new();
            io::stdin().read_line(&mut query).expect(" ");
            let query:String = query.trim().parse().unwrap();
            if query == "show"
               {
                Show::show(&mut data.notes);
                }
            if query == "add"
               {
                Add::add(&mut data.notes);
                }
            if query == "remove"
            {
                Remove::remove(&mut data.notes);
            }
            if query == "exit"
            {
                break;
            }
            if query =="login_page"
            {
                apple= 1;
                break;
            }
            else {
                
                continue;

            }

        }
        if apple == 1 {continue;}
        else {break;}
    }
       
    }

           
     


     





     


 
        
        
    