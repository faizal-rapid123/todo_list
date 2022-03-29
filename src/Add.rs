use std::io;
pub fn add(notes:&mut Vec<String>)
{
    loop 
    {
        println!("enter task");
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect(" ");
        let data:String = data.trim().parse().expect("");
        notes.push(data);
        println!("adding success");
        println!("enter 'add' to add more or anything else to exit");
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect(" ");
        let query:String = query.trim().parse().expect("");
        if query.as_str() == "add"{ continue;}
        else { break; }



    }
  
}