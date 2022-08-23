// use tokio::time::{self, Duration};
use tokio::time::{ Duration};
// use tokio::time::sleep;
use std::default::Default;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

use std::fs;

const ESC: u8 = 0x1b;

#[derive(PartialEq)]
enum State {
    Unknown,
    Command,
    CSI,
}

enum Command {
    Begin,
    MoveToStart,
    ClearScreen,
}

struct ControlCode {
    state: State,
}

impl ControlCode {
    fn new() -> Self {
        ControlCode {
            state: State::Unknown,
        }
    }

    fn is_command_char(ch: u8) -> bool {
        ch == ESC
    }

    fn add(&mut self, ch: u8) -> Option<Command> {
        if self.state == State::Unknown && ControlCode::is_command_char(ch) {
            self.state = State::Command;
            return None;
        } else if self.state == State::Command && ch == '[' as u8 {
            self.state = State::CSI;
            return None;
        } else if self.state == State::CSI {
            return match ch as char {
                'H' => Some(Command::MoveToStart),
                'J' => Some(Command::ClearScreen),
                _ => None,
            };
        }

        None
    }

    fn reset(&mut self) {
        self.state = State::Unknown;
    }
}

#[tokio::main]
async fn main() {
    // sleep(Duration::from_millis(100)).await;
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    interval.tick().await;

    let content = fs::read_to_string("globe.vt").unwrap();

    let lines: Vec<&str> = content.split("\n").collect();

    let mut stdout = stdout();

    let mut control_code = ControlCode::new();

    let mut line_num = 0;
    for line in lines.iter().cycle() {
        let l = line.as_bytes();

        let mut text_start = 0;
        let mut is_text = true;
        for (pos, ch) in l.iter().enumerate() {
            if is_text && ControlCode::is_command_char(*ch) {
                control_code.add(*ch);
                is_text = false;

                let text_end = pos;
                let text = &l[text_start..text_end];
                let s = std::str::from_utf8(text).unwrap();
                // write!(stdout, "{}", s).unwrap();
                // write!(stdout, "asdf").unwrap();
                // write!(stdout, "{}", s.len()).unwrap();
                // write!(stdout, "{} {}", text_start, text_end).unwrap();
                write!(stdout, "{} {}", termion::cursor::Goto(0, line_num as u16 + 1), s).unwrap();

            } else {
                let command = control_code.add(*ch);

                match command {
                    None => (),
                    Some(Command::MoveToStart) => {
                        line_num = 0;
                    },
                    Some(Command::ClearScreen) => {
                        stdout.flush().unwrap();
                        // write!(stdout, "{}", termion::clear::All).unwrap();
                        interval.tick().await;
                    }
                    _ => (),
                }

                if command.is_some() {
                    // write!(stdout, "asdf").unwrap();
                    control_code.reset();
                    is_text = true;
                    text_start = pos + 1;
                }
            }
        }

        if is_text {
            let text_end = l.len();
            let text = &l[text_start..text_end];
            let s = std::str::from_utf8(text).unwrap();
            write!(stdout, "{} {}", termion::cursor::Goto(0, line_num as u16 + 1), s).unwrap();

        }

        line_num += 1;

    }

    // println!("100 ms have elapsed");
    // let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(stdout, "Hey there.").unwrap();
    // write!(stdout, "Hey \nthere.").unwrap();
    // write!(stdout, "\\033[32mThis is in green\\033[0m").unwrap();
}

// fn print_text(stdout: &mut Stdout, line_num: i32, text: &[u8]) {

// }
