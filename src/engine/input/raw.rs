use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct RawInputState {
    held_keys: HashSet<KeyCode>,
}

impl RawInputState {
    pub fn set_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.held_keys.insert(key);
        } else {
            self.held_keys.remove(&key);
        }
    }

    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn clear(&mut self) {
        self.held_keys.clear();
    }
}
