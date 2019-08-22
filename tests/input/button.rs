extern crate anima_engine;

use std::time::Duration;

use self::anima_engine::glium::glutin::{Event, MouseButton, Touch, TouchPhase};

use self::anima_engine::input::{Button, InputEvent, Intermediate, IntermediateEvent};

#[test]
fn click_outside() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(10, 50, MouseButton::Left)
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(10, 50, MouseButton::Left)
        ) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn click_inside() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorReleased(50, 50, MouseButton::Left)
        )
    ];

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonReleased(3)) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn click_canceled() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorReleased(10, 50, MouseButton::Left)
        )
    ];

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonCanceled(3)) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn touch_outside() {
    let events = vec![
        InputEvent::Raw(
            Event::Touch(Touch {
                phase: TouchPhase::Started,
                location: (10.0, 50.0),
                id: 0
            })
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Raw(Event::Touch(_)) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn touch_inside() {
    let events = vec![
        InputEvent::Raw(
            Event::Touch(Touch {
                phase: TouchPhase::Started,
                location: (50.0, 50.0),
                id: 0
            })
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Raw(
            Event::Touch(Touch {
                phase: TouchPhase::Ended,
                location: (50.0, 50.0),
                id: 0
            })
        )
    ];

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonReleased(3)) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn touch_canceled() {
    let events = vec![
        InputEvent::Raw(
            Event::Touch(Touch {
                phase: TouchPhase::Started,
                location: (50.0, 50.0),
                id: 0
            })
        )
    ];
    let mut button = Button::new(3, 40, 40, 20, 20);

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonPressed(3)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Raw(
            Event::Touch(Touch {
                phase: TouchPhase::Ended,
                location: (10.0, 50.0),
                id: 0
            })
        )
    ];

    let events = button.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::ButtonCanceled(3)) => assert!(true),
        _ => assert!(false)
    };
}
