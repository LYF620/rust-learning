use crossterm::event;
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        // for b in io::stdin().bytes() {
        //     match b {
        //         Ok(b) => {
        //             let c = b as char;
        //             if c.is_control() {
        //                 println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        //             } else {
        //                 println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        //             }

        //             if c == 'q' {
        //                 break;
        //             }
        //         }
        //         Err(err) => println!("error: {0}", err),
        //     }
        // }
        disable_raw_mode().unwrap();
    }
}
