#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

macro_rules! color_text {
    ($text:expr, $color:expr) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $color, $text)
    }};
}

#[no_mangle]
pub fn main() -> i32 {
    println!(
        "{}{}{}{}{} {}{}{}{} {}{}{}{}{}{}",
        color_text!("H", 31),
        color_text!("e", 32),
        color_text!("l", 33),
        color_text!("l", 34),
        color_text!("o", 35),
        color_text!("R", 36),
        color_text!("u", 37),
        color_text!("s", 90),
        color_text!("t", 91),
        color_text!("r", 92),
        color_text!("C", 93),
        color_text!("o", 94),
        color_text!("r", 95),
        color_text!("e", 96),
        color_text!("!", 97),
    );

    println!("{}", color_text!("    k       k    k    k       kk          kkk    ", 90));
    println!("{}", color_text!("     k     k     k    k      k  k        k       ", 91));
    println!("{}", color_text!("      k   k      k    k     k    k     k         ", 92));
    println!("{}", color_text!("       k k       k    k    k      k     k        ", 93));
    println!("{}", color_text!("        k        kkkkkk    k      k      k       ", 94));
    println!("{}", color_text!("        k        k    k    k      k        k     ", 95));
    println!("{}", color_text!("        k        k    k     k    k          k    ", 96));
    println!("{}", color_text!("        k        k    k      k  k          k     ", 97));
    println!("{}", color_text!("        k        k    k       kk       kkk       ", 98));
    0
}
