use super::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Input {
    pub keys_pressed: HashMap<Key, bool>,
    pub keys_down: HashMap<Key, bool>,
    pub keys_up: HashMap<Key, bool>,
    pub mouse_position: Vector2,
    pub dt: f32,
}

impl Default for Input {
    fn default() -> Input {
        let mut keys_pressed = HashMap::new();
        keys_pressed.insert(Key::W, false);
        keys_pressed.insert(Key::A, false);
        keys_pressed.insert(Key::S, false);
        keys_pressed.insert(Key::D, false);
        keys_pressed.insert(Key::Space, false);
        keys_pressed.insert(Key::Mouse1, false);
        keys_pressed.insert(Key::Mouse2, false);

        let mut keys_down = HashMap::new();
        keys_down.insert(Key::W, false);
        keys_down.insert(Key::A, false);
        keys_down.insert(Key::S, false);
        keys_down.insert(Key::D, false);
        keys_down.insert(Key::Space, false);
        keys_down.insert(Key::Mouse1, false);
        keys_down.insert(Key::Mouse2, false);
        let mut keys_up = HashMap::new();
        keys_up.insert(Key::W, false);
        keys_up.insert(Key::A, false);
        keys_up.insert(Key::S, false);
        keys_up.insert(Key::D, false);
        keys_up.insert(Key::Space, false);
        keys_up.insert(Key::Mouse1, false);
        keys_up.insert(Key::Mouse2, false);

        Input {
            keys_pressed: keys_pressed,
            keys_down: keys_down,
            keys_up: keys_up,
            mouse_position: Vector2::zeros(),
            dt: 0.0,
        }
    }
}

impl Input {
    pub fn reset(&mut self) {
        self.keys_down.insert(Key::W, false);
        self.keys_down.insert(Key::A, false);
        self.keys_down.insert(Key::S, false);
        self.keys_down.insert(Key::D, false);
        self.keys_down.insert(Key::Space, false);
        self.keys_down.insert(Key::Mouse1, false);
        self.keys_down.insert(Key::Mouse2, false);
        self.keys_up.insert(Key::W, false);
        self.keys_up.insert(Key::A, false);
        self.keys_up.insert(Key::S, false);
        self.keys_up.insert(Key::D, false);
        self.keys_up.insert(Key::Space, false);
        self.keys_up.insert(Key::Mouse1, false);
        self.keys_up.insert(Key::Mouse2, false);
        // do not reset mouse.
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Key {
    W,
    A,
    S,
    D,
    Space,
    Mouse1,
    Mouse2,
}
