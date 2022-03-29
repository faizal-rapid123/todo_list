pub fn show(notes:&mut Vec<String>)
{
    for a in 0..notes.len()
    {
        println!("{}: {}",a,notes[a]);
    }
}