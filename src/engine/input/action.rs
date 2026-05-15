use crate::engine::input::raw::RawInputState;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use winit::keyboard::KeyCode;

pub struct ActionMap<A: Eq + Hash + Copy> {
    bindings: HashMap<KeyCode, Vec<A>>,
}

impl<A: Eq + Hash + Copy> Default for ActionMap<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Eq + Hash + Copy> ActionMap<A> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: KeyCode, action: A) -> &mut Self {
        self.bindings.entry(key).or_default().push(action);
        self
    }

    pub fn resolve(&self, raw_input: &RawInputState) -> ActionState<A> {
        let mut active = HashSet::new();
        for (key, actions) in &self.bindings {
            if raw_input.is_key_held(*key) {
                for &a in actions {
                    active.insert(a);
                }
            }
        }

        ActionState { active }
    }
}

pub struct ActionState<A: Eq + Hash> {
    active: HashSet<A>,
}

impl<A: Eq + Hash> ActionState<A> {
    pub fn is_active(&self, action: A) -> bool {
        self.active.contains(&action)
    }
}
