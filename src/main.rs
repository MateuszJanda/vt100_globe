// use std::time::Duration;
// use tokio::time::sleep;
use std::default::Default;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

use std::fs;

const ESC: u8 = 0x1b;
const CSI: u8 = 0x5b; // [

#[derive(PartialEq)]
enum State {
    CSI,
    Unknown,
}

enum Command {
    MoveToStart,
    ClearScreen,
    None,
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

    fn is_ControlCode(ch: u8) -> bool {
        ch == ESC
    }

    fn add(&mut self, ch: u8) -> Option<Command> {
        if self.state == State::Unknown && ch == '[' as u8 {
            self.state = State::CSI;
            // self.transition();
            return None;
        } else if self.state == State::CSI && ch == 'H' as u8 {
            return Some(Command::MoveToStart);
        } else if self.state == State::CSI && ch == 'J' as u8 {
            return Some(Command::ClearScreen);
        }

        None
    }

    fn reset(&mut self) {
        self.state == State::Unknown;
    }

    // fn transition(&mut self) {

    // }
}

#[tokio::main]
async fn main() {
    // sleep(Duration::from_millis(100)).await;

    let content = fs::read_to_string("globe.vt").unwrap();

    let lines: Vec<&str> = content.split("\n").collect();
    // let words = content.split("\n").collect();

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut control_code = ControlCode::new();

    for line in lines {
        let l = line.as_bytes();

        // Look for ControlCode
        // let mut cmd_start = 0;
        // let mut cmd_end = 0;
        let mut text_start = 0;
        let mut text_end = 0;

        let mut is_text = true;
        for (pos, ch) in l.iter().enumerate() {
            if is_text && ControlCode::is_ControlCode(*ch) {
                is_text = false;
                text_end = pos;

                control_code.reset();

                let text = &l[text_start..text_end];
                // print_text(Command::None, text);

                // cmd_start = pos;
                // cmd_end = pos;
                // text_start = pos;
                // text_end = pos;
                continue;
            }

            if is_text {
                text_end = pos;
                // cmd_start = pos;
            } else {
                let command = control_code.add(*ch);

                if command.is_some() {
                    // let text = &l[text_start..text_end];

                    // print_text(command.unwrap(), text);

                    control_code.reset();
                    is_text = true;
                }

                // let cmd = &l[cmd_start..cmd_end];

                // print_text(cmd, text);

                // cmd_start = pos + 1;
            }
        }
    }

    // println!("100 ms have elapsed");
    // let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(stdout, "Hey there.").unwrap();
    // write!(stdout, "Hey \nthere.").unwrap();
    // write!(stdout, "\\033[32mThis is in green\\033[0m").unwrap();
}

fn print_text(text: &[u8]) {}
