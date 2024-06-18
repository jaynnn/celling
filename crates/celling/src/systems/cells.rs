// use bevy::prelude::*;

// 函数耗时
// use std::time::Instant;
// let now = Instant::now();
// let elapsed_time = now.elapsed();
// println!("Running get_next_po_liquid() took {}", elapsed_time.as_nanos());

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::components::*;
use crate::prelude::{Density, CellDir, PoInfo, Silent};
use crate::res::*;
use crate::comm::*;

const NEIGHBOR_TOP_LEFT: Po = Po::new(-1*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_TOP: Po = Po::new(0*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_TOP_RIGHT: Po = Po::new(1*PIXEL_SIZE, 1*PIXEL_SIZE);
const NEIGHBOR_LEFT: Po = Po::new(-1*PIXEL_SIZE, 0*PIXEL_SIZE);
const NEIGHBOR_RIGHT: Po = Po::new(1*PIXEL_SIZE, 0*PIXEL_SIZE);
const NEIGHBOR_BOTTOM_LEFT: Po = Po::new(-1*PIXEL_SIZE, -1*PIXEL_SIZE);
const NEIGHBOR_BOTTOM: Po = Po::new(0*PIXEL_SIZE, -1*PIXEL_SIZE);
const NEIGHBOR_BOTTOM_RIGHT: Po = Po::new(1*PIXEL_SIZE, -1*PIXEL_SIZE);


// 拆分handle与handle_update_map的原因是：
// 在handle中将逻辑计算并行化，在handle_update_map中统一修改位置信息
// 注意：这意味着同一帧中一个坐标属于一个像素
use bevy::core::FrameCount;
pub fn handle(
    query: Query<Entity, (With<Cell>, Without<Silent>)>,
    query2: Query<(&Transform, &Cell, &Density, &CellDir)>,
    cells_map: Res<CellsMap>,
    par_commands: ParallelCommands,
    _count: Res<FrameCount>,
)
{
    query.par_iter().for_each(|e| {
        if let Ok((t, c, d, cd)) = query2.get(e) {
            let old_p = Po {x: t.translation.x as i32, y: t.translation.y as i32};
            if let (Some(new_p), new_cd_op) = match c {
                Cell::Sand => {
                    (get_next_po_sand(&old_p, &c, d, cd, &cells_map), None)
                }
                Cell::Liquip => {
                    let next_p = get_next_po_liquid(&old_p, &c, d, cd, &cells_map, &query2);
                    if let Some(new_p) = next_p {
                        (next_p, Some(CellDir::calc_dir(&old_p, &new_p)))
                    } else {
                        (next_p, None)
                    }
                    
                }
                Cell::Gas => {
                    let next_p: Option<IVec2> = get_next_po_gas(&old_p, &c, d, cd, &cells_map, &query2);
                    if let Some(new_p) = next_p {
                        (next_p, Some(CellDir::calc_dir(&old_p, &new_p)))
                    } else {
                        (None, None)
                    }
                }
                _ => (None, None)
            } {
                par_commands.command_scope(|mut cmds| {
                    // println!("{}--- handle cell update {:?}_{:?}:{}->{}", count.0, c, e, old_p, new_p);
                    let mut ecmds = cmds.entity(e);
                    
                    if let Some(new_cd) = new_cd_op {
                        ecmds.insert(new_cd).insert(PoInfo::new(e, new_p, old_p));
                    } else {
                        ecmds.insert(PoInfo::new(e, new_p, old_p));
                    }
                });
            }
        } else {
            println!("[ALARM] handle cell update e={:?} not exists.", e);
        }
    });
}

pub fn handle_update_map(
    mut cmds: Commands,
    query: Query<&PoInfo, With<Cell>>,
    mut cells_map: ResMut<CellsMap>,
) {
    let mut updated_map = HashMap::<Po, bool>::new();
    for po_info in query.iter() {
        // info!("{:?}", po_info);
        let cp = po_info.cp;
        let lp = po_info.lp;
        let e = po_info.e;
        // println!("--- handle_update_map curpos={},lastpos={},entity={:?}", cp, lp, e);
        // 如果上一个位置是自己，则要删除，不是自己则说明被占用
        if cp != lp {
            if cells_map.get(&lp) == Some(&e) {
                cells_map.del(&lp);
            }
        }
        if !updated_map.contains_key(&cp) {
            cells_map.add(&cp, &e);
            cmds.entity(e).insert(Transform::from_xyz(
                cp.x as f32,
                cp.y as f32,
                0.,
            )).remove::<PoInfo>();
            updated_map.insert(cp, true);
        }
    }
}

pub fn handle_debug(
    // mut cmds: Commands,
    // cells_map: ResMut<CellsMap>,
    // query: Query<Entity, With<DebugMask>>,
) {
    // for e in query.iter() {
    //     cmds.entity(e).despawn();
    // }
    // cells_map.show_debug_info_all(&mut cmds);
}

fn get_next_po_sand(
    p: &Po, _c: &Cell, _d: &Density, _cd: &CellDir, map: &CellsMap
) -> Option<Po> {
    let c = p.get_neighbor(NEIGHBOR_BOTTOM);
    if map.get(&c) == None {
        return Some(c)
    }
    let c = p.get_neighbor(NEIGHBOR_BOTTOM_LEFT);
    if map.get(&c) == None {
        return Some(c)
    } 
    let c = p.get_neighbor(NEIGHBOR_BOTTOM_RIGHT);
    if map.get(&c) == None {
        return Some(c)
    }
    None
}

fn get_next_po_liquid(
    p: &Po, 
    _c: &Cell, 
    d: &Density, 
    cd: &CellDir, 
    map: &CellsMap, 
    query2: &Query<(&Transform, &Cell, &Density, &CellDir)>
) -> Option<Po> {
    let c = p.get_neighbor(NEIGHBOR_BOTTOM);
    if let Some(bottom_e) = map.get(&c) {
        let nc = query2.component::<Cell>(*bottom_e);
        let nd = query2.component::<Density>(*bottom_e);
        if *nc == Cell::Liquip && d > nd {
            return Some(c)
        }
    } else {
        return Some(c)
    }
    let c = p.get_neighbor(NEIGHBOR_TOP);
    if let Some(neighbor_e) = map.get(&c) {
        let nd = query2.component::<Density>(*neighbor_e);
        if d < nd {
            return Some(c)
        }
    }
    let c = p.get_neighbor(NEIGHBOR_BOTTOM_LEFT);
    if map.get(&c) == None {
        return Some(c)
    } 
    let c = p.get_neighbor(NEIGHBOR_BOTTOM_RIGHT);
    if map.get(&c) == None {
        return Some(c)
    }
    let c1 = p.get_neighbor(NEIGHBOR_LEFT);
    let c2 = p.get_neighbor(NEIGHBOR_RIGHT);
    match (map.get(&c1), map.get(&c2)) {
        (Some(_), Some(_)) => {
            return None
        }
        (Some(ne1), None) => {
            if query2.component::<Cell>(*ne1) == &Cell::Liquip {
                match CellDir::new2([CellDir::None, CellDir::Right]) {
                    CellDir::Right => {
                        return Some(c2)
                    }
                    _ => {
                        return None
                    }
                }
            }
        }
        (None, Some(ne2)) => {
            if query2.component::<Cell>(*ne2) == &Cell::Liquip {
                match CellDir::new2([CellDir::None, CellDir::Left]) {
                    CellDir::Left => {
                        return Some(c1)
                    }
                    _ => {
                        return None
                    }
                }
            }
        }
        (None, None) => {
            match cd {
                CellDir::Left => {
                    return Some(c1)
                }
                CellDir::Right => {
                    return Some(c2)
                }
                _ => {
                    return None
                }
            }
        }
    }
    None
}

fn get_next_po_gas(
    p: &Po, 
    _c: &Cell, 
    d: &Density, 
    cd: &CellDir, 
    map: &CellsMap,
    query2: &Query<(&Transform, &Cell, &Density, &CellDir)>
) -> Option<Po> {
    let c = p.get_neighbor(NEIGHBOR_TOP);
    if let Some(bottom_e) = map.get(&c) {
        let nc = query2.component::<Cell>(*bottom_e);
        let nd = query2.component::<Density>(*bottom_e);
        if *nc == Cell::Liquip && d > nd {
            return Some(c)
        }
    } else {
        return Some(c)
    }
    let c = p.get_neighbor(NEIGHBOR_BOTTOM);
    if let Some(neighbor_e) = map.get(&c) {
        let nd = query2.component::<Density>(*neighbor_e);
        if d < nd {
            return Some(c)
        }
    }
    let c = p.get_neighbor(NEIGHBOR_TOP_LEFT);
    if map.get(&c) == None {
        return Some(c)
    } 
    let c = p.get_neighbor(NEIGHBOR_TOP_RIGHT);
    if map.get(&c) == None {
        return Some(c)
    }
    let c1 = p.get_neighbor(NEIGHBOR_LEFT);
    let c2 = p.get_neighbor(NEIGHBOR_RIGHT);
    match (map.get(&c1), map.get(&c2)) {
        (Some(_), Some(_)) => {
            return None
        }
        (Some(ne1), None) => {
            if query2.component::<Cell>(*ne1) == &Cell::Gas {
                return Some(c2)
            }
        }
        (None, Some(ne2)) => {
            if query2.component::<Cell>(*ne2) == &Cell::Gas {
                return Some(c1)
            }
        }
        (None, None) => {
            match cd {
                CellDir::Left => {
                    return Some(c1)
                }
                CellDir::Right => {
                    return Some(c2)
                }
                _ => {
                    return None
                }
            }
        }
    }
    None
}