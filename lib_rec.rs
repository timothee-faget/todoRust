use std::{error::Error, io};
mod utils;
use crossterm::event::{self, Event, KeyCode};
use utils::{file_io, terminal};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut term = terminal::Terminal::build().unwrap();

    let mut selected = 0;
    let mut tasks = file_io::read_task_file().unwrap();
    let mut help = false;

    loop {
        term.clear()?;
        term.print_app()?;
        if help {
            term.print_help()?;
        }

        for (i, task) in tasks.iter().enumerate() {
            let index = <usize as TryInto<u16>>::try_into(i)?;
            term.print_task(&task, &index, &selected)?;
        }

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    let max_selected = <usize as TryInto<u16>>::try_into(tasks.len() - 1)?;
                    if selected < max_selected {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    let i: usize = selected.into();
                    if let Some(task) = tasks.get_mut(i) {
                        task.complete();
                    }
                    file_io::write_task_file(&tasks)?;
                }
                KeyCode::Char('a') => {
                    // Refactor tout ça hop hop hop
                    let task = term.ask_new_task()?;
                    tasks.push(task);

                    file_io::write_task_file(&tasks).unwrap();

                    let spot = <usize as TryInto<u16>>::try_into(tasks.len()-1);
                    selected = tasks.len() -;
                }
                KeyCode::Char('d') => {
                    let _ = tasks.remove(selected.into());
                    file_io::write_task_file(&tasks).unwrap();
                }
                KeyCode::Char('h') => {
                    help = !help;
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
