extern crate anima_engine;

use std::time::Duration;

use self::anima_engine::glium::glutin::{Event, ElementState, MouseButton};

use self::anima_engine::input::{InputEvent, Intermediate, IntermediateEvent, SelectableArea,
                                SpecialSelect};

#[test]
fn click_outside() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(10, 50, MouseButton::Left)
        )
    ];
    let mut area = SelectableArea::new(3, 40, 40, 20, 20, None);

    let events = area.process(events, Duration::new(0, 0));

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
    let mut area = SelectableArea::new(3, 40, 40, 20, 20, None);

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::SelectablePressed(3, 50, 50)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::SelectablePressed(3, 50, 50)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorReleased(50, 50, MouseButton::Left)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::SelectableReleased(3, 50, 50)
        ) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn click_dragged() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 50, MouseButton::Left)
        )
    ];
    let mut area = SelectableArea::new(3, 40, 40, 20, 20, None);

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::SelectablePressed(3, 50, 50)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 51, MouseButton::Left)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(IntermediateEvent::SelectableDragged(3, 50, 51)) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorReleased(50, 51, MouseButton::Left)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::SelectableReleased(3, 50, 51)
        ) => assert!(true),
        _ => assert!(false)
    };
}

#[test]
fn click_special_dragged() {
    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 50, MouseButton::Right)
        )
    ];
    let mut area = SelectableArea::new(3, 40, 40, 20, 20,
                                       Some(SpecialSelect { button: MouseButton::Right,
                                                            touch_time: Duration::new(0, 0)}));

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::SelectableSpecialPressed(3, 50, 50)
        ) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorPressed(50, 51, MouseButton::Right)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::SelectableSpecialDragged(3, 50, 51)
        ) => assert!(true),
        _ => assert!(false)
    };

    let events = vec![
        InputEvent::Intermediate(
            IntermediateEvent::CursorReleased(50, 51, MouseButton::Right)
        )
    ];

    let events = area.process(events, Duration::new(0, 0));

    match events[0] {
        InputEvent::Intermediate(
            IntermediateEvent::SelectableSpecialReleased(3, 50, 51)
        ) => assert!(true),
        _ => assert!(false)
    };
}
//
