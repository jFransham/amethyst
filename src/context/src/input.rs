//! The input handler for the game engine

use std::collections::hash_map::{Entry, HashMap};
use event::{ElementState, EngineEvent, Event, VirtualKeyCode};

#[derive(PartialEq, Eq)]
enum KeyQueryState{
    NotQueried,
    Queried,
}

pub struct InputHandler {
    pressed_keys: HashMap<VirtualKeyCode, KeyQueryState>,
}

impl InputHandler {
    /// Create a new InputHandler
    pub fn new() -> InputHandler {
        InputHandler {
            pressed_keys: HashMap::new(),
        }
    }

    /// Update the input handler with new engine events
    pub fn update(&mut self, events: &[EngineEvent]) {
        for event in events {
            match event.payload {
                Event::KeyboardInput(ElementState::Pressed, _, Some(key_code)) => {
                    match self.pressed_keys.entry(key_code) {
                        Entry::Occupied(_) => {
                            // nop
                            // Allows more accurate `key_once` calls,
                            // I.e `key_once(key)` is queried after 
                            // second `Pressed` event.
                        },
                        Entry::Vacant(entry) => {
                            entry.insert(KeyQueryState::Queried);
                        }
                    }
                },
                Event::KeyboardInput(ElementState::Released, _, Some(key_code)) => {
                    self.pressed_keys.remove(&key_code);
                },
                Event::Focused(false) => self.pressed_keys.clear(),
                _ => {}
            }
        }
    }

    /// Check if `key` is currently pressed
    pub fn key_down(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains_key(&key)
    }
    
    /// Check if all `keys` are currently pressed
    pub fn keys_down(&self, keys: &[VirtualKeyCode]) -> bool {
        keys.iter().all(|key| self.key_down(*key))
    }
    
    /// Check if `key` is currently pressed and `key_once` or `keys_once` hasn't been
    /// called since this `key` was first pressed.
    pub fn key_once(&mut self, key: VirtualKeyCode) -> bool {
        if !self.pressed_keys.contains_key(&key) {
            return false;
        }
        if let Some(value) = self.pressed_keys.get_mut(&key) { // Should be safe
            if *value == KeyQueryState::NotQueried {
                *value = KeyQueryState::Queried;
                return true;
            }
        }
        return false;
    }

    /// Checks if `keys` are all currently pressed and haven't been called with `key_once` or
    /// `keys_once`
    pub fn keys_once(&mut self, keys: &[VirtualKeyCode]) -> bool {
        keys.iter().any(|key| self.key_once(*key)) && self.keys_down(keys)
    }
}
