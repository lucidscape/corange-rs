#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::mem;
use std::ffi::CString;
use libc::{c_void, c_char, c_int, c_uint, c_ulong, c_float, uint32_t, c_uchar, c_double, c_long, size_t};
use gl::types::{GLint, GLuint, GLenum};
use sdl2_sys::event::SDL_Event;
use sdl2_sys::video::SDL_GLContext;
use sdl2_sys::joystick::SDL_Joystick;

#[macro_export]
macro_rules! typeId {
    ($field_ty:ty) => {
        type_find(CString::new(stringify!($field_ty)).unwrap().as_ptr(), mem::size_of::<$field_ty>())
    };
}

lazy_static! {
    // Type IDs
    pub static ref CAMERA_TYPE:i32 = unsafe { typeId!(camera) };
    pub static ref LIGHT_TYPE:i32 = unsafe { typeId!(light) };
    pub static ref STATIC_TYPE:i32 = unsafe { typeId!(static_object) };
    pub static ref INSTANCE_TYPE:i32 = unsafe { typeId!(instance_object) };
    pub static ref ANIMATED_TYPE:i32 = unsafe { typeId!(animated_object) };
    pub static ref PHYSICS_TYPE:i32 = unsafe { typeId!(physics_object) };
    pub static ref PARTICLES_TYPE:i32 = unsafe { typeId!(particles) };
    pub static ref LANDSCAPE_TYPE:i32 = unsafe { typeId!(landscape) };
    pub static ref BROWNSER_TYPE:i32 = unsafe { typeId!(ui_browser) };
    pub static ref BUTTON_TYPE:i32 = unsafe { typeId!(ui_button) };
    pub static ref DIALOG_TYPE:i32 = unsafe { typeId!(ui_dialog) };
    pub static ref LISTBOX_TYPE:i32 = unsafe { typeId!(ui_listbox) };
    pub static ref OPTION_TYPE:i32 = unsafe { typeId!(ui_option) };
    pub static ref RECTANGLE_TYPE:i32 = unsafe { typeId!(ui_rectangle) };
    pub static ref SLIDER_TYPE:i32 = unsafe { typeId!(ui_slider) };
    pub static ref SPINNER_TYPE:i32 = unsafe { typeId!(ui_spinner) };
    pub static ref STYLE_TYPE:i32 = unsafe { typeId!(ui_style) };
    pub static ref TEXT_TYPE:i32 = unsafe { typeId!(ui_text) };
    pub static ref TEXTBOX_TYPE:i32 = unsafe { typeId!(ui_textbox) };
    pub static ref TOAST_TYPE:i32 = unsafe { typeId!(ui_toast) };
}

pub const LIGHT_TYPE_POINT: c_uint = 0;
pub const LIGHT_TYPE_DIRECTIONAL: c_uint = 1;
pub const LIGHT_TYPE_SUN: c_uint = 2;
pub const LIGHT_TYPE_SPOT: c_uint = 3;
pub const HTTP_ERR_NONE: c_uint = 0;
pub const HTTP_ERR_URL: c_uint = 1;
pub const HTTP_ERR_HOST: c_uint = 2;
pub const HTTP_ERR_SOCKET: c_uint = 3;
pub const HTTP_ERR_DATA: c_uint = 4;
pub const HTTP_ERR_NOFILE: c_uint = 5;
pub const IMAGE_REPEAT_TILE: c_uint = 0;
pub const IMAGE_REPEAT_CLAMP: c_uint = 1;
pub const IMAGE_REPEAT_MIRROR: c_uint = 2;
pub const IMAGE_REPEAT_ERROR: c_uint = 3;
pub const IMAGE_REPEAT_BLACK: c_uint = 4;
pub const IMAGE_SAMPLE_LINEAR: c_uint = 0;
pub const IMAGE_SAMPLE_NEAREST: c_uint = 1;
pub const TEXT_ALIGN_LEFT: c_uint = 0;
pub const TEXT_ALIGN_RIGHT: c_uint = 1;
pub const TEXT_ALIGN_CENTER: c_uint = 2;
pub const TEXT_ALIGN_TOP: c_uint = 0;
pub const TEXT_ALIGN_BOTTOM: c_uint = 1;
pub const RENDERER_MAX_LIGHTS: c_uint = 16;
pub const RENDERER_MAX_DYN_LIGHTS: c_uint = 13;
pub const RO_TYPE_AXIS: c_uint = 0;
pub const RO_TYPE_STATIC: c_uint = 1;
pub const RO_TYPE_INSTANCE: c_uint = 2;
pub const RO_TYPE_ANIMATED: c_uint = 3;
pub const RO_TYPE_PARTICLES: c_uint = 4;
pub const RO_TYPE_LIGHT: c_uint = 5;
pub const RO_TYPE_LANDSCAPE: c_uint = 6;
pub const RO_TYPE_PAINT: c_uint = 7;
pub const RO_TYPE_SPHERE: c_uint = 8;
pub const RO_TYPE_ELLIPSOID: c_uint = 9;
pub const RO_TYPE_CMESH: c_uint = 10;
pub const RO_TYPE_FRUSTUM: c_uint = 11;
pub const RO_TYPE_PLANE: c_uint = 12;
pub const RO_TYPE_LINE: c_uint = 13;
pub const RO_TYPE_POINT: c_uint = 14;

pub fn to_static_str(input: String) -> &'static str {
    unsafe {
        let input = input.clone();
        let output = mem::transmute(&input as &str);
        mem::forget(input);
        output
    }
}

pub fn str(input:&'static str) -> *mut i8 {
    CString::new(input).unwrap().into_raw()
}

pub fn path(input:&'static str) -> fpath {
    unsafe {
        P(str(input))
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct fpath {
    pub ptr: [c_char; 4096usize],
}
impl Clone for fpath {
    fn clone(&self) -> Self { *self }
}
impl Default for fpath {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct timer {
    pub id: c_int,
    pub start: c_ulong,
    pub end: c_ulong,
    pub split: c_ulong,
}
impl Clone for timer {
    fn clone(&self) -> Self { *self }
}
impl Default for timer {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type type_id = c_int;

#[repr(C)]
#[derive(Copy)]
pub struct vec2 {
    pub x: c_float,
    pub y: c_float,
}
impl Clone for vec2 {
    fn clone(&self) -> Self { *self }
}
impl Default for vec2 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vec3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}
impl Clone for vec3 {
    fn clone(&self) -> Self { *self }
}
impl Default for vec3 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}
impl Clone for vec4 {
    fn clone(&self) -> Self { *self }
}
impl Default for vec4 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}
pub type quat = vec4;

#[repr(C)]
#[derive(Copy)]
pub struct quat_dual {
    pub real: quat,
    pub dual: quat,
}
impl Clone for quat_dual {
    fn clone(&self) -> Self { *self }
}
impl Default for quat_dual {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct mat2 {
    pub xx: c_float,
    pub xy: c_float,
    pub yx: c_float,
    pub yy: c_float,
}
impl Clone for mat2 {
    fn clone(&self) -> Self { *self }
}
impl Default for mat2 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct mat3 {
    pub xx: c_float,
    pub xy: c_float,
    pub xz: c_float,
    pub yx: c_float,
    pub yy: c_float,
    pub yz: c_float,
    pub zx: c_float,
    pub zy: c_float,
    pub zz: c_float,
}
impl Clone for mat3 {
    fn clone(&self) -> Self { *self }
}
impl Default for mat3 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct mat4 {
    pub xx: c_float,
    pub xy: c_float,
    pub xz: c_float,
    pub xw: c_float,
    pub yx: c_float,
    pub yy: c_float,
    pub yz: c_float,
    pub yw: c_float,
    pub zx: c_float,
    pub zy: c_float,
    pub zz: c_float,
    pub zw: c_float,
    pub wx: c_float,
    pub wy: c_float,
    pub wz: c_float,
    pub ww: c_float,
}
impl Clone for mat4 {
    fn clone(&self) -> Self { *self }
}
impl Default for mat4 {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct plane {
    pub direction: vec3,
    pub position: vec3,
}
impl Clone for plane {
    fn clone(&self) -> Self { *self }
}
impl Default for plane {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct _box {
    pub top: plane,
    pub bottom: plane,
    pub left: plane,
    pub right: plane,
    pub front: plane,
    pub back: plane,
}
impl Clone for _box {
    fn clone(&self) -> Self { *self }
}
impl Default for _box {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct frustum {
    pub ntr: vec3,
    pub ntl: vec3,
    pub nbr: vec3,
    pub nbl: vec3,
    pub ftr: vec3,
    pub ftl: vec3,
    pub fbr: vec3,
    pub fbl: vec3,
}
impl Clone for frustum {
    fn clone(&self) -> Self { *self }
}
impl Default for frustum {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct sphere {
    pub center: vec3,
    pub radius: c_float,
}
impl Clone for sphere {
    fn clone(&self) -> Self { *self }
}
impl Default for sphere {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ellipsoid {
    pub center: vec3,
    pub radiuses: vec3,
}
impl Clone for ellipsoid {
    fn clone(&self) -> Self { *self }
}
impl Default for ellipsoid {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct capsule {
    pub start: vec3,
    pub end: vec3,
    pub radius: c_float,
}
impl Clone for capsule {
    fn clone(&self) -> Self { *self }
}
impl Default for capsule {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vertex {
    pub position: vec3,
    pub normal: vec3,
    pub tangent: vec3,
    pub binormal: vec3,
    pub color: vec4,
    pub uvs: vec2,
}
impl Clone for vertex {
    fn clone(&self) -> Self { *self }
}
impl Default for vertex {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct mesh {
    pub num_verts: c_int,
    pub num_triangles: c_int,
    pub verticies: *mut vertex,
    pub triangles: *mut uint32_t,
}
impl Clone for mesh {
    fn clone(&self) -> Self { *self }
}
impl Default for mesh {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct model {
    pub num_meshes: c_int,
    pub meshes: *mut *mut mesh,
}
impl Clone for model {
    fn clone(&self) -> Self { *self }
}
impl Default for model {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ctri {
    pub a: vec3,
    pub b: vec3,
    pub c: vec3,
    pub norm: vec3,
    pub bound: sphere,
}
impl Clone for ctri {
    fn clone(&self) -> Self { *self }
}
impl Default for ctri {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct cmesh {
    pub is_leaf: u8,
    pub data: [u64; 5usize],
}
impl cmesh {
    pub unsafe fn division(&mut self) -> *mut plane {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn front(&mut self) -> *mut *mut cmesh {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(24))
    }
    pub unsafe fn back(&mut self) -> *mut *mut cmesh {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(32))
    }
    pub unsafe fn triangles(&mut self) -> *mut *mut ctri {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn triangles_num(&mut self) -> *mut c_int {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(8))
    }
    pub unsafe fn bound(&mut self) -> *mut sphere {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(12))
    }
}
impl Clone for cmesh {
    fn clone(&self) -> Self { *self }
}
impl Default for cmesh {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct collision {
    pub collided: u8,
    pub time: c_float,
    pub point: vec3,
    pub norm: vec3,
    pub flags: c_int,
}
impl Clone for collision {
    fn clone(&self) -> Self { *self }
}
impl Default for collision {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type entity = c_void;

#[repr(C)]
#[derive(Copy)]
pub struct camera {
    pub position: vec3,
    pub target: vec3,
    pub fov: c_float,
    pub near_clip: c_float,
    pub far_clip: c_float,
}
impl Clone for camera {
    fn clone(&self) -> Self { *self }
}
impl Default for camera {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct light {
    pub position: vec3,
    pub target: vec3,
    pub diffuse_color: vec3,
    pub specular_color: vec3,
    pub ambient_color: vec3,
    pub power: c_float,
    pub falloff: c_float,
    pub enabled: u8,
    pub cast_shadows: u8,
    pub _type: c_int,
    pub shadow_color: vec3,
    pub shadow_map_width: c_int,
    pub shadow_map_height: c_int,
    pub orthographic: u8,
    pub ortho_width: c_float,
    pub ortho_height: c_float,
    pub fov: c_float,
    pub aspect_ratio: c_float,
}
impl Clone for light {
    fn clone(&self) -> Self { *self }
}
impl Default for light {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type asset = c_void;

#[repr(C)]
#[derive(Copy)]
pub struct asset_hndl {
    pub path: fpath,
    pub ptr: *mut asset,
    pub timestamp: uint32_t,
}
impl Clone for asset_hndl {
    fn clone(&self) -> Self { *self }
}
impl Default for asset_hndl {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct static_object {
    pub position: vec3,
    pub scale: vec3,
    pub rotation: quat,
    pub active: u8,
    pub recieve_shadows: u8,
    pub cast_shadows: u8,
    pub renderable: asset_hndl,
    pub collision_body: asset_hndl,
}
impl Clone for static_object {
    fn clone(&self) -> Self { *self }
}
impl Default for static_object {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct frame {
    pub joint_count: c_int,
    pub joint_parents: *mut c_int,
    pub joint_positions: *mut vec3,
    pub joint_rotations: *mut quat,
    pub transforms: *mut mat4,
    pub transforms_inv: *mut mat4,
}
impl Clone for frame {
    fn clone(&self) -> Self { *self }
}
impl Default for frame {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct skeleton {
    pub joint_count: c_int,
    pub joint_names: *mut *mut c_char,
    pub rest_pose: *mut frame,
}
impl Clone for skeleton {
    fn clone(&self) -> Self { *self }
}
impl Default for skeleton {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct animated_object {
    pub position: vec3,
    pub scale: vec3,
    pub rotation: quat,
    pub animation_time: c_float,
    pub renderable: asset_hndl,
    pub animation: asset_hndl,
    pub skeleton: asset_hndl,
    pub pose: *mut frame,
}
impl Clone for animated_object {
    fn clone(&self) -> Self { *self }
}
impl Default for animated_object {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct physics_object {
    pub position: vec3,
    pub scale: vec3,
    pub rotation: quat,
    pub velocity: vec3,
    pub angular_velocity: quat,
    pub acceleration: vec3,
    pub angular_acceleration: quat,
    pub previous_position: vec3,
    pub elasticity: c_float,
    pub friction: c_float,
    pub active: u8,
    pub recieve_shadows: u8,
    pub cast_shadows: u8,
    pub renderable: asset_hndl,
}
impl Clone for physics_object {
    fn clone(&self) -> Self { *self }
}
impl Default for physics_object {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct instance_data {
    pub position: vec3,
    pub scale: vec3,
    pub rotation: quat,
    pub world: mat4,
    pub world_normal: mat3,
}
impl Clone for instance_data {
    fn clone(&self) -> Self { *self }
}
impl Default for instance_data {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct instance_object {
    pub num_instances: c_int,
    pub instances: *mut instance_data,
    pub world_buffer: GLuint,
    pub bound: sphere,
    pub renderable: asset_hndl,
    pub collision_body: asset_hndl,
}
impl Clone for instance_object {
    fn clone(&self) -> Self { *self }
}
impl Default for instance_object {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct image {
    pub width: c_int,
    pub height: c_int,
    pub data: *mut c_uchar,
    pub repeat_type: c_int,
    pub sample_type: c_int,
}
impl Clone for image {
    fn clone(&self) -> Self { *self }
}
impl Default for image {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct terrain_chunk {
    pub id: c_int,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub bound: sphere,
    pub left: *mut terrain_chunk,
    pub right: *mut terrain_chunk,
    pub top: *mut terrain_chunk,
    pub bottom: *mut terrain_chunk,
    pub colmesh: *mut cmesh,
    pub num_verts: c_int,
    pub vertex_buffer: GLuint,
    pub num_indicies: [c_int; 7usize],
    pub index_buffers: [GLuint; 7usize],
}
impl Clone for terrain_chunk {
    fn clone(&self) -> Self { *self }
}
impl Default for terrain_chunk {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct terrain {
    pub width: c_int,
    pub height: c_int,
    pub heightmap: *mut c_float,
    pub chunk_width: c_int,
    pub chunk_height: c_int,
    pub num_chunks: c_int,
    pub num_cols: c_int,
    pub num_rows: c_int,
    pub chunks: *mut *mut terrain_chunk,
}
impl Clone for terrain {
    fn clone(&self) -> Self { *self }
}
impl Default for terrain {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct landscape_blobtree {
    pub bound: sphere,
    pub is_leaf: u8,
    pub chunk_index: c_int,
    pub child0: *mut landscape_blobtree,
    pub child1: *mut landscape_blobtree,
    pub child2: *mut landscape_blobtree,
    pub child3: *mut landscape_blobtree,
}
impl Clone for landscape_blobtree {
    fn clone(&self) -> Self { *self }
}
impl Default for landscape_blobtree {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct landscape {
    pub heightmap: asset_hndl,
    pub attribmap: asset_hndl,
    pub attribimage: *mut image,
    pub scale: c_float,
    pub size_x: c_float,
    pub size_y: c_float,
    pub blobtree: *mut landscape_blobtree,
    pub ground0: asset_hndl,
    pub ground1: asset_hndl,
    pub ground2: asset_hndl,
    pub ground3: asset_hndl,
    pub ground0_nm: asset_hndl,
    pub ground1_nm: asset_hndl,
    pub ground2_nm: asset_hndl,
    pub ground3_nm: asset_hndl,
}
impl Clone for landscape {
    fn clone(&self) -> Self { *self }
}
impl Default for landscape {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct effect_key {
    pub time: c_float,
    pub rotation: c_float,
    pub rotation_r: c_float,
    pub scale: vec3,
    pub scale_r: vec3,
    pub color: vec4,
    pub color_r: vec4,
    pub force: vec3,
    pub force_r: vec3,
}
impl Clone for effect_key {
    fn clone(&self) -> Self { *self }
}
impl Default for effect_key {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct effect {
    pub texture: asset_hndl,
    pub texture_nm: asset_hndl,
    pub blend_src: GLuint,
    pub blend_dst: GLuint,
    pub count: c_int,
    pub depth: c_float,
    pub thickness: c_float,
    pub bumpiness: c_float,
    pub scattering: c_float,
    pub lifetime: c_float,
    pub output: c_float,
    pub output_r: c_float,
    pub alpha_decay: u8,
    pub color_decay: u8,
    pub keys_num: c_int,
    pub keys: *mut effect_key,
}
impl Clone for effect {
    fn clone(&self) -> Self { *self }
}
impl Default for effect {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct particles {
    pub position: vec3,
    pub rotation: quat,
    pub scale: vec3,
    pub effect: asset_hndl,
    pub rate: c_float,
    pub count: c_int,
    pub actives: *mut u8,
    pub seeds: *mut c_float,
    pub times: *mut c_float,
    pub rotations: *mut c_float,
    pub scales: *mut vec3,
    pub colors: *mut vec4,
    pub positions: *mut vec3,
    pub velocities: *mut vec3,
    pub vertex_buff: GLuint,
    pub vertex_data: *mut c_float,
}
impl Clone for particles {
    fn clone(&self) -> Self { *self }
}
impl Default for particles {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct bucket {
    pub key: *mut c_char,
    pub item: *mut c_void,
    pub next: *mut bucket,
}
impl Clone for bucket {
    fn clone(&self) -> Self { *self }
}
impl Default for bucket {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct dict {
    pub size: c_int,
    pub buckets: *mut *mut bucket,
}
impl Clone for dict {
    fn clone(&self) -> Self { *self }
}
impl Default for dict {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct config {
    pub entries: *mut dict,
}
impl Clone for config {
    fn clone(&self) -> Self { *self }
}
impl Default for config {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lang {
    pub map: *mut dict,
}
impl Clone for lang {
    fn clone(&self) -> Self { *self }
}
impl Default for lang {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct font {
    pub texture_map: asset_hndl,
    pub width: c_int,
    pub height: c_int,
    pub locations: *mut vec2,
    pub sizes: *mut vec2,
    pub offsets: *mut vec2,
}
impl Clone for font {
    fn clone(&self) -> Self { *self }
}
impl Default for font {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type shader = GLuint;
pub type shader_program = GLuint;

#[repr(C)]
#[derive(Copy)]
pub struct texture {
    pub handle: GLuint,
    pub _type: GLenum,
}
impl Clone for texture {
    fn clone(&self) -> Self { *self }
}
impl Default for texture {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct material_item {
    pub data: [u64; 514usize],
}
impl material_item {
    pub unsafe fn as_int(&mut self) -> *mut c_int {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn as_float(&mut self) -> *mut c_float {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn as_vec2(&mut self) -> *mut vec2 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn as_vec3(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn as_vec4(&mut self) -> *mut vec4 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn as_asset(&mut self) -> *mut asset_hndl {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
}
impl Clone for material_item {
    fn clone(&self) -> Self { *self }
}
impl Default for material_item {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct material_entry {
    pub program: *mut shader_program,
    pub num_items: c_int,
    pub types: *mut c_int,
    pub names: *mut *mut c_char,
    pub items: *mut material_item,
}
impl Clone for material_entry {
    fn clone(&self) -> Self { *self }
}
impl Default for material_entry {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct material {
    pub num_entries: c_int,
    pub entries: *mut *mut material_entry,
}
impl Clone for material {
    fn clone(&self) -> Self { *self }
}
impl Default for material {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vertex_weight {
    pub bone_ids: [c_int; 3usize],
    pub bone_weights: [c_float; 3usize],
}
impl Clone for vertex_weight {
    fn clone(&self) -> Self { *self }
}
impl Default for vertex_weight {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct renderable_surface {
    pub vertex_vbo: GLuint,
    pub triangle_vbo: GLuint,
    pub num_verticies: c_int,
    pub num_triangles: c_int,
    pub bound: sphere,
}
impl Clone for renderable_surface {
    fn clone(&self) -> Self { *self }
}
impl Default for renderable_surface {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct renderable {
    pub surfaces: *mut *mut renderable_surface,
    pub num_surfaces: c_int,
    pub is_rigged: u8,
    pub material: asset_hndl,
}
impl Clone for renderable {
    fn clone(&self) -> Self { *self }
}
impl Default for renderable {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct animation {
    pub frame_count: c_int,
    pub frame_time: c_float,
    pub frames: *mut *mut frame,
}
impl Clone for animation {
    fn clone(&self) -> Self { *self }
}
impl Default for animation {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_text {
    pub string: *mut c_char,
    pub positions_buffer: GLuint,
    pub texcoords_buffer: GLuint,
    pub colors_buffer: GLuint,
    pub num_positions: c_int,
    pub num_texcoords: c_int,
    pub top_left: vec2,
    pub bottom_right: vec2,
    pub font: asset_hndl,
    pub position: vec2,
    pub scale: vec2,
    pub color: vec4,
    pub halign: c_int,
    pub valign: c_int,
    pub line_spacing: c_float,
    pub char_spacing: c_float,
    pub rotation: c_float,
    pub line_length: c_float,
    pub active: u8,
}
impl Clone for ui_text {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_text {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_style {
    pub box_back_image: fpath,
    pub box_back_width: c_int,
    pub box_back_height: c_int,
    pub box_back_tile: u8,
    pub box_back_border_size: c_int,
    pub box_back_border_color: vec4,
    pub box_glitch: c_float,
    pub box_blend_src: c_int,
    pub box_blend_dst: c_int,
    pub box_text_color: vec4,
    pub box_label_color: vec4,
    pub box_text_halign: c_int,
    pub box_text_valign: c_int,
    pub box_up_color: vec4,
    pub box_down_color: vec4,
    pub box_inset_color: vec4,
    pub text_font: fpath,
    pub text_color: vec4,
    pub text_scale: vec2,
    pub spinner_image: fpath,
    pub spinner_speed: c_float,
}
impl Clone for ui_style {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_style {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type ui_elem = c_void;

#[repr(C)]
#[derive(Copy)]
pub struct ui_rectangle {
    pub top_left: vec2,
    pub bottom_right: vec2,
    pub color: vec4,
    pub texture: asset_hndl,
    pub texture_width: c_int,
    pub texture_height: c_int,
    pub texture_tile: u8,
    pub border_size: c_float,
    pub border_color: vec4,
    pub glitch: c_float,
    pub time: c_float,
    pub blend_src: GLenum,
    pub blend_dst: GLenum,
    pub active: u8,
}
impl Clone for ui_rectangle {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_rectangle {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_spinner {
    pub top_left: vec2,
    pub bottom_right: vec2,
    pub color: vec4,
    pub texture: asset_hndl,
    pub speed: c_float,
    pub rotation: c_float,
    pub active: u8,
}
impl Clone for ui_spinner {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_spinner {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_button {
    pub back: *mut ui_rectangle,
    pub label: *mut ui_text,
    pub up_color: vec4,
    pub down_color: vec4,
    pub onclick: Option<unsafe extern "C" fn(arg1: *mut ui_button, data: *mut c_void) -> ()>,
    pub onclick_data: *mut c_void,
    pub active: u8,
    pub enabled: u8,
    pub pressed: u8,
}
impl Clone for ui_button {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_button {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_textbox {
    pub outer: *mut ui_rectangle,
    pub inner: *mut ui_rectangle,
    pub contents: *mut ui_text,
    pub label: *mut ui_text,
    pub password: u8,
    pub max_chars: c_int,
    pub selected: u8,
    pub active: u8,
    pub enabled: u8,
}
impl Clone for ui_textbox {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_textbox {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_listbox {
    pub back: *mut ui_rectangle,
    pub scroll: c_int,
    pub num_items: c_int,
    pub items: *mut *mut ui_text,
    pub active: u8,
    pub onselect: Option<unsafe extern "C" fn(entry: *mut ui_text) -> ()>,
}
impl Clone for ui_listbox {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_listbox {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_browser {
    pub outer: *mut ui_rectangle,
    pub inner: *mut ui_listbox,
    pub directory: fpath,
    pub active: u8,
}
impl Clone for ui_browser {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_browser {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_toast {
    pub label: *mut ui_text,
    pub opacity: c_float,
    pub active: u8,
}
impl Clone for ui_toast {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_toast {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_dialog {
    pub back: *mut ui_button,
    pub title: *mut ui_text,
    pub contents: *mut ui_text,
    pub single_button: u8,
    pub left: *mut ui_button,
    pub right: *mut ui_button,
}
impl Clone for ui_dialog {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_dialog {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_option {
    pub label: *mut ui_button,
    pub num_options: c_int,
    pub options: *mut *mut ui_button,
    pub active: u8,
    pub selected: c_int,
    pub onselect: Option<unsafe extern "C" fn(arg1: *mut ui_option) -> ()>,
}
impl Clone for ui_option {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_option {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ui_slider {
    pub label: *mut ui_button,
    pub slot: *mut ui_rectangle,
    pub bar: *mut ui_rectangle,
    pub pressed: u8,
    pub active: u8,
    pub amount: c_float,
}
impl Clone for ui_slider {
    fn clone(&self) -> Self { *self }
}
impl Default for ui_slider {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct sky {
    pub time: c_float,
    pub seed: uint32_t,
    pub cloud_mesh: [asset_hndl; 14usize],
    pub cloud_tex: [asset_hndl; 14usize],
    pub cloud_opacity: [c_float; 14usize],
    pub sun_sprite: asset_hndl,
    pub sun_tex: asset_hndl,
    pub moon_sprite: asset_hndl,
    pub moon_tex: asset_hndl,
    pub stars_sprite: asset_hndl,
    pub stars_tex: asset_hndl,
    pub is_day: u8,
    pub wind: vec3,
    pub world_sun: mat4,
    pub world_moon: mat4,
    pub world_stars: mat4,
    pub moon_power: c_float,
    pub moon_direction: vec3,
    pub moon_diffuse: vec3,
    pub moon_ambient: vec3,
    pub moon_specular: vec3,
    pub sun_power: c_float,
    pub sun_direction: vec3,
    pub sun_diffuse: vec3,
    pub sun_ambient: vec3,
    pub sun_specular: vec3,
    pub sky_power: c_float,
    pub sky_direction: vec3,
    pub sky_diffuse: vec3,
    pub sky_ambient: vec3,
    pub sky_specular: vec3,
    pub ground_power: c_float,
    pub ground_direction: vec3,
    pub ground_diffuse: vec3,
    pub ground_ambient: vec3,
    pub ground_specular: vec3,
}
impl Clone for sky {
    fn clone(&self) -> Self { *self }
}
impl Default for sky {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct render_object {
    pub _type: c_int,
    pub data: [u64; 12usize],
}
impl render_object {
    pub unsafe fn axis(&mut self) -> *mut mat4 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn sphere(&mut self) -> *mut sphere {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn ellipsoid(&mut self) -> *mut ellipsoid {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn colmesh(&mut self) -> *mut *mut cmesh {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn colworld(&mut self) -> *mut mat4 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(8))
    }
    pub unsafe fn frustum(&mut self) -> *mut frustum {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn plane(&mut self) -> *mut plane {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn line_start(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn line_end(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(12))
    }
    pub unsafe fn line_color(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(24))
    }
    pub unsafe fn line_thickness(&mut self) -> *mut c_float {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(36))
    }
    pub unsafe fn point_pos(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn point_color(&mut self) -> *mut vec3 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(12))
    }
    pub unsafe fn point_size(&mut self) -> *mut c_float {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(24))
    }
    pub unsafe fn static_object(&mut self) -> *mut *mut static_object {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn instance_object(&mut self) -> *mut *mut instance_object {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn animated_object(&mut self) -> *mut *mut animated_object {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn landscape(&mut self) -> *mut *mut landscape {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn particles(&mut self) -> *mut *mut particles {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn light(&mut self) -> *mut *mut light {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn paint_axis(&mut self) -> *mut mat4 {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn paint_radius(&mut self) -> *mut c_float {
        let raw: *mut u8 = mem::transmute(&self.data);
        mem::transmute(raw.offset(64))
    }
}
impl Clone for render_object {
    fn clone(&self) -> Self { *self }
}
impl Default for render_object {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct renderer {
    pub options: asset_hndl,
    pub camera: *mut camera,
    pub dyn_lights_num: c_int,
    pub dyn_light: [*mut light; 13usize],
    pub sky: *mut sky,
    pub mat_static: asset_hndl,
    pub mat_skin: asset_hndl,
    pub mat_instance: asset_hndl,
    pub mat_animated: asset_hndl,
    pub mat_vegetation: asset_hndl,
    pub mat_terrain: asset_hndl,
    pub mat_clear: asset_hndl,
    pub mat_ui: asset_hndl,
    pub mat_ssao: asset_hndl,
    pub mat_compose: asset_hndl,
    pub mat_tonemap: asset_hndl,
    pub mat_post0: asset_hndl,
    pub mat_post1: asset_hndl,
    pub mat_skydome: asset_hndl,
    pub mat_depth: asset_hndl,
    pub mat_depth_ins: asset_hndl,
    pub mat_depth_ani: asset_hndl,
    pub mat_depth_veg: asset_hndl,
    pub mat_depth_ter: asset_hndl,
    pub mat_sun: asset_hndl,
    pub mat_clouds: asset_hndl,
    pub mat_particles: asset_hndl,
    pub mat_sea: asset_hndl,
    pub mesh_skydome: asset_hndl,
    pub mesh_sphere: asset_hndl,
    pub mesh_sea: asset_hndl,
    pub tex_color_correction: asset_hndl,
    pub tex_random: asset_hndl,
    pub tex_random_perlin: asset_hndl,
    pub tex_environment: asset_hndl,
    pub tex_vignetting: asset_hndl,
    pub tex_sea_bump0: asset_hndl,
    pub tex_sea_bump1: asset_hndl,
    pub tex_sea_bump2: asset_hndl,
    pub tex_sea_bump3: asset_hndl,
    pub tex_sea_env: asset_hndl,
    pub tex_cube_sea: asset_hndl,
    pub tex_cube_field: asset_hndl,
    pub tex_white: asset_hndl,
    pub tex_grey: asset_hndl,
    pub tex_skin_lookup: asset_hndl,
    pub gfbo: GLuint,
    pub gdepth_buffer: GLuint,
    pub gdiffuse_buffer: GLuint,
    pub gnormals_buffer: GLuint,
    pub gdiffuse_texture: GLuint,
    pub gnormals_texture: GLuint,
    pub gdepth_texture: GLuint,
    pub ssao_fbo: GLuint,
    pub ssao_buffer: GLuint,
    pub ssao_texture: GLuint,
    pub hdr_fbo: GLuint,
    pub hdr_buffer: GLuint,
    pub hdr_texture: GLuint,
    pub ldr_front_fbo: GLuint,
    pub ldr_front_buffer: GLuint,
    pub ldr_front_texture: GLuint,
    pub ldr_back_fbo: GLuint,
    pub ldr_back_buffer: GLuint,
    pub ldr_back_texture: GLuint,
    pub shadows_fbo: [GLuint; 3usize],
    pub shadows_buffer: [GLuint; 3usize],
    pub shadows_texture: [GLuint; 3usize],
    pub shadows_start: [c_float; 3usize],
    pub shadows_end: [c_float; 3usize],
    pub shadows_widths: [c_int; 3usize],
    pub shadows_heights: [c_int; 3usize],
    pub seed: c_int,
    pub glitch: c_float,
    pub time: c_float,
    pub time_of_day: c_float,
    pub exposure: c_float,
    pub exposure_speed: c_float,
    pub exposure_target: c_float,
    pub skydome_enabled: u8,
    pub sea_enabled: u8,
    pub render_objects_num: c_int,
    pub render_objects: *mut render_object,
    pub camera_view: mat4,
    pub camera_proj: mat4,
    pub camera_inv_view: mat4,
    pub camera_inv_proj: mat4,
    pub camera_near: c_float,
    pub camera_far: c_float,
    pub camera_frustum: _box,
    pub shadow_view: [mat4; 3usize],
    pub shadow_proj: [mat4; 3usize],
    pub shadow_near: [c_float; 3usize],
    pub shadow_far: [c_float; 3usize],
    pub shadow_frustum: [_box; 3usize],
}
impl Clone for renderer {
    fn clone(&self) -> Self { *self }
}
impl Default for renderer {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct list {
    pub num_items: c_int,
    pub num_slots: c_int,
    pub items: *mut *mut c_void,
}
impl Clone for list {
    fn clone(&self) -> Self { *self }
}
impl Default for list {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct int_list {
    pub num_items: c_int,
    pub num_slots: c_int,
    pub items: *mut c_int,
}
impl Clone for int_list {
    fn clone(&self) -> Self { *self }
}
impl Default for int_list {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vertex_list {
    pub num_items: c_int,
    pub num_slots: c_int,
    pub items: *mut vertex,
}
impl Clone for vertex_list {
    fn clone(&self) -> Self { *self }
}
impl Default for vertex_list {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vertex_bucket {
    pub keys: *mut vertex_list,
    pub values: *mut int_list,
}
impl Clone for vertex_bucket {
    fn clone(&self) -> Self { *self }
}
impl Default for vertex_bucket {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct vertex_hashtable {
    pub items: *mut vertex_bucket,
    pub table_size: c_int,
}
impl Clone for vertex_hashtable {
    fn clone(&self) -> Self { *self }
}
impl Default for vertex_hashtable {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct spline {
    pub num_points: c_int,
    pub y0d: c_float,
    pub ynd: c_float,
    pub x0d: c_float,
    pub xnd: c_float,
    pub x: [c_float; 20usize],
    pub y: [c_float; 20usize],
    pub yd: [c_float; 20usize],
    pub xd: [c_float; 20usize],
}
impl Clone for spline {
    fn clone(&self) -> Self { *self }
}
impl Default for spline {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct color_curves {
    pub rgb_spline: *mut spline,
    pub r_spline: *mut spline,
    pub g_spline: *mut spline,
    pub b_spline: *mut spline,
    pub a_spline: *mut spline,
}
impl Clone for color_curves {
    fn clone(&self) -> Self { *self }
}
impl Default for color_curves {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[link(name = "corange")]
extern "C" {
    pub static mut sys_nerr: c_int;
    pub static mut sys_errlist: *const *const c_char;
    pub static mut error_buf: [c_char; 2048usize];
    pub static mut error_str: [c_char; 2048usize];
    pub static mut warning_buf: [c_char; 2048usize];
    pub static mut warning_str: [c_char; 2048usize];
    pub static mut debug_buf: [c_char; 2048usize];
    pub static mut debug_str: [c_char; 2048usize];
    pub static mut ui_style_current: *mut ui_style;
    pub static mut ui_style_corange: ui_style;
    pub static mut ui_style_hunt: ui_style;
    pub static mut dialog_count: c_int;

    pub fn P(path: *const c_char) -> fpath;
    pub fn fpath_full(path: fpath) -> fpath;
    pub fn fpath_file(path: fpath) -> fpath;
    pub fn fpath_file_location(path: fpath) -> fpath;
    pub fn fpath_file_extension(path: fpath) -> fpath;
    pub fn at_error(func: Option<unsafe extern "C" fn(arg1: *const c_char) -> ()>) -> ();
    pub fn at_warning(func: Option<unsafe extern "C" fn(arg1: *const c_char) -> ()>) -> ();
    pub fn at_debug(func: Option<unsafe extern "C" fn(arg1: *const c_char) -> ()>) -> ();
    pub fn error_(arg1: *const c_char) -> ();
    pub fn warning_(arg1: *const c_char) -> ();
    pub fn debug_(arg1: *const c_char) -> ();
    pub fn timer_start(id: c_int, tag: *const c_char) -> timer;
    pub fn timer_split(t: timer, tag: *const c_char) -> timer;
    pub fn timer_stop(t: timer, tag: *const c_char) -> timer;
    pub fn timestamp(out: *mut c_char) -> ();
    pub fn frame_begin() -> ();
    pub fn frame_end() -> ();
    pub fn frame_end_at_rate(fps: c_double) -> ();
    pub fn frame_rate() -> c_double;
    pub fn frame_time() -> c_double;
    pub fn frame_rate_string() -> *mut c_char;
    pub fn type_find(_type: *const c_char, size: size_t) -> type_id;
    pub fn type_id_name(id: c_int) -> *mut c_char;
    pub fn max(x: c_float, y: c_float) -> c_float;
    pub fn min(x: c_float, y: c_float) -> c_float;
    pub fn clamp(x: c_float, bottom: c_float, top: c_float) -> c_float;
    pub fn saturate(x: c_float) -> c_float;
    pub fn between(x: c_float, bottom: c_float, top: c_float) -> u8;
    pub fn between_or(x: c_float, bottom: c_float, top: c_float) -> u8;
    pub fn lerp(p1: c_float, p2: c_float, amount: c_float) -> c_float;
    pub fn smoothstep(p1: c_float, p2: c_float, amount: c_float) -> c_float;
    pub fn smootherstep(p1: c_float, p2: c_float, amount: c_float) -> c_float;
    pub fn cosine_interp(p1: c_float, p2: c_float, amount: c_float) -> c_float;
    pub fn cubic_interp(p1: c_float, p2: c_float, p3: c_float, p4: c_float, amount: c_float) -> c_float;
    pub fn nearest_interp(p1: c_float, p2: c_float, amount: c_float) -> c_float;
    pub fn binearest_interp(tl: c_float, tr: c_float, bl: c_float, br: c_float, x_amount: c_float, y_amount: c_float) -> c_float;
    pub fn bilinear_interp(tl: c_float, tr: c_float, bl: c_float, br: c_float, x_amount: c_float, y_amount: c_float) -> c_float;
    pub fn bicosine_interp(tl: c_float, tr: c_float, bl: c_float, br: c_float, x_amount: c_float, y_amount: c_float) -> c_float;
    pub fn bismoothstep_interp(tl: c_float, tr: c_float, bl: c_float, br: c_float, x_amount: c_float, y_amount: c_float) -> c_float;
    pub fn bismootherstep_interp(tl: c_float, tr: c_float, bl: c_float, br: c_float, x_amount: c_float, y_amount: c_float) -> c_float;
    pub fn vec2_new(x: c_float, y: c_float) -> vec2;
    pub fn vec2_zero() -> vec2;
    pub fn vec2_one() -> vec2;
    pub fn vec2_add(v1: vec2, v2: vec2) -> vec2;
    pub fn vec2_sub(v1: vec2, v2: vec2) -> vec2;
    pub fn vec2_mul(v: vec2, fac: c_float) -> vec2;
    pub fn vec2_mul_vec2(v1: vec2, v2: vec2) -> vec2;
    pub fn vec2_div(v: vec2, fac: c_float) -> vec2;
    pub fn vec2_div_vec2(v1: vec2, v2: vec2) -> vec2;
    pub fn vec2_pow(v: vec2, exp: c_float) -> vec2;
    pub fn vec2_neg(v: vec2) -> vec2;
    pub fn vec2_abs(v: vec2) -> vec2;
    pub fn vec2_floor(v: vec2) -> vec2;
    pub fn vec2_fmod(v: vec2, val: c_float) -> vec2;
    pub fn vec2_max(v: vec2, x: c_float) -> vec2;
    pub fn vec2_min(v: vec2, x: c_float) -> vec2;
    pub fn vec2_clamp(v: vec2, b: c_float, t: c_float) -> vec2;
    pub fn vec2_equ(v1: vec2, v2: vec2) -> u8;
    pub fn vec2_dot(v1: vec2, v2: vec2) -> c_float;
    pub fn vec2_length_sqrd(v: vec2) -> c_float;
    pub fn vec2_length(v: vec2) -> c_float;
    pub fn vec2_dist_sqrd(v1: vec2, v2: vec2) -> c_float;
    pub fn vec2_dist(v1: vec2, v2: vec2) -> c_float;
    pub fn vec2_dist_manhattan(v1: vec2, v2: vec2) -> c_float;
    pub fn vec2_normalize(v: vec2) -> vec2;
    pub fn vec2_reflect(v1: vec2, v2: vec2) -> vec2;
    pub fn vec2_from_string(s: *mut c_char) -> vec2;
    pub fn vec2_print(v: vec2) -> ();
    pub fn vec2_to_array(v: vec2, out: *mut c_float) -> ();
    pub fn vec2_hash(v: vec2) -> c_int;
    pub fn vec2_mix_hash(v: vec2) -> c_int;
    pub fn vec2_saturate(v: vec2) -> vec2;
    pub fn vec2_lerp(v1: vec2, v2: vec2, amount: c_float) -> vec2;
    pub fn vec2_smoothstep(v1: vec2, v2: vec2, amount: c_float) -> vec2;
    pub fn vec2_smootherstep(v1: vec2, v2: vec2, amount: c_float) -> vec2;
    pub fn vec3_new(x: c_float, y: c_float, z: c_float) -> vec3;
    pub fn vec3_zero() -> vec3;
    pub fn vec3_one() -> vec3;
    pub fn vec3_up() -> vec3;
    pub fn vec3_red() -> vec3;
    pub fn vec3_green() -> vec3;
    pub fn vec3_blue() -> vec3;
    pub fn vec3_white() -> vec3;
    pub fn vec3_black() -> vec3;
    pub fn vec3_grey() -> vec3;
    pub fn vec3_light_grey() -> vec3;
    pub fn vec3_dark_grey() -> vec3;
    pub fn vec3_add(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_sub(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_mul(v: vec3, fac: c_float) -> vec3;
    pub fn vec3_mul_vec3(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_div(v: vec3, fac: c_float) -> vec3;
    pub fn vec3_div_vec3(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_pow(v: vec3, fac: c_float) -> vec3;
    pub fn vec3_neg(v: vec3) -> vec3;
    pub fn vec3_abs(v: vec3) -> vec3;
    pub fn vec3_floor(v: vec3) -> vec3;
    pub fn vec3_fmod(v: vec3, val: c_float) -> vec3;
    pub fn vec3_equ(v1: vec3, v2: vec3) -> u8;
    pub fn vec3_neq(v1: vec3, v2: vec3) -> u8;
    pub fn vec3_dot(v1: vec3, v2: vec3) -> c_float;
    pub fn vec3_length_sqrd(v: vec3) -> c_float;
    pub fn vec3_length(v: vec3) -> c_float;
    pub fn vec3_dist_sqrd(v1: vec3, v2: vec3) -> c_float;
    pub fn vec3_dist(v1: vec3, v2: vec3) -> c_float;
    pub fn vec3_dist_manhattan(v1: vec3, v2: vec3) -> c_float;
    pub fn vec3_cross(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_normalize(v: vec3) -> vec3;
    pub fn vec3_reflect(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_project(v1: vec3, v2: vec3) -> vec3;
    pub fn vec3_from_string(s: *mut c_char) -> vec3;
    pub fn vec3_print(v: vec3) -> ();
    pub fn vec3_to_array(v: vec3, out: *mut c_float) -> ();
    pub fn vec3_hash(v: vec3) -> c_int;
    pub fn vec3_saturate(v: vec3) -> vec3;
    pub fn vec3_lerp(v1: vec3, v2: vec3, amount: c_float) -> vec3;
    pub fn vec3_smoothstep(v1: vec3, v2: vec3, amount: c_float) -> vec3;
    pub fn vec3_smootherstep(v1: vec3, v2: vec3, amount: c_float) -> vec3;
    pub fn vec4_new(x: c_float, y: c_float, z: c_float, w: c_float) -> vec4;
    pub fn vec4_zero() -> vec4;
    pub fn vec4_one() -> vec4;
    pub fn vec4_red() -> vec4;
    pub fn vec4_green() -> vec4;
    pub fn vec4_blue() -> vec4;
    pub fn vec4_white() -> vec4;
    pub fn vec4_black() -> vec4;
    pub fn vec4_grey() -> vec4;
    pub fn vec4_light_grey() -> vec4;
    pub fn vec4_dark_grey() -> vec4;
    pub fn vec4_add(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_sub(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_mul(v: vec4, fac: c_float) -> vec4;
    pub fn vec4_mul_vec4(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_div(v: vec4, fac: c_float) -> vec4;
    pub fn vec4_pow(v: vec4, fac: c_float) -> vec4;
    pub fn vec4_neg(v: vec4) -> vec4;
    pub fn vec4_abs(v: vec4) -> vec4;
    pub fn vec4_floor(v: vec4) -> vec4;
    pub fn vec4_fmod(v: vec4, val: c_float) -> vec4;
    pub fn vec4_sqrt(v: vec4) -> vec4;
    pub fn vec4_max(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_min(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_equ(v1: vec4, v2: vec4) -> u8;
    pub fn vec4_dot(v1: vec4, v2: vec4) -> c_float;
    pub fn vec4_length_sqrd(v: vec4) -> c_float;
    pub fn vec4_length(v: vec4) -> c_float;
    pub fn vec4_dist_sqrd(v1: vec4, v2: vec4) -> c_float;
    pub fn vec4_dist(v1: vec4, v2: vec4) -> c_float;
    pub fn vec4_dist_manhattan(v1: vec4, v2: vec4) -> c_float;
    pub fn vec4_normalize(v: vec4) -> vec4;
    pub fn vec4_reflect(v1: vec4, v2: vec4) -> vec4;
    pub fn vec4_from_string(s: *mut c_char) -> vec4;
    pub fn vec4_print(v: vec4) -> ();
    pub fn vec4_to_array(v: vec4, out: *mut c_float) -> ();
    pub fn vec3_to_homogeneous(v: vec3) -> vec4;
    pub fn vec4_from_homogeneous(v: vec4) -> vec3;
    pub fn vec4_hash(v: vec4) -> c_int;
    pub fn vec4_saturate(v: vec4) -> vec4;
    pub fn vec4_lerp(v1: vec4, v2: vec4, amount: c_float) -> vec4;
    pub fn vec4_smoothstep(v1: vec4, v2: vec4, amount: c_float) -> vec4;
    pub fn vec4_smootherstep(v1: vec4, v2: vec4, amount: c_float) -> vec4;
    pub fn vec4_nearest_interp(v1: vec4, v2: vec4, amount: c_float) -> vec4;
    pub fn vec4_binearest_interp(top_left: vec4, top_right: vec4, bottom_left: vec4, bottom_right: vec4, x_amount: c_float, y_amount: c_float) -> vec4;
    pub fn vec4_bilinear_interp(top_left: vec4, top_right: vec4, bottom_left: vec4, bottom_right: vec4, x_amount: c_float, y_amount: c_float) -> vec4;
    pub fn quat_id() -> quat;
    pub fn quat_new(x: c_float, y: c_float, z: c_float, w: c_float) -> quat;
    pub fn quat_from_euler(r: vec3) -> quat;
    pub fn quat_angle_axis(angle: c_float, axis: vec3) -> quat;
    pub fn quat_rotation_x(angle: c_float) -> quat;
    pub fn quat_rotation_y(angle: c_float) -> quat;
    pub fn quat_rotation_z(angle: c_float) -> quat;
    pub fn quat_at(q: quat, i: c_int) -> c_float;
    pub fn quat_real(q: quat) -> c_float;
    pub fn quat_imaginaries(q: quat) -> vec3;
    pub fn quat_to_angle_axis(q: quat, axis: *mut vec3, angle: *mut c_float) -> ();
    pub fn quat_to_euler(q: quat) -> vec3;
    pub fn quat_neg(q: quat) -> quat;
    pub fn quat_dot(q1: quat, q2: quat) -> c_float;
    pub fn quat_scale(q: quat, f: c_float) -> quat;
    pub fn quat_mul_quat(q1: quat, q2: quat) -> quat;
    pub fn quat_mul_vec3(q: quat, v: vec3) -> vec3;
    pub fn quat_inverse(q: quat) -> quat;
    pub fn quat_unit_inverse(q: quat) -> quat;
    pub fn quat_length(q: quat) -> c_float;
    pub fn quat_normalize(q: quat) -> quat;
    pub fn quat_exp(w: vec3) -> quat;
    pub fn quat_log(q: quat) -> vec3;
    pub fn quat_slerp(q1: quat, q2: quat, amount: c_float) -> quat;
    pub fn quat_constrain(q: quat, axis: vec3) -> quat;
    pub fn quat_constrain_y(q: quat) -> quat;
    pub fn quat_distance(q0: quat, q1: quat) -> c_float;
    pub fn quat_interpolate(qs: *mut quat, ws: *mut c_float, count: c_int) -> quat;
    pub fn quat_dual_id() -> quat_dual;
    pub fn quat_dual_new(real: quat, dual: quat) -> quat_dual;
    pub fn quat_dual_transform(q: quat, t: vec3) -> quat_dual;
    pub fn quat_dual_mul(q0: quat_dual, q1: quat_dual) -> quat_dual;
    pub fn quat_dual_mul_vec3(q: quat_dual, v: vec3) -> vec3;
    pub fn quat_dual_mul_vec3_rot(q: quat_dual, v: vec3) -> vec3;
    pub fn mat2_id() -> mat2;
    pub fn mat2_zero() -> mat2;
    pub fn mat2_new(xx: c_float, xy: c_float, yx: c_float, yy: c_float) -> mat2;
    pub fn mat2_mul_mat2(m1: mat2, mat2: mat2) -> mat2;
    pub fn mat2_mul_vec2(m: mat2, v: vec2) -> vec2;
    pub fn mat2_transpose(m: mat2) -> mat2;
    pub fn mat2_det(m: mat2) -> c_float;
    pub fn mat2_inverse(m: mat2) -> mat2;
    pub fn mat2_to_array(m: mat2, out: *mut c_float) -> ();
    pub fn mat2_print(m: mat2) -> ();
    pub fn mat2_rotation(a: c_float) -> mat2;
    pub fn mat3_id() -> mat3;
    pub fn mat3_zero() -> mat3;
    pub fn mat3_new(xx: c_float, xy: c_float, xz: c_float, yx: c_float, yy: c_float, yz: c_float, zx: c_float, zy: c_float, zz: c_float) -> mat3;
    pub fn mat3_mul_mat3(m1: mat3, mat2: mat3) -> mat3;
    pub fn mat3_mul_vec3(m: mat3, v: vec3) -> vec3;
    pub fn mat3_transpose(m: mat3) -> mat3;
    pub fn mat3_det(m: mat3) -> c_float;
    pub fn mat3_inverse(m: mat3) -> mat3;
    pub fn mat3_to_array(m: mat3, out: *mut c_float) -> ();
    pub fn mat3_print(m: mat3) -> ();
    pub fn mat3_scale(s: vec3) -> mat3;
    pub fn mat3_rotation_x(a: c_float) -> mat3;
    pub fn mat3_rotation_y(a: c_float) -> mat3;
    pub fn mat3_rotation_z(a: c_float) -> mat3;
    pub fn mat3_rotation_angle_axis(angle: c_float, axis: vec3) -> mat3;
    pub fn mat4_id() -> mat4;
    pub fn mat4_zero() -> mat4;
    pub fn mat4_new(xx: c_float, xy: c_float, xz: c_float, xw: c_float, yx: c_float, yy: c_float, yz: c_float, yw: c_float, zx: c_float, zy: c_float, zz: c_float, zw: c_float, wx: c_float, wy: c_float, wz: c_float, ww: c_float) -> mat4;
    pub fn mat4_at(m: mat4, i: c_int, j: c_int) -> c_float;
    pub fn mat4_set(m: mat4, x: c_int, y: c_int, v: c_float) -> mat4;
    pub fn mat4_transpose(m: mat4) -> mat4;
    pub fn mat4_mul_mat4(m1: mat4, mat2: mat4) -> mat4;
    pub fn mat4_mul_vec4(m: mat4, v: vec4) -> vec4;
    pub fn mat4_mul_vec3(m: mat4, v: vec3) -> vec3;
    pub fn mat4_det(m: mat4) -> c_float;
    pub fn mat4_inverse(m: mat4) -> mat4;
    pub fn mat3_to_mat4(m: mat3) -> mat4;
    pub fn mat4_to_mat3(m: mat4) -> mat3;
    pub fn mat4_to_quat(m: mat4) -> quat;
    pub fn mat4_to_quat_dual(m: mat4) -> quat_dual;
    pub fn mat4_to_array(m: mat4, out: *mut c_float) -> ();
    pub fn mat4_to_array_trans(m: mat4, out: *mut c_float) -> ();
    pub fn mat4_print(m: mat4) -> ();
    pub fn mat4_translation(v: vec3) -> mat4;
    pub fn mat4_scale(v: vec3) -> mat4;
    pub fn mat4_rotation_x(a: c_float) -> mat4;
    pub fn mat4_rotation_y(a: c_float) -> mat4;
    pub fn mat4_rotation_z(a: c_float) -> mat4;
    pub fn mat4_rotation_axis_angle(axis: vec3, angle: c_float) -> mat4;
    pub fn mat4_rotation_euler(x: c_float, y: c_float, z: c_float) -> mat4;
    pub fn mat4_rotation_quat(q: quat) -> mat4;
    pub fn mat4_rotation_quat_dual(q: quat_dual) -> mat4;
    pub fn mat4_view_look_at(position: vec3, target: vec3, up: vec3) -> mat4;
    pub fn mat4_perspective(fov: c_float, near_clip: c_float, far_clip: c_float, ratio: c_float) -> mat4;
    pub fn mat4_orthographic(left: c_float, right: c_float,  bottom: c_float, top: c_float, near: c_float, far: c_float) -> mat4;
    pub fn mat4_world(position: vec3, scale: vec3, rotation: quat) -> mat4;
    pub fn mat4_lerp(m1: mat4, mat2: mat4, amount: c_float) -> mat4;
    pub fn mat4_smoothstep(m1: mat4, mat2: mat4, amount: c_float) -> mat4;
    pub fn plane_new(position: vec3, direction: vec3) -> plane;
    pub fn plane_transform(p: plane, world: mat4, world_normal: mat3) -> plane;
    pub fn plane_transform_space(p: plane, space: mat3, space_normal: mat3) -> plane;
    pub fn plane_distance(p: plane, point: vec3) -> c_float;
    pub fn point_inside_plane(point: vec3, p: plane) -> u8;
    pub fn point_outside_plane(point: vec3, p: plane) -> u8;
    pub fn point_intersects_plane(point: vec3, p: plane) -> u8;
    pub fn point_swept_inside_plane(point: vec3, v: vec3, p: plane) -> u8;
    pub fn point_swept_outside_plane(point: vec3, v: vec3, p: plane) -> u8;
    pub fn point_swept_intersects_plane(point: vec3, v: vec3, p: plane) -> u8;
    pub fn plane_closest(p: plane, v: vec3) -> vec3;
    pub fn plane_project(p: plane, v: vec3) -> vec3;
    pub fn box_new(x_min: c_float, x_max: c_float, y_min: c_float, y_max: c_float, z_min: c_float, z_max: c_float) -> _box;
    pub fn box_sphere(center: vec3, radius: c_float) -> _box;
    pub fn box_merge(b1: _box, b2: _box) -> _box;
    pub fn box_transform(b1: _box, world: mat4, world_normal: mat3) -> _box;
    pub fn box_invert(b: _box) -> _box;
    pub fn box_invert_depth(b: _box) -> _box;
    pub fn box_invert_width(b: _box) -> _box;
    pub fn box_invert_height(b: _box) -> _box;
    pub fn point_inside_box(point: vec3, b: _box) -> u8;
    pub fn point_outside_box(point: vec3, b: _box) -> u8;
    pub fn point_intersects_box(point: vec3, b: _box) -> u8;
    pub fn frustum_new(ntr: vec3, ntl: vec3, nbr: vec3, nbl: vec3, ftr: vec3, ftl: vec3, fbr: vec3, fbl: vec3) -> frustum;
    pub fn frustum_new_clipbox() -> frustum;
    pub fn frustum_new_camera(view: mat4, proj: mat4) -> frustum;
    pub fn frustum_slice(f: frustum, start: c_float, end: c_float) -> frustum;
    pub fn frustum_transform(f: frustum, m: mat4) -> frustum;
    pub fn frustum_translate(f: frustum, v: vec3) -> frustum;
    pub fn frustum_center(f: frustum) -> vec3;
    pub fn frustum_maximums(f: frustum) -> vec3;
    pub fn frustum_minimums(f: frustum) -> vec3;
    pub fn frustum_box(f: frustum) -> _box;
    pub fn frustum_outside_box(f: frustum, b: _box) -> u8;
    pub fn sphere_unit() -> sphere;
    pub fn sphere_point() -> sphere;
    pub fn sphere_new(center: vec3, radius: c_float) -> sphere;
    pub fn sphere_merge(s1: sphere, s2: sphere) -> sphere;
    pub fn sphere_merge_many(s: *mut sphere, count: c_int) -> sphere;
    pub fn sphere_transform(s: sphere, world: mat4) -> sphere;
    pub fn sphere_translate(s: sphere, x: vec3) -> sphere;
    pub fn sphere_scale(s: sphere, x: c_float) -> sphere;
    pub fn sphere_transform_space(s: sphere, space: mat3) -> sphere;
    pub fn sphere_of_box(bb: _box) -> sphere;
    pub fn sphere_of_frustum(f: frustum) -> sphere;
    pub fn sphere_inside_box(s: sphere, b: _box) -> u8;
    pub fn sphere_outside_box(s: sphere, b: _box) -> u8;
    pub fn sphere_intersects_box(s: sphere, b: _box) -> u8;
    pub fn sphere_inside_frustum(s: sphere, f: frustum) -> u8;
    pub fn sphere_outside_frustum(s: sphere, f: frustum) -> u8;
    pub fn sphere_intersects_frustum(s: sphere, f: frustum) -> u8;
    pub fn sphere_outside_sphere(s1: sphere, s2: sphere) -> u8;
    pub fn sphere_inside_sphere(s1: sphere, s2: sphere) -> u8;
    pub fn sphere_intersects_sphere(s1: sphere, s2: sphere) -> u8;
    pub fn point_inside_sphere(s: sphere, point: vec3) -> u8;
    pub fn point_outside_sphere(s: sphere, point: vec3) -> u8;
    pub fn point_intersects_sphere(s: sphere, point: vec3) -> u8;
    pub fn line_inside_sphere(s: sphere, start: vec3, end: vec3) -> u8;
    pub fn line_outside_sphere(s: sphere, start: vec3, end: vec3) -> u8;
    pub fn line_intersects_sphere(s: sphere, start: vec3, end: vec3) -> u8;
    pub fn sphere_inside_plane(s: sphere, p: plane) -> u8;
    pub fn sphere_outside_plane(s: sphere, p: plane) -> u8;
    pub fn sphere_intersects_plane(s: sphere, p: plane) -> u8;
    pub fn point_swept_inside_sphere(s: sphere, v: vec3, point: vec3) -> u8;
    pub fn point_swept_outside_sphere(s: sphere, v: vec3, point: vec3) -> u8;
    pub fn point_swept_intersects_sphere(s: sphere, v: vec3, point: vec3) -> u8;
    pub fn sphere_swept_inside_plane(s: sphere, v: vec3, p: plane) -> u8;
    pub fn sphere_swept_outside_plane(s: sphere, v: vec3, p: plane) -> u8;
    pub fn sphere_swept_intersects_plane(s: sphere, v: vec3, p: plane) -> u8;
    pub fn sphere_swept_outside_sphere(s1: sphere, v: vec3, s2: sphere) -> u8;
    pub fn sphere_swept_inside_sphere(s1: sphere, v: vec3, s2: sphere) -> u8;
    pub fn sphere_swept_intersects_sphere(s1: sphere, v: vec3, s2: sphere) -> u8;
    pub fn point_inside_triangle(p: vec3, v0: vec3, v1: vec3, v2: vec3) -> u8;
    pub fn sphere_intersects_face(s: sphere, v0: vec3, v1: vec3, v2: vec3, norm: vec3) -> u8;
    pub fn ellipsoid_new(center: vec3, radiuses: vec3) -> ellipsoid;
    pub fn ellipsoid_transform(e: ellipsoid, m: mat4) -> ellipsoid;
    pub fn ellipsoid_of_sphere(s: sphere) -> ellipsoid;
    pub fn ellipsoid_space(e: ellipsoid) -> mat3;
    pub fn ellipsoid_inv_space(e: ellipsoid) -> mat3;
    pub fn capsule_new(start: vec3, end: vec3, radius: c_float) -> capsule;
    pub fn capsule_transform(c: capsule, m: mat4) -> capsule;
    pub fn capsule_inside_plane(c: capsule, p: plane) -> u8;
    pub fn capsule_outside_plane(c: capsule, p: plane) -> u8;
    pub fn capsule_intersects_plane(c: capsule, p: plane) -> u8;
    pub fn vertex_new() -> vertex;
    pub fn vertex_equal(v1: vertex, v2: vertex) -> u8;
    pub fn vertex_print(v: vertex) -> ();
    pub fn mesh_new() -> *mut mesh;
    pub fn mesh_delete(m: *mut mesh) -> ();
    pub fn mesh_generate_normals(m: *mut mesh) -> ();
    pub fn mesh_generate_tangents(m: *mut mesh) -> ();
    pub fn mesh_generate_orthagonal_tangents(m: *mut mesh) -> ();
    pub fn mesh_generate_texcoords_cylinder(m: *mut mesh) -> ();
    pub fn mesh_print(m: *mut mesh) -> ();
    pub fn mesh_surface_area(m: *mut mesh) -> c_float;
    pub fn mesh_transform(m: *mut mesh, transform: mat4) -> ();
    pub fn mesh_translate(m: *mut mesh, translation: vec3) -> ();
    pub fn mesh_scale(m: *mut mesh, scale: c_float) -> ();
    pub fn mesh_bounding_sphere(m: *mut mesh) -> sphere;
    pub fn model_new() -> *mut model;
    pub fn model_delete(m: *mut model) -> ();
    pub fn model_generate_normals(m: *mut model) -> ();
    pub fn model_generate_tangents(m: *mut model) -> ();
    pub fn model_generate_orthagonal_tangents(m: *mut model) -> ();
    pub fn model_generate_texcoords_cylinder(m: *mut model) -> ();
    pub fn model_print(m: *mut model) -> ();
    pub fn model_surface_area(m: *mut model) -> c_float;
    pub fn model_transform(m: *mut model, transform: mat4) -> ();
    pub fn model_translate(m: *mut model, translation: vec3) -> ();
    pub fn model_scale(m: *mut model, scale: c_float) -> ();
    pub fn triangle_tangent(v1: vertex, v2: vertex, v3: vertex) -> vec3;
    pub fn triangle_binormal(v1: vertex, v2: vertex, v3: vertex) -> vec3;
    pub fn triangle_normal(v1: vertex, v2: vertex, v3: vertex) -> vec3;
    pub fn triangle_random_position(v1: vertex, v2: vertex, v3: vertex) -> vec3;
    pub fn triangle_area(v1: vertex, v2: vertex, v3: vertex) -> c_float;
    pub fn triangle_difference_u(v1: vertex, v2: vertex, v3: vertex) -> c_float;
    pub fn triangle_difference_v(v1: vertex, v2: vertex, v3: vertex) -> c_float;
    pub fn triangle_random_position_interpolation(v1: vertex, v2: vertex,  v3: vertex) -> vertex;
    pub fn tween_approach(curr: c_float, target: c_float, timestep: c_float, steepness: c_float) -> c_float;
    pub fn tween_linear(curr: c_float, target: c_float, timestep: c_float, max: c_float) -> c_float;
    pub fn vec3_tween_approach(curr: vec3, target: vec3, timestep: c_float, steepness: c_float) -> vec3;
    pub fn vec3_tween_linear(curr: vec3, target: vec3, timestep: c_float, max: c_float) -> vec3;
    pub fn graphics_init() -> ();
    pub fn graphics_finish() -> ();
    pub fn graphics_set_vsync(vsync: u8) -> ();
    pub fn graphics_set_multisamples(samples: c_int) -> ();
    pub fn graphics_set_fullscreen(fullscreen: u8) -> ();
    pub fn graphics_set_antialiasing(quality: c_int) -> ();
    pub fn graphics_context_new() -> *mut SDL_GLContext;
    pub fn graphics_context_delete(context: *mut SDL_GLContext) -> ();
    pub fn graphics_context_current(context: *mut SDL_GLContext) -> ();
    pub fn graphics_get_multisamples() -> c_int;
    pub fn graphics_get_fullscreen() -> u8;
    pub fn graphics_get_antialiasing() -> c_int;
    pub fn graphics_viewport_set_title(title: *const c_char) -> ();
    pub fn graphics_viewport_set_icon(icon: fpath) -> ();
    pub fn graphics_viewport_set_position(x: c_int, y: c_int) -> ();
    pub fn graphics_viewport_set_size(w: c_int, h: c_int) -> ();
    pub fn graphics_viewport_screenshot() -> ();
    pub fn graphics_viewport_title() -> *const c_char;
    pub fn graphics_viewport_height() -> c_int;
    pub fn graphics_viewport_width() -> c_int;
    pub fn graphics_viewport_ratio() -> c_float;
    pub fn graphics_set_cursor_hidden(hidden: u8) -> ();
    pub fn graphics_get_cursor_hidden() -> u8;
    pub fn graphics_swap() -> ();
    pub fn joystick_init() -> ();
    pub fn joystick_finish() -> ();
    pub fn joystick_count() -> c_int;
    pub fn joystick_get(i: c_int) -> *mut SDL_Joystick;
    pub fn net_init() -> ();
    pub fn net_finish() -> ();
    pub fn net_set_server(server: u8) -> ();
    pub fn net_is_server() -> u8;
    pub fn net_is_client() -> u8;
    pub fn net_http_get(out: *mut c_char, max: c_int, fmt: *mut c_char, ...) -> c_int;
    pub fn net_http_upload(filename: *const c_char, fmt: *mut c_char, ...) -> c_int;
    pub fn ctri_new(a: vec3, b: vec3, c: vec3, norm: vec3) -> ctri;
    pub fn ctri_transform(t: ctri, m: mat4, mn: mat3) -> ctri;
    pub fn ctri_transform_space(t: ctri, s: mat3, sn: mat3) -> ctri;
    pub fn ctri_inside_plane(t: ctri, p: plane) -> u8;
    pub fn ctri_outside_plane(t: ctri, p: plane) -> u8;
    pub fn ctri_intersects_plane(t: ctri, p: plane) -> u8;
    pub fn col_load_file(filename: *mut c_char) -> *mut cmesh;
    pub fn cmesh_delete(cm: *mut cmesh) -> ();
    pub fn cmesh_bound(cm: *mut cmesh) -> sphere;
    pub fn cmesh_subdivide(cm: *mut cmesh, iterations: c_int) -> ();
    pub fn vec3_gravity() -> vec3;
    pub fn quadratic(a: c_float, b: c_float, c: c_float, t0: *mut c_float, t1: *mut c_float) -> u8;
    pub fn collision_none() -> collision;
    pub fn collision_new(time: c_float, point: vec3, norm: vec3) -> collision;
    pub fn collision_merge(c0: collision, c1: collision) -> collision;
    pub fn point_collide_point(p: vec3, v: vec3, p0: vec3) -> collision;
    pub fn point_collide_sphere(p: vec3, v: vec3, s: sphere) -> collision;
    pub fn point_collide_ellipsoid(p: vec3, v: vec3, e: ellipsoid) -> collision;
    pub fn point_collide_edge(p: vec3, v: vec3, e0: vec3, e1: vec3) -> collision;
    pub fn point_collide_face(p: vec3, v: vec3, ct: ctri) -> collision;
    pub fn point_collide_ctri(p: vec3, v: vec3, ct: ctri) -> collision;
    pub fn point_collide_mesh(p: vec3, v: vec3, m: *mut cmesh, world: mat4, world_normal: mat3) -> collision;
    pub fn sphere_collide_point(s: sphere, v: vec3, p: vec3) -> collision;
    pub fn sphere_collide_sphere(s: sphere, v: vec3, s0: sphere) -> collision;
    pub fn sphere_collide_edge(s: sphere, v: vec3, e0: vec3, e1: vec3) -> collision;
    pub fn sphere_collide_face(s: sphere, v: vec3, ct: ctri) -> collision;
    pub fn sphere_collide_ctri(s: sphere, v: vec3, ct: ctri) -> collision;
    pub fn sphere_collide_mesh(s: sphere, v: vec3, m: *mut cmesh, world: mat4, world_normal: mat3) -> collision;
    pub fn ellipsoid_collide_mesh(e: ellipsoid, v: vec3, m: *mut cmesh, world: mat4, world_normal: mat3) -> collision;
    pub fn ellipsoid_collide_point(e: ellipsoid, v: vec3, p: vec3) -> collision;
    pub fn ellipsoid_collide_sphere(e: ellipsoid, v: vec3, s: sphere) -> collision;
    pub fn collision_response_slide(x: *mut c_void, position: *mut vec3, velocity: *mut vec3, colfunc: Option<unsafe extern "C" fn(x: *mut c_void, pos: *mut vec3, vel: *mut vec3) -> collision>) -> ();
    pub fn corange_init(core_assets_path: *const c_char) -> ();
    pub fn corange_finish() -> ();
    pub fn entity_init() -> ();
    pub fn entity_finish() -> ();
    pub fn entity_handler_cast(type_id: c_int, entity_new: c_void, entity_del: unsafe extern "C" fn(entity: *mut c_void) -> ()) -> ();
    pub fn entity_exists(fmt: *mut c_char, ...) -> u8;
    pub fn entity_get(fmt: *const c_char, ...) -> *mut entity;
    pub fn entity_get_as_type_id(fmt: *mut c_char, type_id: c_int, ...) -> *mut entity;
    pub fn entity_new_type_id(fmt: *const c_char, type_id: c_int, ...) -> *mut entity;
    pub fn entity_delete(fmt: *mut c_char, ...) -> ();
    pub fn entity_name(e: *mut entity) -> *mut c_char;
    pub fn entity_typename(a: *mut entity) -> *mut c_char;
    pub fn entity_type_count_type_id(type_id: c_int) -> c_int;
    pub fn entities_new_type_id(name_format: *const c_char, count: c_int, type_id: c_int) -> ();
    pub fn entities_get_type_id(out: *mut *mut entity, returned: *mut c_int, type_id: c_int) -> ();
    pub fn camera_new() -> *mut camera;
    pub fn camera_delete(cam: *mut camera) -> ();
    pub fn camera_direction(c: *mut camera) -> vec3;
    pub fn camera_view_matrix(c: *mut camera) -> mat4;
    pub fn camera_proj_matrix(c: *mut camera) -> mat4;
    pub fn camera_view_proj_matrix(c: *mut camera) -> mat4;
    pub fn camera_normalize_target(c: *mut camera) -> ();
    pub fn camera_control_orbit(c: *mut camera, e: SDL_Event) -> ();
    pub fn camera_control_freecam(c: *mut camera, timestep: c_float) -> ();
    pub fn camera_control_joyorbit(c: *mut camera, timestep: c_float) -> ();
    pub fn light_new() -> *mut light;
    pub fn light_new_position(position: vec3) -> *mut light;
    pub fn light_new_type(position: vec3, _type: c_int) -> *mut light;
    pub fn light_delete(l: *mut light) -> ();
    pub fn light_set_type(l: *mut light, _type: c_int) -> ();
    pub fn light_direction(l: *mut light) -> vec3;
    pub fn light_view_matrix(l: *mut light) -> mat4;
    pub fn light_proj_matrix(l: *mut light) -> mat4;
    pub fn asset_hndl_null() -> asset_hndl;
    pub fn asset_hndl_new(path: fpath) -> asset_hndl;
    pub fn asset_hndl_new_load(path: fpath) -> asset_hndl;
    pub fn asset_hndl_new_ptr(_as: *mut asset) -> asset_hndl;
    pub fn asset_hndl_isnull(ah: *mut asset_hndl) -> u8;
    pub fn asset_hndl_path(ah: *mut asset_hndl) -> fpath;
    pub fn asset_hndl_ptr(ah: *mut asset_hndl) -> *mut asset;
    pub fn asset_hndl_eq(ah0: *mut asset_hndl, ah1: *mut asset_hndl) -> u8;
    pub fn asset_cache_flush() -> ();
    pub fn asset_init() -> ();
    pub fn asset_finish() -> ();
    pub fn asset_add_path_variable(variable: fpath, mapping: fpath) -> ();
    pub fn asset_map_filename(filename: fpath) -> fpath;
    pub fn asset_unmap_filename(filename: fpath) -> fpath;
    pub fn asset_handler_cast(_type: type_id, extension: *const c_char, asset_loader: unsafe extern "C" fn(filename: *const c_char) -> *mut asset, asset_deleter: unsafe extern "C" fn(asset: *mut asset) -> ()) -> ();
    pub fn file_load(filename: fpath) -> ();
    pub fn file_unload(filename: fpath) -> ();
    pub fn file_reload(filename: fpath) -> ();
    pub fn file_isloaded(path: fpath) -> u8;
    pub fn file_exists(path: fpath) -> u8;
    pub fn folder_load(folder: fpath) -> ();
    pub fn folder_unload(folder: fpath) -> ();
    pub fn folder_reload(folder: fpath) -> ();
    pub fn folder_load_recursive(folder: fpath) -> ();
    pub fn asset_get_load(path: fpath) -> *mut asset;
    pub fn asset_get(path: fpath) -> *mut asset;
    pub fn asset_get_as_type(path: fpath, _type: type_id) -> *mut asset;
    pub fn asset_reload_type_id(_type: type_id) -> ();
    pub fn asset_reload_all() -> ();
    pub fn asset_ptr_path(a: *mut asset) -> *mut c_char;
    pub fn asset_ptr_typename(a: *mut asset) -> *mut c_char;
    pub fn static_object_new() -> *mut static_object;
    pub fn static_object_delete(s: *mut static_object) -> ();
    pub fn static_object_world(s: *mut static_object) -> mat4;
    pub fn static_object_world_normal(s: *mut static_object) -> mat3;
    pub fn frame_new() -> *mut frame;
    pub fn frame_copy(f: *mut frame) -> *mut frame;
    pub fn frame_interpolate(f0: *mut frame, f1: *mut frame, amount: c_float) -> *mut frame;
    pub fn frame_copy_to(f: *mut frame, out: *mut frame) -> ();
    pub fn frame_interpolate_to(f0: *mut frame, f1: *mut frame, amount: c_float, out: *mut frame) -> ();
    pub fn frame_decendants_to(f0: *mut frame, f1: *mut frame, amount: c_float, joint: c_int, out: *mut frame) -> ();
    pub fn frame_delete(f: *mut frame) -> ();
    pub fn frame_joint_transform(f: *mut frame, i: c_int) -> mat4;
    pub fn frame_joint_add(f: *mut frame, parent: c_int, position: vec3, rotation: quat) -> ();
    pub fn frame_gen_transforms(f: *mut frame) -> ();
    pub fn frame_gen_inv_transforms(f: *mut frame) -> ();
    pub fn skeleton_new() -> *mut skeleton;
    pub fn skeleton_delete(s: *mut skeleton) -> ();
    pub fn skeleton_joint_add(s: *mut skeleton, name: *mut c_char, parent: c_int) -> ();
    pub fn skeleton_joint_id(s: *mut skeleton, name: *mut c_char) -> c_int;
    pub fn skl_load_file(filename: *mut c_char) -> *mut skeleton;
    pub fn animated_object_new() -> *mut animated_object;
    pub fn animated_object_delete(ao: *mut animated_object) -> ();
    pub fn animated_object_load_skeleton(ao: *mut animated_object, ah: asset_hndl) -> ();
    pub fn animated_object_update(ao: *mut animated_object, timestep: c_float) -> ();
    pub fn physics_object_new() -> *mut physics_object;
    pub fn physics_object_delete(po: *mut physics_object) -> ();
    pub fn physics_object_collide_static(po: *mut physics_object, so: *mut static_object, timestep: c_float) -> ();
    pub fn physics_object_update(po: *mut physics_object, timestep: c_float) -> ();
    pub fn instance_object_new() -> *mut instance_object;
    pub fn instance_object_delete(io: *mut instance_object) -> ();
    pub fn instance_object_update(io: *mut instance_object) -> ();
    pub fn instance_object_add_instance(io: *mut instance_object, position: vec3, scale: vec3, rotation: quat) -> ();
    pub fn instance_object_rem_instance(io: *mut instance_object, i: c_int) -> ();
    pub fn instance_object_world(io: *mut instance_object, i: c_int) -> mat4;
    pub fn instance_object_world_normal(io: *mut instance_object, i: c_int) -> mat3;
    pub fn image_new(width: c_int, height: c_int, data: *mut c_uchar) -> *mut image;
    pub fn image_empty(width: c_int, height: c_int) -> *mut image;
    pub fn image_blank(width: c_int, height: c_int) -> *mut image;
    pub fn image_copy(src: *mut image) -> *mut image;
    pub fn image_delete(i: *mut image) -> ();
    pub fn image_get(i: *mut image, u: c_int, v: c_int) -> vec4;
    pub fn image_set(i: *mut image, u: c_int, v: c_int, c: vec4) -> ();
    pub fn image_map(i: *mut image, f: Option<extern "C" fn(arg1: vec4) -> vec4>) -> ();
    pub fn image_red_channel(src: *mut image) -> *mut image;
    pub fn image_green_channel(src: *mut image) -> *mut image;
    pub fn image_blue_channel(src: *mut image) -> *mut image;
    pub fn image_alpha_channel(src: *mut image) -> *mut image;
    pub fn image_bgr_to_rgb(i: *mut image) -> ();
    pub fn image_rotate_90_clockwise(i: *mut image) -> ();
    pub fn image_rotate_90_counterclockwise(i: *mut image) -> ();
    pub fn image_rotate_180(i: *mut image) -> ();
    pub fn image_rotate_inplace(i: *mut image, amount: c_float) -> ();
    pub fn image_flip_horizontal(i: *mut image) -> ();
    pub fn image_flip_vertical(i: *mut image) -> ();
    pub fn image_fill(i: *mut image, color: vec4) -> ();
    pub fn image_fill_black(i: *mut image) -> ();
    pub fn image_fill_white(i: *mut image) -> ();
    pub fn image_apply_gamma(i: *mut image, amount: c_float) -> ();
    pub fn image_to_gamma(i: *mut image) -> ();
    pub fn image_from_gamma(i: *mut image) -> ();
    pub fn image_rgb_to_hsv(i: *mut image) -> ();
    pub fn image_hsv_to_rgb(i: *mut image) -> ();
    pub fn image_hsv_scalar(i: *mut image) -> ();
    pub fn image_min(i: *mut image) -> vec4;
    pub fn image_max(i: *mut image) -> vec4;
    pub fn image_mean(i: *mut image) -> vec4;
    pub fn image_var(i: *mut image) -> vec4;
    pub fn image_std(i: *mut image) -> vec4;
    pub fn image_auto_contrast(i: *mut image) -> ();
    pub fn image_set_to_mask(i: *mut image) -> ();
    pub fn image_set_brightness(i: *mut image, brightness: c_float) -> ();
    pub fn image_alpha_mean(i: *mut image) -> vec4;
    pub fn image_get_subimage(src: *mut image, left: c_int, top: c_int, width: c_int, height: c_int) -> *mut image;
    pub fn image_set_subimage(i: *mut image, left: c_int, top: c_int, src: *mut image) -> ();
    pub fn image_paste_subimage(i: *mut image, left: c_int, top: c_int, src: *mut image) -> ();
    pub fn image_sample(i: *mut image, uv: vec2) -> vec4;
    pub fn image_paint(i: *mut image, uv: vec2, color: vec4) -> ();
    pub fn image_scale(i: *mut image, scale: vec2) -> ();
    pub fn image_mask_not(i: *mut image) -> ();
    pub fn image_mask_binary(i0: *mut image, i1: *mut image, f: Option<extern "C" fn(arg1: u8, arg2: u8) -> u8>) -> ();
    pub fn image_mask_or(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_and(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_xor(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_nor(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_nand(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_xnor(i0: *mut image, i1: *mut image) -> ();
    pub fn image_mask_alpha(i: *mut image) -> *mut image;
    pub fn image_mask_nearest(i: *mut image) -> *mut image;
    pub fn image_mask_flood_fill(src: *mut image, u: c_int, v: c_int, tolerance: c_float) -> *mut image;
    pub fn image_mask_difference(src: *mut image, color: vec4, tolerance: c_float) -> *mut image;
    pub fn image_mask_count(i: *mut image) -> c_long;
    pub fn image_mask_median(i: *mut image, u: *mut c_int, v: *mut c_int) -> ();
    pub fn image_mask_random(i: *mut image, u: *mut c_int, v: *mut c_int) -> ();
    pub fn image_read_from_file(filename: *mut c_char) -> *mut image;
    pub fn image_tga_load_file(filename: *mut c_char) -> *mut image;
    pub fn image_bmp_load_file(filename: *mut c_char) -> *mut image;
    pub fn image_write_to_file(i: *mut image, filename: *mut c_char) -> ();
    pub fn image_tga_save_file(i: *mut image, filename: *mut c_char) -> ();
    pub fn image_bmp_save_file(i: *mut image, filename: *mut c_char) -> ();
    pub fn terrain_chunk_delete(tc: *mut terrain_chunk) -> ();
    pub fn raw_load_file(filename: *mut c_char) -> *mut terrain;
    pub fn raw_save_file(ter: *mut terrain, filename: *mut c_char) -> ();
    pub fn terrain_delete(ter: *mut terrain) -> ();
    pub fn terrain_get_chunk(ter: *mut terrain, x: c_int, y: c_int) -> *mut terrain_chunk;
    pub fn terrain_reload_chunk(ter: *mut terrain, i: c_int) -> ();
    pub fn terrain_tbn(ter: *mut terrain, position: vec2) -> mat3;
    pub fn terrain_axis(ter: *mut terrain, position: vec2) -> mat3;
    pub fn terrain_height(ter: *mut terrain, position: vec2) -> c_float;
    pub fn terrain_normal(ter: *mut terrain, position: vec2) -> vec3;
    pub fn landscape_new() -> *mut landscape;
    pub fn landscape_delete(l: *mut landscape) -> ();
    pub fn landscape_blobtree_new(ls: *mut landscape) -> *mut landscape_blobtree;
    pub fn landscape_blobtree_delete(lbt: *mut landscape_blobtree) -> ();
    pub fn landscape_blobtree_generate(l: *mut landscape) -> ();
    pub fn landscape_world(l: *mut landscape) -> mat4;
    pub fn landscape_world_normal(l: *mut landscape) -> mat3;
    pub fn landscape_height(l: *mut landscape, pos: vec2) -> c_float;
    pub fn landscape_normal(l: *mut landscape, pos: vec2) -> vec3;
    pub fn landscape_axis(l: *mut landscape, pos: vec2) -> mat3;
    pub fn landscape_paint_height(l: *mut landscape, pos: vec2, radius: c_float, value: c_float, opacity: c_float) -> ();
    pub fn landscape_paint_color(l: *mut landscape, pos: vec2, radius: c_float, _type: c_int, opacity: c_float) -> ();
    pub fn landscape_chunks(l: *mut landscape, pos: vec2, chunks_out: *mut *mut terrain_chunk) -> ();
    pub fn effect_key_lerp(x: effect_key, y: effect_key, amount: c_float) -> effect_key;
    pub fn effect_new() -> *mut effect;
    pub fn effect_load_file(filename: *mut c_char) -> *mut effect;
    pub fn effect_delete(e: *mut effect) -> ();
    pub fn effect_get_key(e: *mut effect, ptime: c_float) -> effect_key;
    pub fn particles_new() -> *mut particles;
    pub fn particles_delete(p: *mut particles) -> ();
    pub fn particles_set_effect(p: *mut particles, effect: asset_hndl) -> ();
    pub fn particles_update(p: *mut particles, timestep: c_float, cam: *mut camera) -> ();
    pub fn bucket_new(string: *mut c_char, item: *mut c_void) -> *mut bucket;
    pub fn bucket_map(b: *mut bucket, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn bucket_filter_map(b: *mut bucket, filter: unsafe extern "C" fn(arg1: *mut c_void) -> c_int, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn bucket_delete_with(b: *mut bucket, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn bucket_delete_recursive(b: *mut bucket) -> ();
    pub fn bucket_print(b: *mut bucket) -> ();
    pub fn dict_new(size: c_int) -> *mut dict;
    pub fn dict_delete(d: *mut dict) -> ();
    pub fn dict_contains(d: *mut dict, string: *mut c_char) -> u8;
    pub fn dict_get(d: *mut dict, string: *mut c_char) -> *mut c_void;
    pub fn dict_set(d: *mut dict, string: *mut c_char, item: *mut c_void) -> ();
    pub fn dict_remove_with(d: *mut dict, string: *mut c_char, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn dict_map(d: *mut dict, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn dict_filter_map(d: *mut dict, filter: unsafe extern "C" fn(arg1: *mut c_void) -> c_int, func: unsafe extern "C" fn(arg1: *mut c_void)  -> ()) -> ();
    pub fn dict_print(d: *mut dict) -> ();
    pub fn dict_find(d: *mut dict, item: *mut c_void) -> *mut c_char;
    pub fn cfg_load_file(filename: *const c_char) -> *mut config;
    pub fn cfg_save_file(c: *mut config, filename: *const c_char) -> ();
    pub fn config_delete(c: *mut config) -> ();
    pub fn config_string(c: *mut config, key: *mut c_char) -> *mut c_char;
    pub fn config_int(c: *mut config, key: *mut c_char) -> c_int;
    pub fn config_float(c: *mut config, key: *mut c_char) -> c_float;
    pub fn config_bool(c: *mut config, key: *mut c_char) -> u8;
    pub fn config_set_string(c: *mut config, key: *mut c_char, val: *mut c_char) -> ();
    pub fn config_set_int(c: *mut config, key: *mut c_char, val: c_int) -> ();
    pub fn config_set_float(c: *mut config, key: *mut c_char, val: c_float) -> ();
    pub fn config_set_bool(c: *mut config, key: *mut c_char, val: u8) -> ();
    pub fn option_graphics_asset(c: *mut config, key: *mut c_char, high: asset_hndl, medium: asset_hndl, low: asset_hndl) -> asset_hndl;
    pub fn option_graphics_int(c: *mut config, key: *mut c_char, high: c_int, medium: c_int, low: c_int) -> c_int;
    pub fn option_graphics_float(c: *mut config, key: *mut c_char, high: c_float, medium: c_float, low: c_float) -> c_float;
    pub fn lang_load_file(filename: *const c_char) -> *mut lang;
    pub fn lang_delete(t: *mut lang) -> ();
    pub fn lang_get(t: *mut lang, id: *mut c_char) -> *mut c_char;
    pub fn set_language(t: asset_hndl) -> ();
    pub fn S(id: *mut c_char) -> *mut c_char;
    pub fn font_load_file(filename: *mut c_char) -> *mut font;
    pub fn font_delete(font: *mut font) -> ();
    pub fn vs_load_file(filename: *mut c_char) -> *mut shader;
    pub fn fs_load_file(filename: *mut c_char) -> *mut shader;
    pub fn gs_load_file(filename: *mut c_char) -> *mut shader;
    pub fn tcs_load_file(filename: *mut c_char) -> *mut shader;
    pub fn tes_load_file(filename: *mut c_char) -> *mut shader;
    pub fn shader_delete(s: *mut shader) -> ();
    pub fn shader_print_log(s: *mut shader) -> ();
    pub fn shader_handle(s: *mut shader) -> GLuint;
    pub fn shader_program_new() -> *mut shader_program;
    pub fn shader_program_delete(p: *mut shader_program) -> ();
    pub fn shader_program_has_shader(p: *mut shader_program, s: *mut shader) -> u8;
    pub fn shader_program_attach_shader(p: *mut shader_program, s: *mut shader) -> ();
    pub fn shader_program_link(p: *mut shader_program) -> ();
    pub fn shader_program_print_info(p: *mut shader_program) -> ();
    pub fn shader_program_print_log(p: *mut shader_program) -> ();
    pub fn shader_program_handle(p: *mut shader_program) -> GLuint;
    pub fn shader_program_get_attribute(p: *mut shader_program, name: *mut c_char) -> GLint;
    pub fn shader_program_enable(p: *mut shader_program) -> ();
    pub fn shader_program_disable(p: *mut shader_program) -> ();
    pub fn shader_program_set_int(p: *mut shader_program, name: *mut c_char, val: c_int) -> ();
    pub fn shader_program_set_float(p: *mut shader_program, name: *mut c_char, val: c_float) -> ();
    pub fn shader_program_set_vec2(p: *mut shader_program, name: *mut c_char, val: vec2) -> ();
    pub fn shader_program_set_vec3(p: *mut shader_program, name: *mut c_char, val: vec3) -> ();
    pub fn shader_program_set_vec4(p: *mut shader_program, name: *mut c_char, val: vec4) -> ();
    pub fn shader_program_set_mat3(p: *mut shader_program, name: *mut c_char, val: mat3) -> ();
    pub fn shader_program_set_mat4(p: *mut shader_program, name: *mut c_char, val: mat4) -> ();
    pub fn shader_program_set_float_array(p: *mut shader_program, name: *mut c_char, vals: *mut c_float, count: c_int) -> ();
    pub fn shader_program_set_vec2_array(p: *mut shader_program, name: *mut c_char, vals: *mut vec2, count: c_int) -> ();
    pub fn shader_program_set_vec3_array(p: *mut shader_program, name: *mut c_char, vals: *mut vec3, count: c_int) -> ();
    pub fn shader_program_set_vec4_array(p: *mut shader_program, name: *mut c_char, vals: *mut vec4, count: c_int) -> ();
    pub fn shader_program_set_mat4_array(p: *mut shader_program, name: *mut c_char, vals: *mut mat4, count: c_int) -> ();
    pub fn shader_program_set_texture(p: *mut shader_program, name: *mut c_char, index: c_int, t: asset_hndl) -> ();
    pub fn shader_program_set_texture_id(p: *mut shader_program, name: *mut c_char, index: c_int, t: GLint) -> ();
    pub fn shader_program_enable_attribute(p: *mut shader_program, name: *mut c_char, count: c_int, stride: c_int, ptr: *mut c_void) -> ();
    pub fn shader_program_enable_attribute_instance(p: *mut shader_program, name: *mut c_char, count: c_int, stride: c_int, ptr: *mut c_void) -> ();
    pub fn shader_program_disable_attribute(p: *mut shader_program, name: *mut c_char) -> ();
    pub fn shader_program_enable_attribute_instance_matrix(p: *mut shader_program, name: *mut c_char, ptr: *mut c_void) -> ();
    pub fn shader_program_disable_attribute_matrix(p: *mut shader_program, name: *mut c_char) -> ();
    pub fn texture_new() -> *mut texture;
    pub fn texture_new_handle(h: GLuint) -> *mut texture;
    pub fn texture_delete(t: *mut texture) -> ();
    pub fn texture_handle(t: *mut texture) -> GLuint;
    pub fn texture_type(t: *mut texture) -> GLenum;
    pub fn texture_set_image(t: *mut texture, i: *mut image) -> ();
    pub fn texture_get_image(t: *mut texture) -> *mut image;
    pub fn texture_generate_mipmaps(t: *mut texture) -> ();
    pub fn texture_set_filtering_nearest(t: *mut texture) -> ();
    pub fn texture_set_filtering_linear(t: *mut texture) -> ();
    pub fn texture_set_filtering_anisotropic(t: *mut texture) -> ();
    pub fn bmp_load_file(filename: *mut c_char) -> *mut texture;
    pub fn tga_load_file(filename: *mut c_char) -> *mut texture;
    pub fn dds_load_file(filename: *mut c_char) -> *mut texture;
    pub fn lut_load_file(filename: *mut c_char) -> *mut texture;
    pub fn acv_load_file(filename: *mut c_char) -> *mut texture;
    pub fn texture_write_to_file(t: *mut texture, filename: *mut c_char) -> ();
    pub fn texture3d_write_to_file(t: *mut texture, filename: *mut c_char) -> ();
    pub fn material_entry_delete(me: *mut material_entry) -> ();
    pub fn material_entry_item(me: *mut material_entry, name: *mut c_char) -> material_item;
    pub fn material_entry_has_item(me: *mut material_entry, name: *mut c_char) -> u8;
    pub fn material_entry_add_item(me: *mut material_entry, name: *mut c_char, _type: c_int, mi: material_item) -> ();
    pub fn material_new() -> *mut material;
    pub fn material_delete(m: *mut material) -> ();
    pub fn mat_load_file(filename: *mut c_char) -> *mut material;
    pub fn material_get_entry(m: *mut material, index: c_int) -> *mut material_entry;
    pub fn material_add_entry(m: *mut material) -> *mut material_entry;
    pub fn material_first_program(m: *mut material) -> *mut shader_program;
    pub fn renderable_surface_new(m: *mut mesh) -> *mut renderable_surface;
    pub fn renderable_surface_new_rigged(m: *mut mesh, weights: *mut vertex_weight) -> *mut renderable_surface;
    pub fn renderable_surface_delete(surface: *mut renderable_surface) -> ();
    pub fn renderable_new() -> *mut renderable;
    pub fn renderable_delete(r: *mut renderable) -> ();
    pub fn renderable_add_mesh(r: *mut renderable, m: *mut mesh) -> ();
    pub fn renderable_add_model(r: *mut renderable, m: *mut model) -> ();
    pub fn renderable_set_material(r: *mut renderable, mat: asset_hndl) -> ();
    pub fn renderable_to_model(r: *mut renderable) -> *mut model;
    pub fn bmf_load_file(filename: *mut c_char) -> *mut renderable;
    pub fn obj_load_file(filename: *mut c_char) -> *mut renderable;
    pub fn smd_load_file(filename: *mut c_char) -> *mut renderable;
    pub fn ply_load_file(filename: *mut c_char) -> *mut renderable;
    pub fn bmf_save_file(r: *mut renderable, filename: *mut c_char) -> ();
    pub fn animation_new() -> *mut animation;
    pub fn animation_delete(a: *mut animation) -> ();
    pub fn animation_duration(a: *mut animation) -> c_float;
    pub fn animation_add_frame(a: *mut animation, base: *mut frame) -> *mut frame;
    pub fn animation_frame(a: *mut animation, i: c_int) -> *mut frame;
    pub fn animation_sample(a: *mut animation, time: c_float) -> *mut frame;
    pub fn animation_sample_to(a: *mut animation, time: c_float, out: *mut frame) -> ();
    pub fn ani_load_file(filename: *mut c_char) -> *mut animation;
    pub fn ui_text_new() -> *mut ui_text;
    pub fn ui_text_new_string(string: *mut c_char) -> *mut ui_text;
    pub fn ui_text_delete(text: *mut ui_text) -> ();
    pub fn ui_text_move(text: *mut ui_text, pos: vec2) -> ();
    pub fn ui_text_set_font(text: *mut ui_text, font: asset_hndl) -> ();
    pub fn ui_text_set_color(text: *mut ui_text, color: vec4) -> ();
    pub fn ui_text_set_scale(text: *mut ui_text, scale: vec2) -> ();
    pub fn ui_text_align(text: *mut ui_text, halign: c_int, valign: c_int) -> ();
    pub fn ui_text_draw(text: *mut ui_text) -> ();
    pub fn ui_text_draw_string(text: *mut ui_text, string: *mut c_char) -> ();
    pub fn ui_text_event(text: *mut ui_text, e: SDL_Event) -> ();
    pub fn ui_text_update(text: *mut ui_text) -> ();
    pub fn ui_text_render(text: *mut ui_text) -> ();
    pub fn ui_text_contains_point(text: *mut ui_text, position: vec2) -> u8;
    pub fn ui_init() -> ();
    pub fn ui_finish() -> ();
    pub fn ui_set_style(s: *mut ui_style) -> ();
    pub fn ui_event(e: SDL_Event) -> ();
    pub fn ui_update() -> ();
    pub fn ui_render() -> ();
    pub fn ui_handler_cast(type_id: c_int, ui_elem_new_func: Option<extern "C" fn() -> *mut c_void>, ui_elem_del_func: Option<unsafe extern "C" fn(arg1: *mut ui_elem) -> ()>, ui_elem_event_func: Option<unsafe extern "C" fn(arg1: *mut ui_elem, arg2: SDL_Event) -> ()>, ui_elem_update_func: Option<unsafe extern "C" fn(arg1: *mut ui_elem) -> ()>, ui_elem_render_func: Option<unsafe extern "C" fn(arg1: *mut ui_elem) -> ()>) -> ();
    pub fn ui_elem_exists(fmt: *mut c_char, ...) -> u8;
    pub fn ui_elem_get(fmt: *mut c_char, ...) -> *mut ui_elem;
    pub fn ui_elem_get_as_type_id(fmt: *mut c_char, type_id: c_int, ...) -> *mut ui_elem;
    pub fn ui_elem_new_type_id(fmt: *mut c_char, type_id: c_int, ...) -> *mut ui_elem;
    pub fn ui_elem_delete(fmt: *mut c_char, ...) -> ();
    pub fn ui_elem_event(fmt: *mut c_char, e: SDL_Event, ...) -> ();
    pub fn ui_elem_update(fmt: *mut c_char, ...) -> ();
    pub fn ui_elem_render(fmt: *mut c_char, ...) -> ();
    pub fn ui_elem_name(e: *mut ui_elem) -> *mut c_char;
    pub fn ui_elem_typename(e: *mut ui_elem) -> *mut c_char;
    pub fn ui_rectangle_new() -> *mut ui_rectangle;
    pub fn ui_rectangle_delete(r: *mut ui_rectangle) -> ();
    pub fn ui_rectangle_event(r: *mut ui_rectangle, e: SDL_Event) -> ();
    pub fn ui_rectangle_update(r: *mut ui_rectangle) -> ();
    pub fn ui_rectangle_render(r: *mut ui_rectangle) -> ();
    pub fn ui_rectangle_move(r: *mut ui_rectangle, pos: vec2) -> ();
    pub fn ui_rectangle_resize(r: *mut ui_rectangle, size: vec2) -> ();
    pub fn ui_rectangle_set_texture(r: *mut ui_rectangle, tex: asset_hndl, width: c_int, height: c_int, tile: u8) -> ();
    pub fn ui_rectangle_set_border(r: *mut ui_rectangle, size: c_float, color: vec4) -> ();
    pub fn ui_rectangle_set_color(r: *mut ui_rectangle, color: vec4) -> ();
    pub fn ui_rectangle_set_glitch(r: *mut ui_rectangle, glitch: c_float) -> ();
    pub fn ui_rectangle_center(r: *mut ui_rectangle) -> vec2;
    pub fn ui_rectangle_contains_point(r: *mut ui_rectangle, pos: vec2) -> u8;
    pub fn ui_rectangle_blend(r: *mut ui_rectangle, blend_src: GLenum, blend_dst: GLenum) -> ();
    pub fn ui_rectangle_position(r: *mut ui_rectangle) -> vec2;
    pub fn ui_rectangle_size(r: *mut ui_rectangle) -> vec2;
    pub fn ui_spinner_new() -> *mut ui_spinner;
    pub fn ui_spinner_delete(s: *mut ui_spinner) -> ();
    pub fn ui_spinner_event(s: *mut ui_spinner, e: SDL_Event) -> ();
    pub fn ui_spinner_update(s: *mut ui_spinner) -> ();
    pub fn ui_spinner_render(s: *mut ui_spinner) -> ();
    pub fn ui_button_new() -> *mut ui_button;
    pub fn ui_button_delete(b: *mut ui_button) -> ();
    pub fn ui_button_move(b: *mut ui_button, pos: vec2) -> ();
    pub fn ui_button_resize(b: *mut ui_button, size: vec2) -> ();
    pub fn ui_button_set_label(b: *mut ui_button, label: *mut c_char) -> ();
    pub fn ui_button_set_label_color(b: *mut ui_button, color: vec4) -> ();
    pub fn ui_button_set_font(b: *mut ui_button, f: asset_hndl) -> ();
    pub fn ui_button_set_onclick(b: *mut ui_button, onclick: Option<unsafe extern "C" fn(arg1: *mut ui_button, arg2: *mut c_void) -> ()>) -> ();
    pub fn ui_button_set_onclick_data(b: *mut ui_button, data: *mut c_void) -> ();
    pub fn ui_button_set_active(b: *mut ui_button, active: u8) -> ();
    pub fn ui_button_set_enabled(b: *mut ui_button, enabled: u8) -> ();
    pub fn ui_button_set_texture(b: *mut ui_button, tex: asset_hndl, width: c_int, height: c_int, tile: u8) -> ();
    pub fn ui_button_disable(b: *mut ui_button) -> ();
    pub fn ui_button_enable(b: *mut ui_button) -> ();
    pub fn ui_button_position(b: *mut ui_button) -> vec2;
    pub fn ui_button_size(b: *mut ui_button) -> vec2;
    pub fn ui_button_event(b: *mut ui_button, e: SDL_Event) -> ();
    pub fn ui_button_update(b: *mut ui_button) -> ();
    pub fn ui_button_render(b: *mut ui_button) -> ();
    pub fn ui_button_contains_point(b: *mut ui_button, pos: vec2) -> u8;
    pub fn ui_textbox_new() -> *mut ui_textbox;
    pub fn ui_textbox_delete(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_set_password(tb: *mut ui_textbox, password: u8) -> ();
    pub fn ui_textbox_set_max_chars(tb: *mut ui_textbox, l: c_int) -> ();
    pub fn ui_textbox_addchar(tb: *mut ui_textbox, c: c_char) -> ();
    pub fn ui_textbox_rmchar(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_move(tb: *mut ui_textbox, pos: vec2) -> ();
    pub fn ui_textbox_resize(tb: *mut ui_textbox, size: vec2) -> ();
    pub fn ui_textbox_set_font(tb: *mut ui_textbox, f: asset_hndl) -> ();
    pub fn ui_textbox_set_label(tb: *mut ui_textbox, label: *mut c_char) -> ();
    pub fn ui_textbox_set_contents(tb: *mut ui_textbox, label: *mut c_char) -> ();
    pub fn ui_textbox_set_alignment(tb: *mut ui_textbox, halign: c_int, valign: c_int) -> ();
    pub fn ui_textbox_disable(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_enable(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_event(tb: *mut ui_textbox, e: SDL_Event) -> ();
    pub fn ui_textbox_update(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_render(tb: *mut ui_textbox) -> ();
    pub fn ui_textbox_contains_point(tb: *mut ui_textbox, p: vec2) -> u8;
    pub fn ui_listbox_new() -> *mut ui_listbox;
    pub fn ui_listbox_delete(lb: *mut ui_listbox) -> ();
    pub fn ui_listbox_clear(lb: *mut ui_listbox) -> ();
    pub fn ui_listbox_add_item(lb: *mut ui_listbox, item: *mut c_char) -> *mut ui_text;
    pub fn ui_listbox_move(lb: *mut ui_listbox, pos: vec2) -> ();
    pub fn ui_listbox_resize(lb: *mut ui_listbox, size: vec2) -> ();
    pub fn ui_listbox_event(lb: *mut ui_listbox, e: SDL_Event) -> ();
    pub fn ui_listbox_update(lb: *mut ui_listbox) -> ();
    pub fn ui_listbox_render(lb: *mut ui_listbox) -> ();
    pub fn ui_listbox_set_onselect(lb: *mut ui_listbox, onselect: Option<unsafe extern "C" fn(entry: *mut ui_text) -> ()>) -> ();
    pub fn ui_browser_new() -> *mut ui_browser;
    pub fn ui_browser_delete(b: *mut ui_browser) -> ();
    pub fn ui_browser_chdir(b: *mut ui_browser, dir: fpath) -> ();
    pub fn ui_browser_event(b: *mut ui_browser, e: SDL_Event) -> ();
    pub fn ui_browser_update(b: *mut ui_browser) -> ();
    pub fn ui_browser_render(b: *mut ui_browser) -> ();
    pub fn ui_toast_popup(fmt: *mut c_char, ...) -> ();
    pub fn ui_toast_new() -> *mut ui_toast;
    pub fn ui_toast_delete(t: *mut ui_toast) -> ();
    pub fn ui_toast_set_label(t: *mut ui_toast, label: *mut c_char) -> ();
    pub fn ui_toast_set_font(t: *mut ui_toast, f: asset_hndl) -> ();
    pub fn ui_toast_event(t: *mut ui_toast, e: SDL_Event) -> ();
    pub fn ui_toast_update(t: *mut ui_toast) -> ();
    pub fn ui_toast_render(t: *mut ui_toast) -> ();
    pub fn ui_dialog_new() -> *mut ui_dialog;
    pub fn ui_dialog_delete(d: *mut ui_dialog) -> ();
    pub fn ui_dialog_set_single_button(d: *mut ui_dialog, single: u8) -> ();
    pub fn ui_dialog_set_title(d: *mut ui_dialog, title: *mut c_char) -> ();
    pub fn ui_dialog_set_contents(d: *mut ui_dialog, contents: *mut c_char) -> ();
    pub fn ui_dialog_set_button_left(d: *mut ui_dialog, left: *mut c_char, onleft: Option<unsafe extern "C" fn(arg1: *mut ui_button, arg2: *mut c_void) -> ()>) -> ();
    pub fn ui_dialog_set_button_right(d: *mut ui_dialog, right: *mut c_char, onright: Option<unsafe extern "C" fn(arg1: *mut ui_button, arg2: *mut c_void) -> ()>) -> ();
    pub fn ui_dialog_set_font(d: *mut ui_dialog, fnt: asset_hndl) -> ();
    pub fn ui_dialog_event(d: *mut ui_dialog, e: SDL_Event) -> ();
    pub fn ui_dialog_update(d: *mut ui_dialog) -> ();
    pub fn ui_dialog_render(d: *mut ui_dialog) -> ();
    pub fn ui_option_new() -> *mut ui_option;
    pub fn ui_option_delete(o: *mut ui_option) -> ();
    pub fn ui_option_set_active(o: *mut ui_option, active: u8) -> ();
    pub fn ui_option_move(o: *mut ui_option, position: vec2) -> ();
    pub fn ui_option_set_options(o: *mut ui_option, label: *mut c_char, num: c_int, values: *mut *mut c_char) -> ();
    pub fn ui_option_get_selected(o: *mut ui_option) -> c_int;
    pub fn ui_option_set_selected(o: *mut ui_option, selected: c_int) -> ();
    pub fn ui_option_set_onselect(o: *mut ui_option, onselect: Option<unsafe extern "C" fn(arg1: *mut ui_option) -> ()>) -> ();
    pub fn ui_option_deactivate(o: *mut ui_option) -> ();
    pub fn ui_option_activate(o: *mut ui_option) -> ();
    pub fn ui_option_event(o: *mut ui_option, e: SDL_Event) -> ();
    pub fn ui_option_update(o: *mut ui_option) -> ();
    pub fn ui_option_render(o: *mut ui_option) -> ();
    pub fn ui_slider_new() -> *mut ui_slider;
    pub fn ui_slider_delete(s: *mut ui_slider) -> ();
    pub fn ui_slider_set_label(s: *mut ui_slider, label: *mut c_char) -> ();
    pub fn ui_slider_move(s: *mut ui_slider, position: vec2) -> ();
    pub fn ui_slider_set_amount(s: *mut ui_slider, amount: c_float) -> ();
    pub fn ui_slider_get_amount(s: *mut ui_slider) -> c_float;
    pub fn ui_slider_set_active(s: *mut ui_slider, active: u8) -> ();
    pub fn ui_slider_deactivate(s: *mut ui_slider) -> ();
    pub fn ui_slider_activate(s: *mut ui_slider) -> ();
    pub fn ui_slider_event(s: *mut ui_slider, e: SDL_Event) -> ();
    pub fn ui_slider_update(s: *mut ui_slider) -> ();
    pub fn ui_slider_render(s: *mut ui_slider) -> ();
    pub fn sky_new() -> *mut sky;
    pub fn sky_delete(s: *mut sky) -> ();
    pub fn sky_update(s: *mut sky, t: c_float, seed: uint32_t) -> ();
    pub fn render_object_static(s: *mut static_object) -> render_object;
    pub fn render_object_instance(s: *mut instance_object) -> render_object;
    pub fn render_object_animated(a: *mut animated_object) -> render_object;
    pub fn render_object_particles(p: *mut particles) -> render_object;
    pub fn render_object_light(l: *mut light) -> render_object;
    pub fn render_object_axis(a: mat4) -> render_object;
    pub fn render_object_sphere(s: sphere) -> render_object;
    pub fn render_object_ellipsoid(e: ellipsoid) -> render_object;
    pub fn render_object_frustum(f: frustum) -> render_object;
    pub fn render_object_plane(p: plane) -> render_object;
    pub fn render_object_cmesh(cm: *mut cmesh, world: mat4) -> render_object;
    pub fn render_object_landscape(l: *mut landscape) -> render_object;
    pub fn render_object_paint(paint_axis: mat4, paint_radius: c_float) -> render_object;
    pub fn render_object_line(start: vec3, end: vec3, color: vec3, thickness: c_float) -> render_object;
    pub fn render_object_point(pos: vec3, color: vec3, size: c_float) -> render_object;
    pub fn renderer_new(options: asset_hndl) -> *mut renderer;
    pub fn renderer_delete(dr: *mut renderer) -> ();
    pub fn renderer_set_camera(dr: *mut renderer, cam: *mut camera) -> ();
    pub fn renderer_set_color_correction(dr: *mut renderer, t: asset_hndl) -> ();
    pub fn renderer_set_vignetting(dr: *mut renderer, v: asset_hndl) -> ();
    pub fn renderer_set_glitch(dr: *mut renderer, glitch: c_float) -> ();
    pub fn renderer_set_skydome_enabled(dr: *mut renderer, enabled: u8) -> ();
    pub fn renderer_set_sea_enabled(dr: *mut renderer, enabled: u8) -> ();
    pub fn renderer_set_tod(dr: *mut renderer, tod: c_float, seed: c_int) -> ();
    pub fn renderer_add(dr: *mut renderer, ro: render_object) -> ();
    pub fn renderer_add_dyn_light(dr: *mut renderer, l: *mut light) -> ();
    pub fn renderer_render(dr: *mut renderer) -> ();
    pub fn list_new() -> *mut list;
    pub fn list_push_back(l: *mut list, item: *mut c_void) -> ();
    pub fn list_pop_back(l: *mut list) -> *mut c_void;
    pub fn list_pop_at(l: *mut list, index: c_int) -> *mut c_void;
    pub fn list_remove(l: *mut list, item: *mut c_void) -> ();
    pub fn list_get(l: *mut list, index: c_int) -> *mut c_void;
    pub fn list_set(l: *mut list, index: c_int, item: *mut c_void) -> ();
    pub fn list_is_empty(l: *mut list) -> u8;
    pub fn list_delete(l: *mut list) -> ();
    pub fn list_clear(l: *mut list) -> ();
    pub fn list_delete_with(l: *mut list, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn list_clear_with(l: *mut list, func: unsafe extern "C" fn(arg1: *mut c_void) -> ()) -> ();
    pub fn int_list_new() -> *mut int_list;
    pub fn int_list_delete(l: *mut int_list) -> ();
    pub fn int_list_push_back(l: *mut int_list, item: c_int) -> ();
    pub fn int_list_pop_back(l: *mut int_list) -> c_int;
    pub fn int_list_get(l: *mut int_list, index: c_int) -> c_int;
    pub fn int_list_set(l: *mut int_list, index: c_int, item: c_int) -> ();
    pub fn int_list_is_empty(l: *mut int_list) -> u8;
    pub fn int_list_clear(l: *mut int_list) -> ();
    pub fn vertex_list_new() -> *mut vertex_list;
    pub fn vertex_list_delete(l: *mut vertex_list) -> ();
    pub fn vertex_list_push_back(l: *mut vertex_list, item: vertex) -> ();
    pub fn vertex_list_pop_back(l: *mut vertex_list) -> vertex;
    pub fn vertex_list_get(l: *mut vertex_list, index: c_int) -> vertex;
    pub fn vertex_list_set(l: *mut vertex_list, index: c_int, item: vertex) -> ();
    pub fn vertex_list_clear(l: *mut vertex_list) -> ();
    pub fn vertex_hash(ht: *mut vertex_hashtable, v: vertex) -> c_int;
    pub fn vertex_hashtable_new(size: c_int) -> *mut vertex_hashtable;
    pub fn vertex_hashtable_delete(ht: *mut vertex_hashtable) -> ();
    pub fn vertex_hashtable_set(ht: *mut vertex_hashtable, key: vertex, value: c_int) -> ();
    pub fn vertex_hashtable_get(ht: *mut vertex_hashtable, key: vertex) -> c_int;
    pub fn spline_new() -> *mut spline;
    pub fn spline_delete(s: *mut spline) -> ();
    pub fn spline_add_point(s: *mut spline, p: vec2) -> ();
    pub fn spline_get_point(s: *mut spline, i: c_int) -> vec2;
    pub fn spline_set_point(s: *mut spline, i: c_int, p: vec2) -> ();
    pub fn spline_update(s: *mut spline) -> ();
    pub fn spline_print(s: *mut spline) -> ();
    pub fn spline_get_x(s: *mut spline, y: c_float) -> c_float;
    pub fn spline_get_y(s: *mut spline, x: c_float) -> c_float;
    pub fn spline_get_x_between(s: *mut spline, low: c_int, high: c_int, y: c_float) -> c_float;
    pub fn spline_get_y_between(s: *mut spline, low: c_int, high: c_int, x: c_float) -> c_float;
    pub fn color_curves_load(filename: *mut c_char) -> *mut color_curves;
    pub fn color_curves_delete(cc: *mut color_curves) -> ();
    pub fn color_curves_write_lut(cc: *mut color_curves, filename: *mut c_char) -> ();
    pub fn color_curves_map(cc: *mut color_curves, _in: vec3) -> vec3;
    pub fn randf() -> c_float;
    pub fn randf_seed(s: c_float) -> c_float;
    pub fn randf_n() -> c_float;
    pub fn randf_nseed(s: c_float) -> c_float;
    pub fn randf_scale(s: c_float) -> c_float;
    pub fn randf_nscale(s: c_float) -> c_float;
    pub fn randf_range(s: c_float, e: c_float) -> c_float;
    pub fn randf_circle(radius: c_float) -> vec2;
}
