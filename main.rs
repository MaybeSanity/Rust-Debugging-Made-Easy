/*
[dependencies]
chrono = "0.4.13"
ansi_term = "0.12.1"
*/

extern crate chrono;
extern crate ansi_term;

use chrono::Local;
use ansi_term::*;
#[allow(unused_imports)]
use std::{thread, time};

fn main() {
    DbgPrint(Type::Debug, "Hello world");
    std::thread::sleep(time::Duration::from_secs(5));
    DbgPrint(Type::Note, "Hello world");
    std::thread::sleep(time::Duration::from_secs(5));
    DbgPrint(Type::Warn, "Hello world");
    std::thread::sleep(time::Duration::from_secs(5));
    DbgPrint(Type::Error, "Hello world");
}

pub enum Type {
    Debug,
    Note,
    Warn,
    Error
}

#[allow(non_snake_case)]
fn DbgPrint(print_type: Type, msg: &str) {
    let current_time = Local::now();

    let color_debug = Style::new().fg(Color::Green).paint(msg);
    let color_note = Style::new().fg(Color::Blue).paint(msg);
    let color_warn = Style::new().fg(Color::Yellow).paint(msg);
    let color_error = Style::new().fg(Color::Red).paint(msg);

    match print_type {
        Type::Debug => println!("[DEBUG: {}]\n{}", current_time.format("%Y-%m-%d][%H:%M:%S"), color_debug),
        Type::Note => println!("[NOTE: {}]\n{}", current_time.format("%Y-%m-%d][%H:%M:%S"), color_note),
        Type::Warn => println!("[WARN: {}]\n{}", current_time.format("%Y-%m-%d][%H:%M:%S"), color_warn),
        Type::Error => println!("[ERROR: {}]\n{}", current_time.format("%Y-%m-%d][%H:%M:%S"), color_error)
    }
}
