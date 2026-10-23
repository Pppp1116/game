use game::hitbox::HitBox;
use game::soldiers::Soldier;
use game::world::World;
use raylib::consts::MouseButton;
use raylib::drawing::RaylibDraw;
use raylib::ffi::{Color, Vector2};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let height = rl.get_screen_height();
    let width = rl.get_screen_width();
    let mut world = World::new(width, height);

    let soldier_size = Vector2::new(20.0, 20.0);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse = rl.get_mouse_position();

            let hitbox = HitBox::new(mouse, soldier_size);
            let soldier = Soldier::new(hitbox);

            world.push(soldier);
        }
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        world.update(dt, &mut d);
    }
}
