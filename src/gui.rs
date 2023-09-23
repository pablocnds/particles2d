use crate::{config::*, visualizer::DrawableParticle};
use macroquad::prelude::*;

fn draw_text_box(pos: &Vec2, txt: &Vec<String>, font_sz: u16, max_width: f32, box_color: &Color) {
    if let Some(l) = txt.first() {
        let l_dim = measure_text(l, None, font_sz, 1.0);
        let l_height = l_dim.height + l_dim.offset_y;
        draw_rectangle(
            pos.x,
            pos.y,
            max_width + 10.0,
            l_height * txt.len() as f32 + 10.0,
            *box_color,
        );

        for (i, line) in txt.iter().enumerate() {
            let h = l_height * (i + 1) as f32;
            draw_text(
                line.trim_end(),
                10.0 + pos.x,
                h + pos.y,
                font_sz as f32,
                BLACK,
            );
        }
    }
}

pub struct GlobalText {
    pub fields: Vec<(String, String)>,
    pub max_width: f32,
    pub font_sz: u16,
}
impl Default for GlobalText {
    fn default() -> Self {
        Self {
            fields: vec![],
            max_width: DEF_GUI_TXT_MAX_WIDTH,
            font_sz: DEF_GUI_FONT_SZ,
        }
    }
}
impl GlobalText {
    pub fn new(fields: &[String]) -> Self {
        let fields = fields.iter().map(|x| (x.clone(), String::new())).collect();
        Self {
            fields,
            ..Default::default()
        }
    }
    pub fn create_field(&mut self, field: &str, value: &str) {
        if self.fields.iter_mut().any(|x| x.0 == field) {
            // TODO logger
            println!("ERROR! create_field requested for already existing field.");
        }
        else {
            self.fields.push((field.to_string(), value.to_string()));
        }
    }
    pub fn update_field(&mut self, field: &str, value: &str) {
        if let Some(field) = self.fields.iter_mut().find(|x| x.0 == field) {
            field.1.replace_range(.., value);
        } else {
            // TODO add logger or/and error reporting system
            println!("ERROR! update_field requested for a non-existing field.")
        }
    }
    pub(crate) fn draw(&self) {
        let mut txt = Vec::new();
        for (field, val) in self.fields.iter() {
            // Reduce the size of the text until it fits the size limit
            let line = format!("{}: {}", field, val);
            let mut l = &line[..];
            let mut line_dim;
            loop {
                line_dim = measure_text(l, None, self.font_sz, 1.0);
                if line_dim.width < self.max_width {
                    break;
                }
                l = &line[..l.len() - 2];
            }
            txt.push(l.to_owned());
        }

        draw_text_box(
            &Vec2::new(0.0, 0.0),
            &txt,
            self.font_sz,
            self.max_width,
            &DEF_GUI_TXT_RECT_COLOR,
        );
    }
}

/// Text with information about a tracked particle that will be displayed in
/// the gui alongside the particle.
///
/// If the tracked particle is deleted, this text will disapear automatically.
pub struct MiniText {
    pub fields: Vec<(String, String)>,
    pub tracked_uid: u64,
    pub tracked_index: usize,
    pub disabled: bool,
}
impl MiniText {
    /// Tries to create a new MiniText tracking the particle with the given uid. Will
    /// return None if the particle doesn't exist.
    pub fn try_new<T: DrawableParticle>(
        fields: &[&str],
        particle_uid: u64,
        particles: &[T],
    ) -> Option<Self> {
        let fields = fields.iter().map(|x| (x.to_string(), String::new())).collect();
        // If the given id does not exist in the vector, it returns None
        let tracked_index = particles.iter().position(|x| x.get_id() == particle_uid)?;
        Some(Self {
            fields,
            tracked_uid: particle_uid,
            tracked_index,
            disabled: false,
        })
    }
    pub fn get_font_sz() -> u16 {
        GUI_MINITEXT_FONT_SZ
    }
    pub fn update_field(&mut self, field: &str, new_value: &str) {
        if let Some(field) = self.fields.iter_mut().find(|x| x.0 == field) {
            field.1.replace_range(.., new_value);
        } else {
            // TODO add logger or/and error reporting system
            println!("ERROR! update_field requested for a non-existing field.");
        }
    }
    /// Will only fail if the tracked particle is removed from the vector. Will return
    /// true if successful, false if failed.
    ///
    /// Will panic if particle is disabled, which happens once after drawing fails.
    ///
    /// This structure should be removed if this fails or else it will panic.
    pub(crate) fn try_draw<T: DrawableParticle>(
        &mut self,
        particles: &[T],
        camera: &crate::visualizer::Camera,
    ) -> bool {
        // ! Will panic if instance is disabled. This is to enforce memory leak safety.
        if self.disabled {
            panic!("Draw method has been called on disabled MiniText instance! This instance should have been removed.");
        }
        // Check if the position of the particle has moved and search for it
        let particle = particles.get(self.tracked_index);
        match particle {
            Some(p) => {
                self.draw_at(
                    &camera.coord_world_to_px(p.get_pos()),
                    camera.dist_world_to_px(p.get_size()),
                );
            }
            None => {
                if let Some(i) = particles
                    .iter()
                    .position(|x| self.tracked_uid == x.get_id())
                {
                    self.tracked_index = i;
                    let p = &particles[i];
                    self.draw_at(
                        &camera.coord_world_to_px(p.get_pos()),
                        camera.dist_world_to_px(p.get_size()),
                    );
                } else {
                    // If the tracked particle was removed, disable self.
                    self.disabled = true;
                    return false;
                }
            }
        }

        true
    }
    /// Coordinate and radius are expected in pixel values
    fn draw_at(&self, particle_pos: &Vec2, particle_rad: f32) {
        let mut txt = Vec::new();
        for (field, val) in self.fields.iter() {
            txt.push(format!("{field}: {val}"));
        }
        let pos = *particle_pos + Vec2::new(particle_rad, particle_rad);
        // TODO DEF_GUI... is wrong
        draw_text_box(
            &pos,
            &txt,
            Self::get_font_sz(),
            GUI_MINITEXT_MAX_WIDTH,
            &DEF_GUI_TXT_RECT_COLOR,
        );
    }
}

pub struct Gui {
    pub glob_text: GlobalText,
    pub mini_text: Vec<MiniText>,
    pub paused: bool,
    expect_frame: bool,
}
impl Gui {
    pub fn new(global_text: GlobalText) -> Self {
        Gui {
            glob_text: global_text,
            mini_text: vec![],
            paused: false,
            expect_frame: false,
        }
    }

    // TODO removal logic requires more testing
    pub(crate) fn draw<T: DrawableParticle>(
        &mut self,
        particles: &[T],
        camera: &crate::visualizer::Camera,
    ) {
        // 1. DRAW GLOBAL TEXT
        self.glob_text.draw();

        // 2. DRAW MINI TEXTs
        let mut to_remove = false;
        for mt in self.mini_text.iter_mut() {
            let res = mt.try_draw(particles, camera);
            if !res {
                to_remove = true;
                //TODO remove print
                println!("TRY_DRAW FAILED BECAUSE A PARTICLE WAS REMOVED.");
            }
        }
        // Remove disabled MiniText instances with swap_remove
        if to_remove {
            let mut i = 0;
            while i > self.mini_text.len() {
                if self.mini_text[i].disabled {
                    self.mini_text.swap_remove(i);
                    // TODO test and remove print
                    println!("MINITEXT REMOVED BECAUSE IT WAS DISABLED.");
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Pauses the update logic if running, or unpauses it if already in pause.
    /// Will not pause the camera logic, so moving/zooming is still possible.
    pub(crate) fn switch_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Sets the expectation for a next frame if pause mode is active. This
    /// expectation can then be checked and consumed with expect_frame().
    pub(crate) fn request_next_frame(&mut self) {
        if self.paused {
            self.expect_frame = true;
        }
    }

    /// Returns whether or not a new frame has been requested during pause mode
    /// with request_next_frame(). This method will consume this expectation.
    pub(crate) fn expect_frame_consume(&mut self) -> bool {
        let expectation = self.expect_frame;
        self.expect_frame = false;
        expectation
    }

    /// Get the minitext associated to the particle with the given uid. 
    /// Will return None if the given uid doesn't exist or doesn't have a minitext.
    pub fn get_minitext_mut(&mut self, uid: u64) -> Option<&mut MiniText> {
        self.mini_text.iter_mut().find(|x| x.tracked_uid == uid)
    }

    /// Fails and return None if uid already has a minitext.
    ///
    /// If successful, will add an empty minitext owned by the gui and return its mut reference.
    pub fn create_minitext<T: DrawableParticle>(
        &mut self,
        fields: &[&str],
        particle_uid: u64,
        particles: &[T],
    ) -> Option<&mut MiniText> {
        if self.get_minitext_mut(particle_uid).is_some() {
            // TODO REMOVE PRINTLN
            println!("ATTEMPTED TO CREATE A MINITEXT FOR ALREADY TRACKED PARTICLE");
            return None;
        }
        self.mini_text
            .push(MiniText::try_new(fields, particle_uid, particles)?);
        self.get_minitext_mut(particle_uid)
    }

    pub fn remove_minitext(&mut self, _uid: u64) {
        todo!()
    }
}
