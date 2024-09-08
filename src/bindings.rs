use std::ffi::{c_char, c_uchar, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[repr(C)]
pub struct Image {
	data: *const c_void,
	width: i32,
	height: i32,
	mipmaps: i32,
	format: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Texture2D {
	id: u32,
	pub width: i32,
	pub height: i32,
	mipmaps: i32,
	format: i32,
}

#[link(name = "raylib")]
extern "C" {
	pub fn InitWindow(width: i32, height: i32, title: *const c_char);
	pub fn WindowShouldClose() -> bool;
	pub fn SetTargetFPS(fps: i32);

	pub fn GetScreenWidth() -> i32;
	pub fn GetScreenHeight() -> i32;

	pub fn BeginDrawing();
	pub fn EndDrawing();
	pub fn ClearBackground(color: Color);

	pub fn LoadImageFromMemory(
		fileType: *const c_char,
		fileData: *const c_uchar,
		dataSize: i32,
	) -> Image;

	pub fn LoadTextureFromImage(image: Image) -> Texture2D;
	pub fn DrawTexturePro(
		texture: Texture2D,
		source: Rectangle,
		target: Rectangle,
		origin: Vector2,
		rotation: f32,
		tint: Color,
	);
}
