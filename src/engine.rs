use std::mem;
use std::sync::{Arc, Mutex};
use sdl2_sys::event::*;
use sdl2_sys::keycode::*;
use corange::*;
use viewport::Viewport;
use renderer::Renderer;
use camera::{Camera, CameraType};

lazy_static! {
    pub static ref FRAME:Arc<Mutex<u64>> = Arc::new(Mutex::new(0u64));
    pub static ref VIEWPORT:Arc<Mutex<Viewport>> = Arc::new(Mutex::new(Viewport::default()));
    pub static ref RENDERER:Arc<Mutex<Renderer>> = Arc::new(Mutex::new(Renderer::default()));
    pub static ref CAMERA:Arc<Mutex<Camera>> = Arc::new(Mutex::new(Camera::default()));
}

/// Initialize the Corange engine and load default assets/shaders
pub fn initialize(assets_path:&'static str) {
    unsafe { corange_init(str(assets_path)) }
}

fn frame() -> u64 {
    *FRAME.lock().unwrap()
}

/// Reconfigure viewport
pub fn set_viewport(title:String, width:usize, height:usize, fullscreen:bool, antialiasing:usize, multisampling:usize) {
    *VIEWPORT.lock().unwrap() = Viewport {
        title: title,
        width: width,
        height: height,
        fullscreen: fullscreen,
        antialiasing: antialiasing,
        multisampling: multisampling,
        frame: frame() + 1
    };
}

/// Reconfigure renderer
pub fn set_renderer(configuration: String, color_correction_texture: Option<String>, vignetting_texture: Option<String>, texture_noise: f32, sea_enabled: bool, sky_enabled: bool, sky_time: f32) {
    *RENDERER.lock().unwrap() = Renderer {
        configuration: configuration,
        color_correction_texture: color_correction_texture,
        vignetting_texture: vignetting_texture,
        texture_noise: texture_noise,
        sea_enabled: sea_enabled,
        sky_enabled: sky_enabled,
        sky_time: sky_time,
        frame: frame() + 1
    };
}

/// Reconfigure camera
pub fn set_camera(position: vec3, target: vec3, fov: f32, near_clip: f32, far_clip: f32, movement: CameraType) {
    *CAMERA.lock().unwrap() = Camera {
        position: position,
        target: target,
        fov: fov,
        near_clip: near_clip,
        far_clip: far_clip,
        movement: movement,
        frame: frame() + 1
    };
}

/// Assert Corange viewport settings matches user-submitted settings
fn update_viewport() {
    let configuration = VIEWPORT.lock().unwrap().clone();
    if configuration.frame == frame() {
        configuration.apply();
    }
}

/// Assert Corange renderer settings matches user-submitted settings
fn update_renderer(renderer:*mut renderer) {
    let configuration = RENDERER.lock().unwrap().clone();
    if configuration.frame == frame() {
        configuration.apply(renderer);
    }
}

// Assert Corange renderer settings matches user-submitted settings
fn update_camera(camera:*mut camera) -> CameraType {
    let configuration = CAMERA.lock().unwrap().clone();
    let camera_type = configuration.movement.clone();
    if configuration.frame == frame() {
        configuration.apply(camera);
    }
    camera_type
}

/// Enter main rendering loop
pub fn run(event_handler:Option<&Fn(SDL_Event)>, update_handler:Option<&Fn(f64, *mut renderer)>) {
    unsafe {

        // Initialize viewport
        VIEWPORT.lock().unwrap().clone().apply();

        // Initialize renderer
        let renderer = RENDERER.lock().unwrap().clone().initialize();

        // Initialize camera
        let camera = entity_new_type_id(str("camera"), *CAMERA_TYPE) as *mut camera;
        update_camera(camera);

        'main: loop {
            // Synchronize engine component settings
            update_viewport();
            update_renderer(renderer);
            let camera_type = update_camera(camera);

            // Initialize frame
            frame_begin();

            // Handle SDL events
            loop {
                let mut raw = mem::uninitialized();
                match SDL_PollEvent(&mut raw) == 1 {
                    true => {
                        let mut event:SDL_Event = mem::transmute_copy(&raw);
                        match *event.type_() {
                            SDL_QUIT => break 'main,
                            SDL_KEYUP => {
                                match (*event.key()).keysym.sym {
                                    SDLK_ESCAPE => break 'main,
                                    _ => ()
                                }
                            }
                            _ => ()
                        }

                        // Update camera
                        let event:SDL_Event = mem::transmute_copy(&raw);
                        match camera_type {
                            CameraType::Orbit => camera_control_orbit(camera, event),
                            CameraType::Free => camera_control_freecam(camera, frame_time() as f32),
                            CameraType::Manual => ()
                        }

                        // Foward events to UI controller
                        let event:SDL_Event = mem::transmute_copy(&raw);
                        ui_event(event);

                        // Forward events to user-defined handler
                        if let Some(handler) = event_handler {
                            let event:SDL_Event = mem::transmute_copy(&raw);
                            handler(event);
                        }
                    }
                    false => break
                }
            }

            // Call user-defined frame update handler
            if let Some(handler) = update_handler {
                handler(frame_time(), renderer);
            }

            // Render scene
            renderer_set_camera(renderer, camera);
            renderer_render(renderer);

            // Render UI
            ui_update();
            ui_render();

            // Swap buffers and end frame
            graphics_swap();
            frame_end();

            // Increment frame count
            *FRAME.lock().unwrap() += 1;
        }
    }
}
