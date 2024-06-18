use bevy::prelude::*;
use rand::prelude::SliceRandom;
use crate::comm::*;
use crate::prelude::CellsMap;

#[derive(Component, Eq, PartialEq, Copy, Clone, Default, Debug)]
pub enum Cell {
    #[default]
    Sand,
    Liquip,
    Gas,
    Stable,
}

#[derive(Component, Default, Clone, Copy)]
pub enum CellType {
    #[default]
    Sand,
    Liquip,
    Gas,
    Stable,
}

#[derive(Component, Debug)]
pub struct PoInfo {
    pub e: Entity,
    // 当前帧位置
    pub cp: Po,
    // 上一帧位置
    pub lp: Po,
}

impl PoInfo {
    pub fn new(e: Entity, cp: Po, lp: Po) -> Self {
        Self {
            e, cp, lp
        }
    }
}

#[derive(Component, Default)]
pub struct CellVelocity(pub f32, pub f32);

// 静止
#[derive(Component)]
pub struct Silent;

#[derive(Component, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Density(pub i32);

#[derive(Component, Default, Clone, Copy)]
// 方向 0=无 1=左 2=右
pub enum CellDir {
    #[default]
    None,
    Left,
    Right
}
const LIQUIP_CELL_DIR_VEC: [(CellDir, u32); 3] = [(CellDir::None, 1), (CellDir::Left, 10), (CellDir::Right, 10)];
impl CellDir {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        LIQUIP_CELL_DIR_VEC.choose_weighted(&mut rng, |item| item.1).unwrap().0
    }

    pub fn new2(v: [Self; 2]) -> Self {
        let mut rng = rand::thread_rng();
        *v.choose(&mut rng).unwrap()
    }

    pub fn new3(v: [Self; 3]) -> Self {
        let mut rng = rand::thread_rng();
        *v.choose(&mut rng).unwrap()
    }

    pub fn calc_dir(from: &Po, to: &Po) -> Self {
        if from.x < to.x {
            Self::Right
        } else if from.x > to.x {
            Self::Left
        } else {
            Self::None
        }
    }
}

#[derive(Bundle, Default, Clone)]
pub struct CellBundle {
    pub c: Cell,
    pub d: Density,
    pub cd: CellDir,
}

pub struct CellBundleQuery {
    pub t: Transform,
    pub cb: CellBundle
}

impl CellBundleQuery {
    pub fn new(t: Transform, c: Cell, d: Density, cd: CellDir) -> Self {
        CellBundleQuery {
            cb: CellBundle {
                c: c,
                d: d,
                cd: cd
            },
            t: t
        }
    }
}

pub fn create_cell(cmds: &mut Commands, map: &mut CellsMap, bd: CellBundle, x: i32, y: i32, color: Color) -> Option<Entity> {
    let (x, y) = get_cell_create_pos(x, y);
    let p = Po {x, y};
    // if map.get(&p).is_some() {
    //     return None;
    // }
    let ecmd = cmds.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(PIXEL_SIZE as f32)),
                    color: color,
                    ..default()
                },
                transform: Transform::from_xyz(
                    x as f32,
                    y as f32,
                    1.,
                ),
                ..default()
            },
            bd,
            CellVelocity(0., 0.),
        )
    );
    let e = ecmd.id();
    // ecmd.insert(PoInfo::new(e, p, p));
    map.add(&p, &e);
    // println!("create_cell po={}", &p);
    Some(e)
}
