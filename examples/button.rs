#![no_std]
#![no_main]

use punda::{
    button::{Button, State},
    serial,
};

punda::punda!(init = init, button_handler = button_handler);

fn init(cx: &mut punda::context::InitContext) {
    serial::println(cx, "Press a button".into());
}

fn button_handler(cx: &mut punda::context::HandlerContext, button: Button, direction: State) {
    let message = match (button, direction) {
        (Button::A, State::Released) => "A released",
        (Button::B, State::Released) => "B released",
        (Button::Both, State::Released) => "both released",
        (Button::A, State::Pushed) => "A pushed",
        (Button::B, State::Pushed) => "B pushed",
        (Button::Both, State::Pushed) => "both pushed",
    };

    serial::println(cx, message.into());
}