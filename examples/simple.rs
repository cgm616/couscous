#![no_std]
#![no_main]

punda::punda!(init: init);

fn init(cx: &mut punda::context::UserContext) {
    hprintln!("Hello from the BBC micro:bit");
}