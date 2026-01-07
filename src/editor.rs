//use std::io::{self, Read};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {

}

impl Editor {
    pub fn default() -> Self {
        Editor{} // default constructor, returns an editor. I guess we can skip this if we want.
    }
    pub fn run(&self) {
        let _ = enable_raw_mode();

        // use crossterm for input instead of stdin,
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");

                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => {
                    println!("Error: {err}");
                },
                _ => ()
            }
        }


        // for b in io::stdin().bytes() {
        //     match b {
        //       Ok(b) => {
        //             let c = b as char;
        //             if c.is_control() {
        //                 println!("Binary: {0:08b} Ascii: {0:#03} \r", b);
        //             } else {
        //                 println!("Binary: {0:08b} Ascii: {0:#03} Character: {1:#?} \r", b, c);
        //             }
        //             if c == 'q' {
        //                 break;
        //             }
        //         },
        //         Err(err) => {
        //             println!("Error: {}", err);
        //         }
        //     }

        // }
        let _ = disable_raw_mode();

    }
}

