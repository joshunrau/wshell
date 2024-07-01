use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn exec(command: &str) -> String {
    let mut args: VecDeque<&str> = {
        let mut v = Vec::new();
        for elem in command.trim().split_whitespace() {
            v.push(elem);
        }
        v.into()
    };
    let program = args.pop_front().unwrap_or("");
    match program {
        "echo" => Vec::from(args).join(" "),
        _ => String::from(&format!("wshell: command not found: {program}")),
    }
}
