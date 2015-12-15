extern crate libc;
extern crate sdl2_sys;
#[macro_use] extern crate corange_rs;
#[macro_use] extern crate lazy_static;

use std::mem;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use libc::c_void;
use sdl2_sys::event::*;
use corange_rs::*;

fn main() {
    unsafe {
        // Initialize engine and load default assets/shaders
        corange_init(str("./src/corange/assets_core/"));

        // Initialize viewport
        graphics_viewport_set_title(str("Rendering"));
        graphics_viewport_set_size(1280, 720);
        let height = graphics_viewport_height() as f32;

        // Acquire type ids
        let camera_type = typeid!(camera);
        let static_type = typeid!(static_object);
        let animated_type = typeid!(animated_object);
        let button_type = typeid!(ui_button);

        // Initialize camera
        let cam = entity_new_type_id(str("camera"), camera_type) as *mut camera;
        (*cam).position = vec3_new(25.0, 25.0, 10.0);
        (*cam).target = vec3_new(0.0, 7.0, 0.0);

        // Initialize podium
        folder_load(path("./examples/assets/podium/"));
        let s_podium = entity_new_type_id(str("podium"), static_type) as *mut static_object;
        (*s_podium).renderable = asset_hndl_new(path("./examples/assets/podium/podium.bmf"));

        // Initialize cello
        folder_load(path("./examples/assets/cello/"));
        let s_cello = entity_new_type_id(str("cello"), static_type) as *mut static_object;
        (*s_cello).renderable = asset_hndl_new(path("./examples/assets/cello/cello.bmf"));
        (*s_cello).position = vec3_new(0.0, 3.0, 0.0);
        (*s_cello).rotation = quat_rotation_x(-1.7);
        (*s_cello).scale = vec3_new(0.75, 0.75, 0.75);

        // Initialize piano
        folder_load(path("./examples/assets/piano/"));
        let s_piano = entity_new_type_id(str("piano"), static_type) as *mut static_object;
        (*s_piano).renderable = asset_hndl_new(path("./examples/assets/piano/piano.bmf"));
        (*s_piano).position = vec3_new(1.0, 5.0, 0.0);

        // Initialize dino
        folder_load(path("./examples/assets/dino/"));
        let s_dino = entity_new_type_id(str("dino"), static_type) as *mut static_object;
        (*s_dino).renderable = asset_hndl_new(path("./examples/assets/dino/dino.bmf"));
        (*s_dino).scale = vec3_new(4.0, 4.0, 4.0);

        // Initialize imrod
        folder_load(path("./examples/assets/imrod/"));
        let a_imrod = entity_new_type_id(str("imrod"), animated_type) as *mut animated_object;
        animated_object_load_skeleton(a_imrod, asset_hndl_new(path("./examples/assets/imrod/imrod.skl")));
        (*a_imrod).renderable = asset_hndl_new(path("./examples/assets/imrod/imrod.bmf"));
        (*a_imrod).animation = asset_hndl_new(path("./examples/assets/imrod/imrod.ani"));
        (*a_imrod).rotation = quat_rotation_y(1.57);
        (*a_imrod).scale = vec3_new(1.25, 1.25, 1.25);

        // Initialize FPS counter
        let framerate = ui_elem_new_type_id(str("framerate"), button_type) as *mut ui_button;
        ui_button_move(framerate, vec2_new(10.0, 10.0));
        ui_button_resize(framerate, vec2_new(30.0, 25.0));
        ui_button_set_label(framerate, str("FRAMERATE"));
        ui_button_disable(framerate);

        // Initialize object label
        let object = ui_elem_new_type_id(str("object"), button_type) as *mut ui_button;
        ui_button_move(object, vec2_new(10.0, height - 70.0));
        ui_button_resize(object, vec2_new(60.0, 25.0));
        ui_button_set_label(object, str("Object"));
        ui_button_disable(object);

        // Initialize piano button
        let b_piano = ui_elem_new_type_id(str("piano"), button_type) as *mut ui_button;
        ui_button_move(b_piano, vec2_new(80.0, height - 70.0));
        ui_button_resize(b_piano, vec2_new(50.0, 25.0));
        ui_button_set_label(b_piano, str("Piano"));
        ui_button_set_onclick(b_piano, Some(select_piano));

        // Initialize cello button
        let b_cello = ui_elem_new_type_id(str("cello"), button_type) as *mut ui_button;
        ui_button_move(b_cello, vec2_new(140.0, height - 70.0));
        ui_button_resize(b_cello, vec2_new(50.0, 25.0));
        ui_button_set_label(b_cello, str("Cello"));
        ui_button_set_onclick(b_cello, Some(select_cello));

        // Initialize dino button
        let b_dino = ui_elem_new_type_id(str("dino"), button_type) as *mut ui_button;
        ui_button_move(b_dino, vec2_new(260.0, height - 70.0));
        ui_button_resize(b_dino, vec2_new(40.0, 25.0));
        ui_button_set_label(b_dino, str("Dino"));
        ui_button_set_onclick(b_dino, Some(select_dino));

        // Initialize imrod button
        let b_imrod = ui_elem_new_type_id(str("imrod"), button_type) as *mut ui_button;
        ui_button_move(b_imrod, vec2_new(200.0, height - 70.0));
        ui_button_resize(b_imrod, vec2_new(50.0, 25.0));
        ui_button_set_label(b_imrod, str("Imrod"));
        ui_button_set_onclick(b_imrod, Some(select_imrod));

        // Initialize renderer
        let renderer = renderer_new(asset_hndl_new_load(path("./examples/assets/graphics.cfg")));
        renderer_set_camera(renderer, cam);
        renderer_set_tod(renderer, 0.15, 0);
        renderer_set_skydome_enabled(renderer, 0);

        loop {
            // Initialize frame
            frame_begin();

            // Handle SDL events
            loop {
                let mut raw = mem::uninitialized();
                match SDL_PollEvent(&mut raw) == 1 {
                    true => {
                        // Foward events to camera controller
                        let event:SDL_Event = mem::transmute_copy(&raw);
                        camera_control_orbit(cam, event);

                        // Foward events to UI controller
                        let event:SDL_Event = mem::transmute_copy(&raw);
                        ui_event(event);
                    }
                    false => break
                }
            }

            // Update camera orbit
            camera_control_joyorbit(cam, frame_time() as f32);

            // Update UI
            ui_button_set_label(framerate, frame_rate_string());
            ui_update();

            // Update animated objects
            animated_object_update(a_imrod, frame_time() as f32 * 0.25);

            // Add objects to scene
            renderer_add(renderer, render_object_static(s_podium));
            match *SELECTED.lock().unwrap() {
                0 => renderer_add(renderer, render_object_static(s_piano)),
                1 => renderer_add(renderer, render_object_static(s_cello)),
                2 => renderer_add(renderer, render_object_static(s_dino)),
                3 => renderer_add(renderer, render_object_animated(a_imrod)),
                _ => ()
            }

            // Render scene
            renderer_render(renderer);

            // Render UI
            ui_render();

            // Swap buffers and end frame
            graphics_swap();
            frame_end();
        }
    }
}

lazy_static! {
    static ref SELECTED:Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

fn select(item:usize) {
    let mut selected = SELECTED.lock().unwrap();
    *selected = item;
}

unsafe extern fn select_piano(_:*mut ui_button, _:*mut libc::c_void) {
    select(0);
}

unsafe extern fn select_cello(_:*mut ui_button, _:*mut libc::c_void) {
    select(1);
}

unsafe extern fn select_dino(_:*mut ui_button, _:*mut libc::c_void) {
    select(2);
}

unsafe extern fn select_imrod(_:*mut ui_button, _:*mut libc::c_void) {
    select(3);
}
