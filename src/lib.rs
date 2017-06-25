extern crate rustbox;

use std::collections::LinkedList;

use rustbox::{Color, RustBox};

pub struct Screen {
    pub rustbox: RustBox,
    input: Vec<String>,
    history_position: usize,
    scrollback: LinkedList<String>,
    update_needed: bool,
    line_height: usize,
    input_height: usize,
    output_height: usize,
}

impl Screen {
    pub fn new() -> Screen {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };
        let line_height = (rustbox.height() as f32 * (9.0 / 10.0)) as usize;
        let mut scrollback = LinkedList::new();
        scrollback.push_front(String::new());
        let mut input = Vec::with_capacity(100);
        input.push(String::new());
        Screen {
            rustbox: rustbox,
            input: input,
            history_position: 0,
            scrollback: scrollback,
            update_needed: true,
            line_height: line_height,
            input_height: line_height + 2,
            output_height: line_height,
        }
    }

    pub fn write_input(&mut self, c: char) {
        self.input[self.history_position].push(c);
        self.update_needed = true;
    }

    pub fn history_up(&mut self) {
        if self.history_position > 0 {
            self.history_position -= 1;
            self.update_needed = true;
        }
    }

    pub fn history_down(&mut self) {
        if self.history_position < self.input.len() - 2 {
            self.history_position += 1;
            self.update_needed = true;
        }
    }

    pub fn write_output(&mut self, s: String) {
        let lines = s.split("\n");
        for (i, line) in lines.enumerate() {
            if i == 0 {
                if let Some(front) = self.scrollback.front_mut() {
                    front.push_str(line);
                }
            } else {
                self.scrollback.push_front(line.to_string());
            }
        }
        if self.scrollback.len() > self.output_height {
            self.scrollback.pop_back();
        }
        self.update_needed = true;
    }

    pub fn delete(&mut self) {
        self.input[self.history_position].pop();
        self.update_needed = true;
    }

    pub fn enter(&mut self) -> String {
        let ret = self.input[self.history_position].clone();
        self.write_output("\n> ".to_string() + ret.as_str() + "\n");
        if self.scrollback.len() > self.output_height {
            self.scrollback.pop_back();
        }
        self.history_position = self.input.len();
        self.input.push(String::new());
        self.update_needed = true;
        ret
    }

    pub fn update(&mut self) {
        if self.update_needed {
            self.rustbox.clear();
            self.draw();
            for (i, line) in self.scrollback.iter().enumerate() {
                self.rustbox.print(1,
                               self.line_height - 1 - i,
                               rustbox::RB_NORMAL,
                               Color::Default,
                               Color::Default,
                               line.as_str());
            }
            self.rustbox.print(3,
                               self.input_height,
                               rustbox::RB_NORMAL,
                               Color::Default,
                               Color::Default,
                               self.input[self.history_position].as_str());
            self.rustbox.present();
        }
        self.update_needed = false;
    }

    pub fn draw(&mut self) {
        for x in 0..self.rustbox.width() {
            self.rustbox.print_char(x,
                                    self.line_height,
                                    rustbox::RB_NORMAL,
                                    Color::Default,
                                    Color::Default,
                                    '─');
            self.rustbox.print(1,
                               self.input_height,
                               rustbox::RB_NORMAL,
                               Color::Default,
                               Color::Default,
                               "> ");
        }
    }
}