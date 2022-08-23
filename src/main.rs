// use std::time::Duration;
// use tokio::time::sleep;
use std::default::Default;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

use std::fs;

const ESC: u8 = 0x1b;

#[derive(PartialEq)]
enum State {
    CSI,
    Unknown,
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

    fn is_comand(ch: u8) -> bool {
        ch == ESC
    }

    fn add(&mut self, ch: u8) -> Option<Command> {
        if self.state == State::Unknown && ch == '[' as u8 {
            self.state = State::CSI;
            return None;
        } else if self.state == State::CSI && ch == 'H' as u8 {
            return match ch as char {
                'H' => Some(Command::MoveToStart),
                'J' => Some(Command::ClearScreen),
                _ => None,
            };
        }

        None
    }

    fn reset(&mut self) {
        self.state == State::Unknown;
    }
}

#[tokio::main]
async fn main() {
    // sleep(Duration::from_millis(100)).await;

    let content = fs::read_to_string("globe.vt").unwrap();

    let lines: Vec<&str> = content.split("\n").collect();

    let mut stdout = stdout();

    let mut control_code = ControlCode::new();

    for (line_num, line) in lines.iter().enumerate() {
        let l = line.as_bytes();

        let mut text_start = 0;
        let mut is_text = true;
        for (pos, ch) in l.iter().enumerate() {
            if is_text && ControlCode::is_comand(*ch) {
                is_text = false;
                let text_end = pos;

                control_code.add(*ch);

                let text = &l[text_start..text_end];
                print_text(line_num as i32, text);
            } else {
                let command = control_code.add(*ch);

                match command {
                    None => (),
                    Some(Command::MoveToStart) => (),
                    Some(Command::ClearScreen) => {
                        write!(stdout, "{}", termion::clear::All).unwrap()
                    }
                    _ => (),
                }

                if command.is_some() {
                    control_code.reset();
                    is_text = true;
                    text_start = pos + 1;
                }
            }
        }
    }

    // println!("100 ms have elapsed");
    // let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(stdout, "Hey there.").unwrap();
    // write!(stdout, "Hey \nthere.").unwrap();
    // write!(stdout, "\\033[32mThis is in green\\033[0m").unwrap();
}

fn print_text(line_num: i32, text: &[u8]) {}
