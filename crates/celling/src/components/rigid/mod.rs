use bevy::prelude::*;

use crate::comm::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct RigidCheckField {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub cur_x: i32,
    pub cur_y: i32
}

#[derive(Component)]
pub struct RigidMeterial(pub f32);

impl RigidCheckField {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            w, 
            h,
            cur_x: 0,
            cur_y: 0,
        }
    }

    pub fn set_xy(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

}

impl Iterator for RigidCheckField {
    type Item = Po;
    fn next(&mut self) -> Option<Self::Item> {
        let x = get_fix_pos(self.x - self.w / 2);
        let y = get_fix_pos(self.y - self.h / 2);
        let cur_x = x + self.cur_x;
        let cur_y = y + self.cur_y;
        // info!("===== {} {} {} {} {} {}", self.x, self.y, x, y, cur_x, cur_y);
        if cur_y <= get_fix_pos(y + self.h) {
            self.cur_x += PIXEL_SIZE;
            if x + self.cur_x >= get_fix_pos(x + self.w) {
                self.cur_x = 0;
                self.cur_y += PIXEL_SIZE;
            }
            let p = Po {x: cur_x, y: cur_y};
            Some(p)
        } else {
            self.cur_x = 0;
            self.cur_y = 0;
            None
        }
    }
}