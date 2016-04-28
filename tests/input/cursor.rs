extern crate anima_engine;

use self::anima_engine::glium::glutin::{Event, ElementState, MouseButton};

use self::anima_engine::input::{Cursor, InputEvent, Intermediate, IntermediateEvent};

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
