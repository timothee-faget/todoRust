use crossterm::{cursor, terminal, ExecutableCommand};

use std::{error::Error, io};

use super::task;

pub struct Terminal {
    stdout: io::Stdout,
    offset: u16,
    mode: TerminalMode,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if self.mode == TerminalMode::Raw {
            self.switch_mode().unwrap();
        }
        self.leave_alternate().unwrap();
    }
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdout: io::stdout(),
            offset: 0,
            mode: TerminalMode::Classic,
        }
    }

    pub fn build() -> Result<Terminal, Box<dyn Error>> {
        let mut term = Terminal {
            stdout: io::stdout(),
            offset: 0,
            mode: TerminalMode::Classic,
        };
        term.switch_mode()?;
        term.enter_alternate()?;
        term.clear()?;
        Ok(term)
    }

    pub fn switch_mode(&mut self) -> Result<&Self, Box<dyn Error>> {
        match self.mode {
            TerminalMode::Raw => {
                terminal::disable_raw_mode()?;
                self.stdout.execute(cursor::Show)?;
                self.mode = TerminalMode::Classic;
                Ok(self)
            }
            TerminalMode::Classic => {
                terminal::enable_raw_mode()?;
                self.stdout.execute(cursor::Hide)?;
                self.mode = TerminalMode::Raw;
                Ok(self)
            }
        }
    }

    pub fn enter_alternate(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.offset = 0;
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn move_to_line(&mut self, line: u16) -> Result<(), Box<dyn Error>> {
        self.stdout.execute(cursor::MoveTo(0, line))?;
        Ok(())
    }

    pub fn print_sep(&mut self) -> Result<(), Box<dyn Error>> {
        self.move_to_line(self.offset)?;
        let sep = "=".repeat(terminal::size()?.0.into());
        println!("{}", sep);
        self.offset += 1;
        Ok(())
    }

    pub fn print_app(&mut self) -> Result<(), Box<dyn Error>> {
        println!("todoRust");
        self.offset += 1;
        self.print_sep()?;
        Ok(())
    }

    pub fn print_help(&mut self) -> Result<(), Box<dyn Error>> {
        let help_lines = vec![
            "",
            "Bienvenu dans todoRust, une simple todo-list en Rust",
            "",
            "\t-Déplacez vous avec les flèches du clavier",
            "\t-Complétez une tâche avec la touche 'Entrée'",
            "\t-Ajoutez une tâche avec la touche 'a'",
            "\t-Supprimez une tâche avec la touche 'd'",
            "\t-Affichez l'aide avec la touche 'h'",
            "\t-Quittez avec la touche 'q'",
            "",
        ];
        for line in help_lines {
            self.move_to_line(self.offset)?;
            println!("{line}");
            self.offset += 1;
        }
        self.print_sep()?;
        Ok(())
    }

    pub fn print_task(
        &mut self,
        task: &task::Task,
        index: &u16,
        selected: &u16,
    ) -> Result<(), Box<dyn Error>> {
        self.move_to_line(self.offset)?;
        if index == selected {
            print!("> ");
        } else {
            print!("  ");
        }
        println!("{task}");
        self.offset += 1;
        Ok(())
    }

    pub fn ask_new_task(&mut self) -> Result<task::Task, Box<dyn Error>> {
        self.move_to_line(self.offset)?;
        println!("Nouvelle tâche :");
        self.move_to_line(self.offset + 1)?;
        self.switch_mode()?;

        let mut title = String::new();
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");

        self.switch_mode()?;
        Ok(task::Task::new(title.as_str()))
    }
}

#[derive(Debug, PartialEq)]
enum TerminalMode {
    Raw,
    Classic,
}

#[cfg(test)]
mod tests {
    use crate::utils::task;

    use super::*;

    #[test]
    fn new_terminal() {
        let term = Terminal::new();

        assert!(term.offset == 0)
    }

    #[test]
    fn build_terminal() {
        let term = Terminal::build();

        assert!(term.is_ok())
    }

    #[test]
    fn switch_mode() {
        let mut term = Terminal::new();

        assert!(term.switch_mode().is_ok());
        assert_eq!(term.mode, TerminalMode::Raw);
        assert!(term.switch_mode().is_ok());
        assert_eq!(term.mode, TerminalMode::Classic)
    }

    #[test]
    fn alterate_screen() {
        let mut term = Terminal::new();

        assert!(term.enter_alternate().is_ok());
        assert!(term.leave_alternate().is_ok())
    }

    #[test]
    #[ignore]
    fn clear_terminal() {
        let mut term = Terminal::new();

        term.print_app().unwrap();
        assert!(term.clear().is_ok());
        assert_eq!(term.offset, 0)
    }

    #[test]
    fn move_to_line() {
        let mut term = Terminal::build().unwrap();
        assert!(term.move_to_line(0).is_ok());
        assert!(term.move_to_line(2).is_ok());
    }

    #[test]
    fn print_sep() {
        let mut term = Terminal::build().unwrap();

        assert!(term.print_sep().is_ok());
        assert_eq!(term.offset, 1)
    }

    #[test]
    fn print_app() {
        let mut term = Terminal::build().unwrap();

        assert!(term.print_app().is_ok());
        assert_eq!(term.offset, 2)
    }

    #[test]
    fn print_help() {
        let mut term = Terminal::build().unwrap();

        assert!(term.print_help().is_ok());
        assert_eq!(term.offset, 11)
    }

    #[test]
    fn print_task() {
        let mut term = Terminal::build().unwrap();
        let task = task::Task::new("Bonjour");
        let i = 0;
        let selected = 1;

        assert!(term.print_task(&task, &i, &selected).is_ok());
        assert_eq!(term.offset, 1);
    }

    #[test]
    #[ignore]
    fn ask_new_task() {
        let mut term = Terminal::build().unwrap();
        let task = task::Task::new("Bonjour");
        let i = 0;
        let selected = 0;

        term.print_app().unwrap();
        term.print_task(&task, &i, &selected).unwrap();
        assert!(term.ask_new_task().is_ok())
    }
}
