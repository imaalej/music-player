use crossterm::event::KeyCode;

use crate::config::InitBindings;

pub fn parse_keycode(s: &str) -> KeyCode {
    match s {
        "Backspace" => KeyCode::Backspace,
        "Enter" => KeyCode::Enter,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "PageUp" => KeyCode::PageUp,
        "PageDown" => KeyCode::PageDown,
        "Tab" => KeyCode::Tab,
        "BackTab" => KeyCode::BackTab,
        "Delete" => KeyCode::Delete,
        "Insert" => KeyCode::Insert,
        "Null" => KeyCode::Null,
        "Esc" => KeyCode::Esc,
         _ => {
            if let Some(n) = s.strip_prefix('F').and_then(|num| num.parse::<u8>().ok()) {
                KeyCode::F(n)
            } else if s.len() == 1 {
                KeyCode::Char(s.chars().next().unwrap())
            } else {
                panic!("Invalid keycode: {}", s)
            }
        }
    }
}

pub struct Bindings {
    pub exit: KeyCode,
    pub move_select_top: KeyCode,
    pub move_select_bottom: KeyCode,
    pub move_select_down: KeyCode,
    pub move_select_down_tab: KeyCode,
    pub move_select_up: KeyCode,
    pub move_select_up_tab: KeyCode,
    pub next_page: KeyCode,
    pub previous_page: KeyCode,
    pub open_folder: KeyCode,
    pub previous_folder: KeyCode,
    pub add_music_to_list: KeyCode,
    pub start_and_pause: KeyCode,
    pub volume_down: KeyCode,
    pub volume_up: KeyCode,
    pub search_mode: KeyCode,
    pub command_mode: KeyCode
}

impl Bindings{
    #[rustfmt::skip]
    pub fn new(init_bindings: InitBindings) -> Bindings {
        Bindings{
            exit: parse_keycode(&init_bindings.exit),
            move_select_top: parse_keycode(&init_bindings.move_select_top),
            move_select_bottom: parse_keycode(&init_bindings.move_select_bottom),
            move_select_down: parse_keycode(&init_bindings.move_select_down),
            move_select_down_tab: parse_keycode(&init_bindings.move_select_down_tab),
            move_select_up: parse_keycode(&init_bindings.move_select_up),
            move_select_up_tab: parse_keycode(&init_bindings.move_select_up_tab),
            next_page: parse_keycode(&init_bindings.next_page),
            previous_page: parse_keycode(&init_bindings.previous_page),
            open_folder: parse_keycode(&init_bindings.open_folder),
            previous_folder: parse_keycode(&init_bindings.previous_folder),
            add_music_to_list: parse_keycode(&init_bindings.add_music_to_list),
            start_and_pause: parse_keycode(&init_bindings.start_and_pause),
            volume_down: parse_keycode(&init_bindings.volume_down),
            volume_up: parse_keycode(&init_bindings.volume_up),
            search_mode: parse_keycode(&init_bindings.search_mode),
            command_mode: parse_keycode(&init_bindings.command_mode),

        }
    }
}