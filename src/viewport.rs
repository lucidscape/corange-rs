use corange::*;

#[derive(Clone, Debug)]
pub struct Viewport {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub fullscreen: bool,
    pub antialiasing: usize,
    pub multisampling: usize,
    pub frame: u64
}

impl Default for Viewport {
    fn default() -> Viewport {
        Viewport {
            title: String::from("Output"),
            width: 1280,
            height: 720,
            fullscreen: false,
            antialiasing: 0,
            multisampling: 0,
            frame:0
        }
    }
}

impl Viewport {
    pub fn apply(&self) {
        unsafe {
            graphics_viewport_set_title(str(to_static_str(self.title.clone())));
            graphics_viewport_set_size(self.width as i32, self.height as i32);
            graphics_set_antialiasing(self.antialiasing as i32);
            graphics_set_multisamples(self.multisampling as i32);
            graphics_set_fullscreen(if self.fullscreen { 1 } else { 0 });
        }
    }
}
