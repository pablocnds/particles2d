use particles2d::*;
use macroquad::prelude::*;

fn setup(particles: &mut Vec<BasicParticle>, gui: &mut Gui) {
    particles.push(BasicParticle::new(Vec2::new(0., 0.), 1., RED, 0));
    particles.push(BasicParticle::new(Vec2::new(0., 0.), 1., BLUE, 1));

    gui.create_minitext(&["x", "y"], 0, particles);
    gui.create_minitext(&["x", "y"], 1, particles);

    gui.glob_text.create_field("Run time", &get_time().to_string());
}

fn update(particles: &mut Vec<BasicParticle>, gui: &mut Gui) {
    particles[0].pos = Vec2::new(
        get_time().sin() as f32 * 10.0,
        get_time().cos() as f32 * 30.0,
    );
    particles[1].pos = Vec2::new(
        -get_time().sin() as f32 * 10.0,
        -get_time().cos() as f32 * 30.0,
    );

    gui.get_minitext_mut(0).unwrap().update_field("x", &particles[0].get_pos().x.to_string());
    gui.get_minitext_mut(0).unwrap().update_field("y", &particles[0].get_pos().y.to_string());
    gui.get_minitext_mut(1).unwrap().update_field("x", &particles[1].get_pos().x.to_string());
    gui.get_minitext_mut(1).unwrap().update_field("y", &particles[1].get_pos().y.to_string());

    gui.glob_text.update_field("Run time", &get_time().to_string());
}

fn main() {
    run(setup, update);
}
