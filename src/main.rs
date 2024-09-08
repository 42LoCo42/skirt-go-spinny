mod bindings;
use bindings::*;

mod util;
use util::*;

fn main() {
	unsafe {
		InitWindow(800, 600, c"Skirt go Spinny :3".as_ptr());
		SetTargetFPS(60);

		let skirt = load_texture(include_bytes!("skirt.png"));
		let blahaj = load_texture(include_bytes!("blahaj.png"));

		let mut angle: f32 = 0.0;

		while !WindowShouldClose() {
			BeginDrawing();

			ClearBackground(Color {
				r: 0x28,
				g: 0x28,
				b: 0x28,
				a: 0xff,
			});

			my_draw_texture(
				skirt,
				Vector2 {
					x: GetScreenWidth() as f32 / 2.0,
					y: GetScreenHeight() as f32 / 2.0,
				},
				angle,
				0.75,
				false,
				false,
			);

			my_draw_texture(
				blahaj,
				Vector2 {
					x: GetScreenWidth() as f32 * 0.2,
					y: GetScreenHeight() as f32 / 2.0,
				},
				90.0,
				0.8,
				false,
				true,
			);

			my_draw_texture(
				blahaj,
				Vector2 {
					x: GetScreenWidth() as f32 * 0.8,
					y: GetScreenHeight() as f32 / 2.0,
				},
				90.0,
				0.8,
				false,
				false,
			);

			EndDrawing();

			angle += 5.0;
		}
	};
}

// xMOunB2>%cwO+{@zF%M7e[XQrfj]cvff0J2v(b#1vMlg#gCyN[iB$0Rh-JVMi59oCwhlb5
// vMunNh8O(Qf/9w$h!czOvMuq2i5@#Sw[tNgh8eVMh7-=UvqvDlA=kPhvqP<}zF7OpAaZp2
// :3
