// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::time::Duration;

use glium::glutin::MouseButton;

use super::super::Intermediate;
use super::super::super::{InputEvent, IntermediateEvent};

pub struct SelectableArea {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    special: Option<SpecialSelect>,
    pressed: Option<(i32, i32)>,
    special_pressed: Option<(i32, i32)>
}

impl SelectableArea {
    pub fn new(id: u32, x: i32, y: i32, width: i32, height: i32,
               special: Option<SpecialSelect>) -> SelectableArea {
        SelectableArea {
            id: id,
            x: x,
            y: y,
            width: width,
            height: height,
            special: special,
            pressed: None,
            special_pressed: None
        }
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;

        0 <= dx && dx <= self.width &&
        0 <= dy && dy <= self.height
    }
}

impl<'a> Intermediate for &'a mut SelectableArea {
    fn process(self, input: Vec<InputEvent>, dt: Duration) -> Vec<InputEvent> {
        input.into_iter().filter_map(|event| {
            match event {
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(x, y, MouseButton::Left)
                ) if self.pressed.is_none() && self.inside(x, y) => {
                    self.pressed = Some((x, y));

                    Some(InputEvent::Intermediate(
                        IntermediateEvent::SelectablePressed(self.id, x, y)
                    ))
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(x, y, MouseButton::Left)
                ) if self.inside(x, y) => {
                    let old = self.pressed.unwrap();

                    if old == (x, y) {
                        Some(InputEvent::Intermediate(
                            IntermediateEvent::SelectablePressed(self.id, x, y)
                        ))
                    } else {
                        Some(InputEvent::Intermediate(
                            IntermediateEvent::SelectableDragged(self.id, x, y)
                        ))
                    }
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorReleased(x, y, MouseButton::Left)
                ) if self.pressed.is_some() && self.inside(x, y) => {
                    self.pressed = None;

                    Some(InputEvent::Intermediate(
                        IntermediateEvent::SelectableReleased(self.id, x, y)
                    ))
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(x, y, button)
                ) if self.special_pressed.is_none() && self.inside(x, y) &&
                     self.special.is_some() && self.special.unwrap().button == button => {

                    self.special_pressed = Some((x, y));

                    Some(InputEvent::Intermediate(
                        IntermediateEvent::SelectableSpecialPressed(self.id, x, y)
                    ))
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorPressed(x, y, button)
                ) if self.inside(x, y) &&
                     self.special.is_some() && self.special.unwrap().button == button => {

                    let old = self.special_pressed.unwrap();

                    if old == (x, y) {
                        Some(InputEvent::Intermediate(
                            IntermediateEvent::SelectableSpecialPressed(self.id, x, y)
                        ))
                    } else {
                        Some(InputEvent::Intermediate(
                            IntermediateEvent::SelectableSpecialDragged(self.id, x, y)
                        ))
                    }
                },
                InputEvent::Intermediate(
                    IntermediateEvent::CursorReleased(x, y, button)
                ) if self.special_pressed.is_some() && self.inside(x, y) &&
                     self.special.is_some() && self.special.unwrap().button == button => {

                    self.special_pressed = None;

                    Some(InputEvent::Intermediate(
                        IntermediateEvent::SelectableSpecialReleased(self.id, x, y)
                    ))
                },
                event => Some(event)
            }
        }).collect()
    }
}

#[derive(Clone, Copy)]
pub struct SpecialSelect {
    pub button: MouseButton,
    pub touch_time: Duration
}
