use crossterm::event::{self, Event, KeyCode};
use exitfailure::ExitFailure;

use crate::app::{self, App, Mode};
use crate::view::bindings::Bindings;

pub fn handle_event(app: &mut App, music_database: &str, bindings: &Bindings) -> Result<bool, ExitFailure> {
    let mut is_loop = true;

    if app.error != None {
        app.error = None;
    }
    
    //Handle input
    if let Event::Key(key) = event::read().unwrap() {
        if app.mode == Mode::Browse {
            match key.code {
                code if code == bindings.exit => is_loop = false,
                code if code == bindings.move_select_top => app.move_select_top(),
                code if code == bindings.move_select_bottom => app.move_select_bottom(),
                KeyCode::Down => app.move_select_down(1),
                code if code == bindings.move_select_down => app.move_select_down(1),
                code if code == bindings.move_select_down_tab => app.move_select_down(5),
                KeyCode::Up => app.move_select_up(1),
                code if code == bindings.move_select_up => app.move_select_up(1),
                code if code == bindings.move_select_up_tab => app.move_select_up(5),
                code if code == bindings.next_page => app.next_page(),
                code if code == bindings.previous_page => app.previous_page(),
                code if code == bindings.open_folder => app.open_folder(),
                KeyCode::Right => app.open_folder(),
                code if code == bindings.previous_folder => app.back_previous_folder(music_database),
                KeyCode::Left => app.back_previous_folder(music_database),
                code if code == bindings.add_music_to_list => app.add_music_to_list(),
                code if code == bindings.start_and_pause => app.stop_or_start_play(),
                code if code == bindings.volume_down => app.update_volume(&|v| if v > 0.0 {v - 0.05} else {0.0}),
                code if code == bindings.volume_up => app.update_volume(&|v| if v < 1.25 {v + 0.05} else {1.25}),
                KeyCode::Char('+') => app.update_volume(&|v| if v < 1.25 {v + 0.05} else {1.25}),
                //code if code == bindings.search_mode => app.set_mode(Mode::Search),
                code if code == bindings.command_mode => app.set_mode(Mode::Command),
                KeyCode::Esc => {
                    app.populate_files()?;
                    app.search_buffer = Vec::new();
                }
                _ => {}
            }
        }

        if app.mode == Mode::Search {
            match key.code {
                KeyCode::Char(chr) => {
                    app.add_to_search_buffer(chr);
                    app.execute_search();  // Trigger immediately after typing
                },
                KeyCode::Backspace => {
                    if !app.search_buffer.is_empty() {
                        app.search_buffer.truncate(app.search_buffer.len() - 1);
                        app.execute_search();  // Trigger immediately after deletion
                    }
                },
                KeyCode::Esc => {
                    app.set_mode(Mode::Browse);
                    app.search_buffer.clear();
                    app.search_buffer = Vec::new();
                },
                _ => {}
            }
        } else if key.code == bindings.search_mode {
            app.set_mode(Mode::Search);
            app.search_buffer.clear();
            app.add_to_search_buffer(':');
        } 

        if app.mode == app::Mode::Command {
            match key.code {
                KeyCode::Char(chr) => app.add_to_command_buffer(chr),
                KeyCode::Enter => app.execute_command(),
                KeyCode::Backspace => {
                    if app.command_buffer.len() > 1 {
                        app.command_buffer.truncate(app.command_buffer.len() - 1);
                    };
                }
                KeyCode::Esc => {
                    app.set_mode(app::Mode::Browse);
                    app.command_buffer = Vec::new();
                }
                _ => {}
            }
        }
    }

    Ok(is_loop)
}
