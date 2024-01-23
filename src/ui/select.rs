use std::fmt::Display;
use std::io::{stdout, Write};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::traits::has_id::HasId;

pub struct Select<T> where T: Clone + Display + HasId {
    prompt: String,
    items: Vec<T>,
    arrow_pos: usize,
    marked: Vec<T>,
}

impl<T: Clone + Display + HasId> Select<T> {
    pub fn new() -> Select<T> {
        Select {
            prompt: "".to_string(),
            items: Vec::new(),
            arrow_pos: 0,
            marked: Vec::new(),
        }
    }

    pub fn with_prompt(mut self, prompt: &str) -> Select<T> {
        self.prompt = prompt.to_string();
        self
    }

    pub fn default(mut self, default: usize) -> Select<T> {
        self.arrow_pos = default;
        self
    }

    pub fn items(mut self, items: &[T]) -> Select<T> {
        self.items.extend_from_slice(items);
        self
    }

    pub fn set_marked(mut self, marked: Vec<T>) -> Select<T> {
        self.marked = marked;
        self
    }

    pub fn get_marked(&self) -> Vec<T> {
        self.marked.clone()
    }

    fn mark(&mut self, id: i32) {
        if let Some(item) = self.items.iter().find(|x| x.get_id() == id) {
            self.marked.push(item.clone());
        }
    }

    fn unmark(&mut self, id: i32) {
        self.marked.retain(|x| x.get_id() != id);
    }

    fn toggle_mark(&mut self, id: i32) {
        if self.marked.iter().any(|x| x.get_id() == id) {
            self.unmark(id);
        } else {
            self.mark(id);
        }
    }

    pub fn print_prompt(&mut self) {
        std::process::Command::new("clear").status().unwrap();

        let mut prompt = self.prompt.clone(); // Clone the prompt to make it mutable
        prompt.push_str("\n");

        for (i, item) in self.items.iter().enumerate() {
            let mark = if self.marked.iter().any(|x| x.get_id() == item.get_id()) {
                "x".to_string()
            } else {
                " ".to_string()
            };

            let line = format!("[{}] {}\n", mark, item.to_string());

            if i == self.arrow_pos {
                prompt.push_str(&format!("> {}", line.green()));
                continue;
            }

            prompt.push_str(&format!("  {}", line));
        }

        write!(stdout(), "{}", prompt).unwrap();
    }

    pub fn run(mut self) -> Select<T> {
        self.print_prompt();

        loop {
            enable_raw_mode().unwrap();
            let event = crossterm::event::read().unwrap();
            match event {
                crossterm::event::Event::Key(KeyEvent {
                                                 code,
                                                 modifiers,
                                                 ..
                                             }) => {
                    match (code, modifiers) {
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if self.arrow_pos > 0 {
                                self.arrow_pos -= 1;
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if self.arrow_pos < self.items.len() - 1 {
                                self.arrow_pos += 1;
                            }
                        }
                        (KeyCode::Char(' '), KeyModifiers::NONE) => {
                            self.toggle_mark(self.items[self.arrow_pos].get_id());
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            disable_raw_mode().unwrap();
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            disable_raw_mode().unwrap();
            self.print_prompt();
        }

        return self;
    }
}