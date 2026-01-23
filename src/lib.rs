
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};
use std::rc::Rc;
use std::cell::RefCell;

struct Blob {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: String,
    stiffness: f32,
    friction: f32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id("main_canvas")
        .expect("should have canvas")
        .dyn_into::<HtmlCanvasElement>()?;
    
    let context = canvas.get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
    let blobs = Rc::new(RefCell::new(Vec::<Blob>::new()));

    // 1. Mouse Move Listener
    {
        let mouse_pos = mouse_pos.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            *mouse_pos.borrow_mut() = (event.client_x() as f32, event.client_y() as f32);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // 1b. Touch Move Listener
    {
        let mouse_pos = mouse_pos.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default(); 
            if let Some(touch) = event.touches().get(0) {
                *mouse_pos.borrow_mut() = (touch.client_x() as f32, touch.client_y() as f32);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let window = web_sys::window().unwrap();
        let canvas = window.document().unwrap().get_element_by_id("main_canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        let time = window.performance().unwrap().now() as f32;
        
        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let mut blobs_mut = blobs.borrow_mut();
        let (mx, my) = *mouse_pos.borrow();
        
        let slider = window.document().unwrap().get_element_by_id("slider")
            .map(|e| e.dyn_into::<HtmlInputElement>().unwrap());
        let target_count = slider.map(|s| s.value().parse::<usize>().unwrap_or(10)).unwrap_or(10);

        // --- POPULATION LOGIC (FIXED) ---
        while blobs_mut.len() < target_count {
            let current_len = blobs_mut.len();
            let variety = (current_len as f32 * 0.07).sin(); 
            blobs_mut.push(Blob { 
                x: mx, y: my, vx: 0.0, vy: 0.0, 
                color: format!("hsl({}, 70%, 60%)", (current_len * 13) % 360),
                stiffness: 0.015 + (variety * 0.005),
                friction: 0.88 + (variety * 0.04),
            });
        }
        while blobs_mut.len() > target_count {
            blobs_mut.pop();
        }

        // --- DRAWING ---
        context.set_fill_style_str("rgba(0, 0, 0, 0.2)"); 
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        for blob in blobs_mut.iter_mut() {
            let dx = mx - blob.x;
            let dy = my - blob.y;

            // Unique jitter based on blob DNA
            let jitter_x = (time * 0.01 + (blob.stiffness * 100.0)).cos() * 0.5;
            let jitter_y = (time * 0.01 + (blob.friction * 100.0)).sin() * 0.5;

            // Update velocity using unique stiffness/friction
            blob.vx = (blob.vx + dx * blob.stiffness + jitter_x) * blob.friction; 
            blob.vy = (blob.vy + dy * blob.stiffness + jitter_y) * blob.friction;           

            blob.x += blob.vx;
            blob.y += blob.vy;

            let dist = (dx*dx + dy*dy).sqrt();
            let radius = (dist * 0.05).max(3.0);

            context.begin_path();
            context.set_fill_style_str(&blob.color);
            let _ = context.arc(blob.x as f64, blob.y as f64, radius as f64, 0.0, std::f64::consts::PI * 2.0);
            context.fill();
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

