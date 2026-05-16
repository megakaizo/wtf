use std::io::{Stdout, Write};
use crossterm::{
    style::Print, queue, cursor::MoveTo
}; 


pub fn show_menu(stdout: &mut Stdout) {
    queue!(
        stdout,
        MoveTo(10, 5),
        Print("War. Territory. Fortress."),

        MoveTo(10, 7),
        Print("[SPACE] -> START | [ESC] -> EXIT"),
    ).unwrap();
    stdout.flush().unwrap();
}


