use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::prelude::*;
use rand::seq::IteratorRandom;
use marching_squares::{Field as MarchingSquaresField, march, simplify};
use earcutr;

use crate::comm::{Po, PoCreate, PoDir, PIXEL_SIZE, PIXEL_SIZE_F, PIXEL_SIZE_HALF_F, get_fix_pos};
use crate::components::RigidCheckField;
use crate::res::CellsMap;
use crate::components::*;

fn evaluate_velocity(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn cell_trans_rigid(cmds: &mut Commands, e: Entity, dir: PoDir) {
    if let Some(v) = match dir {
        PoDir::Left => {
            Some(Velocity::linear(Vec2 {x: -100., y: 100.}))
        }
        PoDir::Right => {
            Some(Velocity::linear(Vec2 {x: 100., y: 100.}))
        }
        _ => {
            None
        }
    } {
        cmds.entity(e).remove::<Cell>().insert((
            RigidBody::Dynamic,
            Collider::cuboid(PIXEL_SIZE_HALF_F, PIXEL_SIZE_HALF_F),
            v
        ));
    }
}

pub fn handle(
    mut query: Query<(&Transform, &mut RigidCheckField, &Velocity)>,
    map: Res<CellsMap>,
    mut cmds: Commands,
) {
    for (t, mut r, v) in query.iter_mut() {
        let ev = evaluate_velocity(v.linvel.x as f64, v.linvel.y as f64);
        if ev > 600. {
            // TODO: 并行化和减少遍历个数;
            let x = t.translation.x as i32;
            let y = t.translation.y as i32;
            r.set_xy(x, y);
            // info!("set {} {}", t.translation.x, t.translation.y);
            let mut rng = rand::thread_rng();
            for p in r.into_iter().choose_multiple(&mut rng, 10) {
                if let Some(e) = map.get(&p) {
                    // println!("{}", p);
                    let dir = p.calc_dir_lr(&Po {x: x, y: y});
                    cell_trans_rigid(&mut cmds, *e, dir);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RigidField {
    dimensions: (usize, usize),
    field: HashMap<Po, f64>,
}

impl RigidField {
    fn new(w: usize, h: usize) -> Self {
        Self {
            dimensions: (w, h),
            field: HashMap::new(),
        }
    }

    fn set_field(&mut self, key: Po, value: f64) {
        self.field.insert(key, value);
    }
}
impl MarchingSquaresField for RigidField {
    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn z_at(&self, x: usize, y: usize) -> f64 {
        if let Some(v) = self.field.get(&Po {x: x as i32, y: y as i32}) {
            *v
        } else {
            0.
        }
    }
}

#[derive(Event)]
pub struct RigidizeEvent {
    w: i32,
    h: i32,
    meterial_value: f32,
}

impl RigidizeEvent {
    pub fn new(w: i32, h: i32, meterial_value: f32) -> Self {
        Self {
            w, h, meterial_value
        }
    }
}

pub fn rigidize(
    query: Query<(&Transform, &RigidMeterial)>,
    mut event: EventReader<RigidizeEvent>,
    mut gizmos: Gizmos,
    mut cmds: Commands,
) {
    // let w = &1920;
    // let h = &1980;
    // let meterial_value = &1.;
    for RigidizeEvent {w, h, meterial_value} in event.read() {
        let w = *w;
        let h = *h;
        let w_half = w / 2;
        let h_half = h / 2;
        let mut rigid_field = RigidField::new(w as usize, h as usize);

        for (t, c) in query.iter() {
            let x = get_fix_pos(t.translation.x as i32) / PIXEL_SIZE;
            let y = get_fix_pos(t.translation.y as i32) / PIXEL_SIZE;
            gizmos.cuboid(Transform::from_xyz(x as f32, y as f32, 0.), Color::GREEN);
            if *meterial_value == c.0 && x.abs() < w_half && y.abs() < h_half {
                rigid_field.set_field(Po {x: x + w_half, y: y + h_half}, c.0 as f64);
            }
        }
        
        let contours: Vec<Vec<(f64, f64)>> = march(&rigid_field, 0.5);
        for c in contours {
            let v = simplify::simplify_with_eps(&c, 10.);
            let mut verticles = Vec::new();
            for (x, y) in &v {
                verticles.push(*x as f32);
                verticles.push(*y as f32);
            } 
    
            let result = earcutr::earcut(&verticles, &[], 2).unwrap();
            let mut coords = Vec::new();
            let mut indices = Vec::new();
            for (i, t) in result.chunks(3).enumerate() {
                coords.push(Vect::new((verticles[t[0]*2] as f32 - w_half as f32) * PIXEL_SIZE_F, (verticles[t[0]*2+1] as f32 - h_half as f32)*PIXEL_SIZE_F));
                coords.push(Vect::new((verticles[t[1]*2] as f32 - w_half as f32) * PIXEL_SIZE_F, (verticles[t[1]*2+1] as f32 - h_half as f32)*PIXEL_SIZE_F));
                coords.push(Vect::new((verticles[t[2]*2] as f32 - w_half as f32) * PIXEL_SIZE_F, (verticles[t[2]*2+1] as f32 - h_half as f32)*PIXEL_SIZE_F));
                indices.push([(i*3) as u32, (i*3+1) as u32, (i*3+2) as u32])
            }

            if !coords.is_empty() {
                cmds.spawn((
                    RigidBody::Dynamic,
                    Collider::trimesh(coords, indices),
                ));
            }
        }
    }
}