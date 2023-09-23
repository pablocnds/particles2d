use crate::config::*;
pub use crate::gui::*;
use macroquad::{miniquad::window::screen_size, prelude::*};

pub type FnSim<T> = fn(&mut Vec<T>, &mut Gui);

pub trait DrawableParticle {
    fn get_pos(&self) -> &Vec2;
    fn get_size(&self) -> f32;
    fn get_color(&self) -> &Color;
    /// Should provide a non-changing unique ID
    fn get_id(&self) -> u64;
}

/// Simple example of a particle struct implementing DrawableParticle.
pub struct BasicParticle {
    pub pos: Vec2,
    pub sz: f32,
    pub color: Color,
    pub id: u64,
}
impl DrawableParticle for BasicParticle {
    fn get_pos(&self) -> &Vec2 {
        &self.pos
    }
    fn get_size(&self) -> f32 {
        self.sz
    }
    fn get_color(&self) -> &Color {
        &self.color
    }
    fn get_id(&self) -> u64 {
        self.id
    }
}
impl BasicParticle {
    pub fn new(pos: Vec2, sz: f32, color: Color, id: u64) -> Self {
        Self { pos, sz, color, id }
    }
}

pub struct Camera {
    screen_scale: f32,
    screen_sz: Vec2,
    offset: Vec2,
    zoom: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            screen_scale: screen_width() / 100.,
            screen_sz: Vec2::from(screen_size()),
            offset: Vec2 { x: 0., y: 0. },
            zoom: 1.,
        }
    }
}
impl Camera {
    pub(crate) fn draw_world_particle<T: DrawableParticle>(&self, particle: &T) {
        let gx_pos = self.coord_world_to_px(particle.get_pos());
        let gx_sz = self.dist_world_to_px(particle.get_size());
        draw_circle(gx_pos.x, gx_pos.y, gx_sz, *particle.get_color());
    }
    pub fn coord_world_to_px(&self, world_coord: &Vec2) -> Vec2 {
        (*world_coord - self.offset) * self.zoom * self.screen_scale + (self.screen_sz / 2.)
    }
    pub fn dist_world_to_px(&self, world_distance: f32) -> f32 {
        world_distance * self.zoom * self.screen_scale
    }
}

/// Entry point which will run the main loop. 
/// Takes a setup function and an update function and runs everything.
pub fn run<T>(particle_setup: FnSim<T>, update: FnSim<T>)
where
    T: DrawableParticle + 'static,
{
    macroquad::Window::new("Particle Visualizer", run_ac(particle_setup, update));
}

async fn run_ac<T: DrawableParticle>(usetup: FnSim<T>, uupdate: FnSim<T>) {
    let mut cam = Camera::default();
    let mut particles = Vec::new();
    let mut gui = Gui::new(GlobalText::default());
    usetup(&mut particles, &mut gui);
    loop {
        if !gui.paused || gui.expect_frame_consume() {
            uupdate(&mut particles, &mut gui);
        }
        update_camera_(&mut cam, &particles, &mut gui);
        // TODO see if I can customize framerate
        // let frame_time_milis = get_frame_time() * 1000.0;
        // let time_to_sleep_milis = config::FRAME_TIME_MILIS - frame_time_milis;
        // if time_to_sleep_milis > 0.0 {
        //     std::thread::sleep(std::time::Duration::from_millis(time_to_sleep_milis as u64));
        // }
        next_frame().await
    }
}

fn update_camera_<T>(camera: &mut Camera, particles: &[T], gui: &mut Gui)
where
    T: DrawableParticle,
{
    let frame_time = get_frame_time();
    clear_background(BG_COLOR);

    // TODO try to loop through keys down instead of checking all of them
    if is_key_down(KeyCode::D) {
        camera.offset.x += PAN_VEL * (1. / camera.zoom) * frame_time;
    }
    if is_key_down(KeyCode::A) {
        camera.offset.x -= PAN_VEL * (1. / camera.zoom) * frame_time;
    }
    if is_key_down(KeyCode::W) {
        camera.offset.y -= PAN_VEL * (1. / camera.zoom) * frame_time;
    }
    if is_key_down(KeyCode::S) {
        camera.offset.y += PAN_VEL * (1. / camera.zoom) * frame_time;
    }
    if is_key_down(KeyCode::Z) && camera.zoom < MAX_ZOOM {
        camera.zoom *= 1. + (ZOOM_CHG_VEL * frame_time);
    }
    if is_key_down(KeyCode::X) && camera.zoom > MIN_ZOOM {
        camera.zoom *= 1. - (ZOOM_CHG_VEL * frame_time);
    }
    // TODO does not belong here
    if is_key_pressed(KeyCode::Space) {
        gui.switch_pause();
    }
    if is_key_pressed(KeyCode::Tab) {
        gui.request_next_frame();
    }

    for particle in particles.iter() {
        camera.draw_world_particle(particle);
    }

    gui.draw(particles, camera);
}
