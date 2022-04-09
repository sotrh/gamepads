use std::{cell::RefCell, rc::Rc};
use gilrs::Event;
use log::info;
use wasm_bindgen::{prelude::*, JsCast};

fn window() -> web_sys::Window {
    web_sys::window().expect("Platform does not have a global window.")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` should succeed");
}

fn game_loop(mut callback: impl FnMut() + 'static) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("Couldn't setup console logging");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        callback();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    info!("Starting game loop");
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = gilrs::Gilrs::new()?;

    game_loop(move || {
        info!("game_loop");
        while let Some(Event { id, event, time}) = context.next_event() {
            info!("Gamepad event");
            // info!("{:?}", id);
        }
    });

    Ok(())
}
