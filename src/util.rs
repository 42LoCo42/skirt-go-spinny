use crate::{
	Color, DrawTexturePro, LoadImageFromMemory, LoadTextureFromImage, Rectangle, Texture2D, Vector2,
};

const WHITE: Color = Color {
	r: 0xff,
	g: 0xff,
	b: 0xff,
	a: 0xff,
};

pub fn load_texture(data: &[u8]) -> Texture2D {
	unsafe {
		LoadTextureFromImage(LoadImageFromMemory(
			c".png".as_ptr(),
			data.as_ptr(),
			data.len() as i32,
		))
	}
}

pub fn my_draw_texture(
	texture: Texture2D,
	at: Vector2,
	rotation: f32,
	scale: f32,
	flip_hori: bool,
	flip_vert: bool,
) {
	let mut source = Rectangle {
		x: 0.0,
		y: 0.0,
		w: texture.width as f32,
		h: texture.height as f32,
	};

	let target = Rectangle {
		x: at.x,
		y: at.y,
		w: source.w * scale,
		h: source.h * scale,
	};

	let origin = Vector2 {
		x: target.w / 2.0,
		y: target.h / 2.0,
	};

	if flip_hori {
		source.w *= -1.0;
	}

	if flip_vert {
		source.h *= -1.0;
	}

	unsafe { DrawTexturePro(texture, source, target, origin, rotation, WHITE) };
}
