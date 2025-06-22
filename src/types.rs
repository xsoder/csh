#[derive(Debug)]
pub enum Token {
    Echo,
    Alias,
    Ls,
    Cat,
    Clear,
    Type,
    Exit,
    Unknown,
}
#[derive(Debug)]
pub struct Buffer {
   value: String,
   status: i32,
   aliased: String,
}
