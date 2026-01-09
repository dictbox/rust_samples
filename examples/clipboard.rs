use clipboard_win::{formats, set_clipboard};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let text: &String = &args[1];
        set_clipboard(formats::Unicode, text).expect("to set clipboard");
    }
}


