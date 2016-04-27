// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use glium::glutin::{Event, ElementState, MouseButton};

use super::Intermediate;
use super::super::{InputEvent, IntermediateEvent};

/// A `struct` that converts raw mouse inputs to cursor events.
pub struct Cursor {
    pub pos: Option<(i32, i32)>,
    pub pressed : HashMap<MouseButton, bool>
}

impl Cursor {
    /// Creates a `Cursor` without an initial position and without any pressed buttons.
    pub fn new() -> Cursor {
        Cursor {
            pos: None,
            pressed: HashMap::new()
        }
    }
}

impl<'a> Intermediate for &'a mut Cursor {
    fn process(self, input: Vec<InputEvent>) -> Vec<InputEvent> {
        let mut output = input.into_iter().filter_map(|event| {
            match event {
                InputEvent::Raw(Event::MouseMoved(x, y)) => {
                    self.pos = Some((x, y));

                    None
                },
                InputEvent::Raw(Event::MouseInput(ElementState::Pressed, button)) => {
                    self.pressed.insert(button, true);

                    None
                },
                InputEvent::Raw(Event::MouseInput(ElementState::Released, button)) => {
                    self.pressed.insert(button, false);

                    if let Some((x, y)) = self.pos {
                        Some(InputEvent::Intermediate(
                            IntermediateEvent::CursorReleased(x, y, button)
                        ))
                    } else {
                        None
                    }
                },
                event => Some(event)
            }
        }).collect::<Vec<_>>();

        if let Some((x, y)) = self.pos {
            for (button, pressed) in self.pressed.iter() {
                if *pressed {
                    output.push(InputEvent::Intermediate(
                        IntermediateEvent::CursorPressed(x, y, *button))
                    );
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use glium::glutin::{Event, ElementState, MouseButton};

    use super::Cursor;
    use super::super::Intermediate;
    use super::super::super::{InputEvent, IntermediateEvent};

    #[test]
    fn capture_move() {
        let events = vec![InputEvent::Raw(Event::MouseMoved(50, 50))];
        let mut cursor = Cursor::new();

        assert_eq!(cursor.pos, None);

        let events = cursor.process(events);

        assert_eq!(cursor.pos, Some((50, 50)));
        assert!(events.is_empty());
    }

    #[test]
    fn capture_click() {
        let events = vec![
            InputEvent::Raw(Event::MouseMoved(50, 50)),
            InputEvent::Raw(Event::MouseInput(ElementState::Pressed, MouseButton::Left))
        ];
        let mut cursor = Cursor::new();

        assert_eq!(cursor.pos, None);

        let events = cursor.process(events);

        assert_eq!(cursor.pos, Some((50, 50)));
        match events[0] {
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
            ) => assert!(true),
            _ => assert!(false)
        };

        let events = cursor.process(vec![]);
        
        match events[0] {
            InputEvent::Intermediate(
                IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
            ) => assert!(true),
            _ => assert!(false)
        };
    }

    #[test]
    fn capture_released() {
        let events = vec![
            InputEvent::Raw(Event::MouseMoved(50, 50)),
            InputEvent::Raw(Event::MouseInput(ElementState::Released, MouseButton::Left))
        ];
        let mut cursor = Cursor::new();

        assert_eq!(cursor.pos, None);

        let events = cursor.process(events);

        assert_eq!(cursor.pos, Some((50, 50)));
        match events[0] {
            InputEvent::Intermediate(
                IntermediateEvent::CursorReleased(50, 50, MouseButton::Left)
            ) => assert!(true),
            _ => assert!(false)
        };
    }
}
