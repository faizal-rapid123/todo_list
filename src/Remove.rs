use std::io;
pub fn remove(notes:&mut Vec<String>)
{
  loop
  {

    println!("enter the task number that you want to remove or enter 'exit' to exit from remove manager");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("");
    let task:String = task.trim().parse().unwrap();
    if task == "exit" { break;}
    let task:usize = match task.parse()
    {
        Ok(t)=>t,
        err=>{println!("number should be a integer and in range : {:?}",err); continue;}
    };
    if task >= 0 as usize && task< notes.len()
    {
          for i in task..notes.len()-1
          {
            notes[i] = notes[i+1].clone();
          }
          notes.pop();
    }
    else  {
      println!("the task query must be in range");
      continue;
        
    }


  }
   

   
}