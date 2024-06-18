pub mod camera;

use bevy::prelude::*;

pub type Po = IVec2;
pub const PIXEL_SIZE: i32 = 8;
pub const PIXEL_SIZE_HALF: i32 = PIXEL_SIZE / 2;

pub const PIXEL_SIZE_F: f32 = PIXEL_SIZE as f32;
pub const PIXEL_SIZE_HALF_F: f32 = PIXEL_SIZE_F / 2.;

const NEIGHBOR_TOP_LEFT: Po = Po::new(-1*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_TOP: Po = Po::new(0*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_TOP_RIGHT: Po = Po::new(1*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_LEFT: Po = Po::new(-1*PIXEL_SIZE, 0*PIXEL_SIZE);
const NEIGHBOR_RIGHT: Po = Po::new(1*PIXEL_SIZE, 0*PIXEL_SIZE);
const NEIGHBOR_BOTTOM_LEFT: Po = Po::new(-1*PIXEL_SIZE, -1*PIXEL_SIZE);
const NEIGHBOR_BOTTOM: Po = Po::new(0*PIXEL_SIZE, -1*PIXEL_SIZE);
const NEIGHBOR_BOTTOM_RIGHT: Po = Po::new(1*PIXEL_SIZE, -1*PIXEL_SIZE);

const NEIGHBOR_COUNT: usize = 8;

const NEIGHBOR_COORDINATES: [Po; NEIGHBOR_COUNT] = [
    NEIGHBOR_TOP_LEFT, NEIGHBOR_TOP, NEIGHBOR_TOP_RIGHT,
    NEIGHBOR_LEFT,                   NEIGHBOR_RIGHT,
    NEIGHBOR_BOTTOM_LEFT,NEIGHBOR_BOTTOM,NEIGHBOR_BOTTOM_RIGHT
];

pub trait NeighborGetter {
    fn get_neighbor(&self, p: Po) -> Po;
    fn get_neighbors(&self) -> Vec<Self>
        where Self: Sized;
}

impl NeighborGetter for Po {
    fn get_neighbor(&self, p: Self) -> Self {
        *self + p
    }

    fn get_neighbors(&self) -> Vec<Self> {
        NEIGHBOR_COORDINATES.iter().map(|p| self.get_neighbor(*p)).collect()
    }
}

pub fn get_fix_pos(n: i32) -> i32 {
    n - n % PIXEL_SIZE
}

pub fn get_cell_create_pos(x: i32, y: i32) ->(i32, i32) {
    (get_fix_pos(x), get_fix_pos(y))
}
pub trait PoCreate {
    fn create(x: i32, y: i32) -> Self;
    fn calc_dir_lr(self, to: &Po) -> PoDir;
}

pub enum PoDir {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
    None,
}

impl PoCreate for Po {
    fn create(x: i32, y: i32) -> Self {
        Self {
            x: get_fix_pos(x),
            y: get_fix_pos(y)
        }
    }
    // 计算左右方向
    fn calc_dir_lr(self, to: &Po) -> PoDir {
        if self.x < to.x {
            PoDir::Right
        } else if self.x > to.x {
            PoDir::Left
        } else {
            PoDir::None
        }
    }
}



