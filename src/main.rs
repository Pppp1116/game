use game::hitbox::HitBox;
use game::soldiers::Soldier;
use game::world::World;
use raylib::consts::MouseButton;
use raylib::drawing::RaylibDraw;
use raylib::ffi::Color;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let height = rl.get_screen_height();
    let width = rl.get_screen_width();
    let mut world = World::new(width, height);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse = rl.get_mouse_position();

            let hitbox = HitBox::new(mouse);
            let soldier = Soldier::new(hitbox);

            world.push(soldier);
        }
        //check bullet collision
        world.check_bullet_colision();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        world.draw(&mut d);
    }
}
