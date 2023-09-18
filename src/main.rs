//! Simple text input support
//!
//! Return creates a new line, backspace removes the last character.
//! Clicking toggle IME (Input Method Editor) support, but the font used as limited support of characters.
//! You should change the provided font with another one to test other languages input.

use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*, text::{BreakLineOn, Text2dBounds}, a11y::accesskit::Size};

#[derive(Component, Default)]
struct TargetText;

#[derive(Component, Default)]
struct InitialChars(Vec<char>);

#[derive(Component, Default)]
struct CursorIndex(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                listen_received_character_events,
                listen_received_keyboard_events,
            ),
        )
        .run();
}

#[derive(Bundle, Default)]
struct Target {
    target_text: TargetText,
    text2d: Text2dBundle,
    initial_chars: InitialChars,
    cursor_index: CursorIndex,
}


fn target_text(target: &str, font: Handle<Font>) -> Target {
    let sections = target.chars().map(
        |c| TextSection {
            value: c.to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        }
    ).collect::<Vec<_>>();

    Target {
        text2d: Text2dBundle {
            text: Text { 
                sections,
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::WordBoundary,
            },
            text_2d_bounds: Text2dBounds {
                size: Vec2{x: 1000.0, y: 120.0}
            },
            ..Default::default()
        },
        initial_chars: InitialChars(target.chars().collect::<Vec<_>>()),
        ..Default::default()
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/JetBrains Mono NL Regular Nerd Font Complete Windows Compatible.ttf");

    commands.spawn(target_text(&"0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 ", font));
}

fn listen_received_keyboard_events(
    mut events: EventReader<KeyboardInput>,
    mut text: Query<(&mut Text, &mut CursorIndex, &mut InitialChars), With<TargetText>>
){
    let (mut text, mut cursor, initial_chars) = text.single_mut();
    for event in events.iter() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.key_code {
            Some(KeyCode::Back) => {
                if cursor.0 > 0{
                    cursor.0 -= 1;
                    text.sections[cursor.0].value = initial_chars.0[cursor.0].to_string();
                    text.sections[cursor.0].style.color = Color::WHITE;
                }
            },
            _ => {}
        }
    }
}

fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut text: Query<(&mut Text, &mut CursorIndex), With<TargetText>>,
) {
    let (mut text, mut cursor) = text.single_mut();
    for event in events.into_iter() {
        if cursor.0 >= text.sections.len() {
            continue;
        }

        if text.sections[cursor.0].value.eq(&event.char.to_string())
        {
            text.sections[cursor.0].style.color = Color::DARK_GREEN;
            cursor.0 += 1;
        } else if event.char.is_ascii_alphanumeric() {
            text.sections[cursor.0].value = event.char.to_string();
            text.sections[cursor.0].style.color = Color::RED;
            cursor.0 += 1;
        }
    }
}