// use std::time::Duration;
// use tokio::time::sleep;
use std::fs;

const ESC: u8 = 0x1b;
const CSI: u8 = 0x5b; // [


struct Command {

}

impl Command {
    fn new() -> Self {
        Command {  }
    }

    fn add(&mut self, ch: u8) {
        if ch == CSI {
            self.transition();
        }
    }

    fn transition(&mut self) {

    }
}


#[tokio::main]
async fn main() {
    // sleep(Duration::from_millis(100)).await;

    let content = fs::read_to_string("globe.vt").unwrap();

    let lines: Vec<&str> = content.split("\n").collect();
    // let words = content.split("\n").collect();

    for line in lines {
        let l = line.as_bytes();

        // Look for command
        let mut cmd_start = 0;
        let mut cmd_end = 0;
        let mut text_start = 0;
        let mut text_end = 0;

        let mut is_text = true;
        for (pos, ch) in l.iter().enumerate() {
            if is_text && *ch == ESC {
                is_text = false;
                text_end = pos;



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



                let text = &l[text_start..text_end];
                let cmd = &l[cmd_start..cmd_end];


                print_text(cmd, text);


                cmd_start = pos + 1;
            }
        }
    }

    println!("100 ms have elapsed");
}

fn print_text(cmd: &[u8], text: &[u8]) {}
