use std::fs;
use std::io::{stdout, Write};
use termion::screen::AlternateScreen;
use tokio::time::Duration;

const ESC: u8 = 0x1b;

#[derive(PartialEq)]
enum State {
    Unknown,
    Command,
    CSI,
}

enum Command {
    MoveToStart,
    ClearScreen,
}

struct CommandFsm {
    state: State,
}

impl CommandFsm {
    fn new() -> Self {
        CommandFsm {
            state: State::Unknown,
        }
    }

    fn is_new_command(ch: u8) -> bool {
        ch == ESC
    }

    fn add(&mut self, ch: u8) -> Option<Command> {
        if self.state == State::Unknown && CommandFsm::is_new_command(ch) {
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
    let content = fs::read_to_string("globe.vt").unwrap();
    let lines: Vec<&str> = content.split("\n").collect();

    let mut screen = AlternateScreen::from(stdout());
    let mut command_fsm = CommandFsm::new();

    let mut line_num: u16 = 1;
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    interval.tick().await;
    for line in lines.iter().cycle() {
        let line_bytes = line.as_bytes();

        let mut text_start = 0;
        let mut is_text = true;
        for (pos, ch) in line_bytes.iter().enumerate() {
            if is_text && CommandFsm::is_new_command(*ch) {
                command_fsm.add(*ch);
                is_text = false;

                let text_end = pos;
                let text = &line_bytes[text_start..text_end];
                write!(
                    screen,
                    "{}{}{}",
                    termion::cursor::Hide,
                    termion::cursor::Goto(1, line_num),
                    std::str::from_utf8(text).unwrap()
                )
                .unwrap();
            } else if !is_text {
                let command = command_fsm.add(*ch);

                match command {
                    None => (),
                    Some(Command::MoveToStart) => {
                        screen.flush().unwrap();
                        write!(screen, "{}", termion::clear::All).unwrap();
                        line_num = 1;
                        interval.tick().await;
                    }
                    Some(Command::ClearScreen) => {
                        write!(screen, "{}", termion::clear::All).unwrap();
                    }
                }

                if command.is_some() {
                    command_fsm.reset();
                    is_text = true;
                    text_start = pos + 1;
                }
            }
        }

        if is_text {
            let text_end = line_bytes.len();
            let text = &line_bytes[text_start..text_end];
            write!(
                screen,
                "{}{}{}",
                termion::cursor::Hide,
                termion::cursor::Goto(1, line_num),
                std::str::from_utf8(text).unwrap()
            )
            .unwrap();
        }

        line_num += 1;
    }
}
