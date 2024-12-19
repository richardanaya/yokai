use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, event, terminal};
use std::time::Duration;

struct CleanUp;

mod entities;

use entities::{Entity, EntityName, Item, Land, Map, Monster, Player};

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        print!("{}", crossterm::cursor::Show);
    }
}

fn main() -> crossterm::Result<()> {
    let mut map = Map::new(terminal::size().expect("Could not get terminal size"));
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    print!("{}", crossterm::cursor::Hide);

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                let mut should_render = true;
                match event {
                    KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => break,
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(-1, 0),
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(1, 0),
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(0, -1),
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(0, 1),
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(-1, -1),
                    KeyEvent {
                        code: KeyCode::Char('e'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(1, -1),
                    KeyEvent {
                        code: KeyCode::Char('z'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(-1, 1),
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(1, 1),
                    KeyEvent {
                        code: KeyCode::Char('C'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => {
                        map.show_stats = !map.show_stats;
                    },
                    KeyEvent {
                        code: KeyCode::Char('x'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => map.move_player(0, 0),
                    _ => {
                        should_render = false;
                    }
                }

                if should_render {
                    map.render(terminal::size().expect("Could not get terminal size"));
                }
            };
        } else {
            map.render(terminal::size().expect("Could not get terminal size"));
        }
    }
    Ok(())
}
