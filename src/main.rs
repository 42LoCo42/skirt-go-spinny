mod bindings;
use bindings::*;

fn main() {
	unsafe {
		InitWindow(800, 600, c"Skirt go Spinny :3".as_ptr());
		SetTargetFPS(60);

		let skirt_data = include_bytes!("skirt.png");
		let skirt_img = LoadImageFromMemory(
			c".png".as_ptr(),
			skirt_data.as_ptr(),
			skirt_data.len() as i32,
		);
		let skirt = LoadTextureFromImage(skirt_img);

		let skirt_rect = Rectangle {
			x: 0.0,
			y: 0.0,
			w: skirt.width as f32,
			h: skirt.height as f32,
		};

		let mut skirt_pos = Rectangle {
			x: 0.0,
			y: 0.0,
			w: skirt.width as f32,
			h: skirt.height as f32,
		};

		let skirt_cntr = Vector2 {
			x: skirt.width as f32 / 2.0,
			y: skirt.height as f32 / 2.0,
		};

		let mut angle: f32 = 0.0;

		while !WindowShouldClose() {
			BeginDrawing();

			skirt_pos.x = GetScreenWidth() as f32 / 2.0;
			skirt_pos.y = GetScreenHeight() as f32 / 2.0;

			ClearBackground(Color {
				r: 0x28,
				g: 0x28,
				b: 0x28,
				a: 0xff,
			});

			DrawTexturePro(
				skirt.clone(),
				skirt_rect.clone(),
				skirt_pos.clone(),
				skirt_cntr.clone(),
				angle,
				Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
					a: 0xff,
				},
			);

			EndDrawing();

			angle += 5.0;
		}
	};
}
