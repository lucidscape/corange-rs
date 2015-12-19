use corange;
use corange::*;

#[derive(Clone, Debug)]
pub struct Renderer {
    pub configuration: String,
    pub color_correction_texture: Option<String>,
    pub vignetting_texture: Option<String>,
    pub texture_noise: f32,
    pub sea_enabled: bool,
    pub sky_enabled: bool,
    pub sky_time: f32,
    pub frame: u64
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            configuration: String::from("renderer.cfg"),
            color_correction_texture: None,
            vignetting_texture: None,
            texture_noise: 0.0,
            sea_enabled: false,
            sky_enabled: false,
            sky_time: 0.15,
            frame: 0
        }
    }
}

impl Renderer {
    pub fn initialize(self) -> *mut corange::renderer {
        unsafe {
            let renderer = renderer_new(asset_hndl_new_load(path(to_static_str(self.configuration.clone()))));
            self.apply(renderer);
            renderer
        }
    }

    pub fn apply(self, renderer:*mut corange::renderer) {
        unsafe {
            renderer_set_glitch(renderer, self.texture_noise);
            renderer_set_skydome_enabled(renderer, if self.sky_enabled { 1 } else { 0 });
            renderer_set_sea_enabled(renderer, if self.sea_enabled { 1 } else { 0 });
            renderer_set_tod(renderer, self.sky_time, 0);
            if let Some(texture) = self.color_correction_texture {
                renderer_set_color_correction(renderer, asset_hndl_new_load(path(to_static_str(texture))));
            }
            if let Some(texture) = self.vignetting_texture {
                renderer_set_vignetting(renderer, asset_hndl_new_load(path(to_static_str(texture))));
            }
        }
    }
}
