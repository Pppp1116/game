use game::soldiers::Soldier;
use game::traits::Draw;
use raylib::consts::MouseButton;
use raylib::prelude::Vector2;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse = rl.get_mouse_position();

            Soldier::new(mouse);
        }
        let mut d = rl.begin_drawing(&thread);

        let position = Vector2::new(10.0, 12.0);

        Soldier::new(position).draw(&mut d);
    }
}
