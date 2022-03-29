pub fn show(notes:&mut Vec<String>)
{
    if notes.len() == 0
    {
        println!("nothing to show it is empty for now");
        return;
    }
    for a in 0..notes.len()
    {
        println!("{}: {}",a,notes[a]);
    }
}