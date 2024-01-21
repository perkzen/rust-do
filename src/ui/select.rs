use std::fmt::Display;
use std::io::{stdout, Write};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Select<T> where T: Clone + Display {
    prompt: String,
    items: Vec<T>,
    arrow_pos: usize,
    marked: Vec<usize>,
}

impl<T: Clone + Display> Select<T> {
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

    fn mark(&mut self, index: usize) {
        if !self.marked.contains(&index) {
            self.marked.push(index);
        }
    }

    fn unmark(&mut self, index: usize) {
        if let Some(pos) = self.marked.iter().position(|x| *x == index) {
            self.marked.remove(pos);
        }
    }

    fn toggle_mark(&mut self, index: usize) {
        if self.marked.contains(&index) {
            self.unmark(index);
        } else {
            self.mark(index);
        }
    }



    pub fn print_prompt(&mut self) {
        std::process::Command::new("clear").status().unwrap();

        let mut prompt = self.prompt.clone(); // Clone the prompt to make it mutable
        prompt.push_str("\n");

        for (i, item) in self.items.iter().enumerate() {
            let line = format!("{}\n", item.to_string());

            if i == self.arrow_pos {
                prompt.push_str(&format!("> {}", line.green()));
                continue;
            }

            prompt.push_str(&format!("  {}", line));
        }

        write!(stdout(), "{}", prompt).unwrap();
    }

    pub fn run(mut self) {
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
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            self.toggle_mark(self.arrow_pos);
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
    }
}