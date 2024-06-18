use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::utils::HashMap;
use crate::{comm::*, CellsMap, components::*, systems::rigids::RigidizeEvent};

fn u8_array_to_i32(bytes: [u8; 4]) -> i32 {
    (((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | ((bytes[3] as u32))) as i32
}

fn i32_to_u8_array(n: i32) -> [u8; 4] {
    [
        ((n >> 24) & 0xFF) as u8,
        ((n >> 16) & 0xFF) as u8,
        ((n >> 8 ) & 0xFF) as u8,
        ((n      ) & 0xFF) as u8,
    ]
}

pub fn spawn_image_sprite_handle(
    mut cmds: Commands,
    mut spawn_events: EventReader<SpawnImageSpriteEvent>,
    mut loading_map: Local<LoadingImageMap>,
    mut loading_queue: Local<VecDeque<String>>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut map: ResMut<CellsMap>,
    mut rigid_events: EventWriter<RigidizeEvent>,
) {
    for ev in spawn_events.read() {
        let pos = ev.pos;
        let path = &ev.path;
        if let Some(loading_image) = loading_map.get_mut(path.to_string()) {
            if loading_image.is_loaded() {
                do_spawn_image_sprite(&mut cmds, &loading_image.bin_data, &mut map, loading_image.pos);
                rigid_events.send(RigidizeEvent::new(1920, 1080, 1.));
            }
        } else {
            info!("spawn_image_sprite_handle {:?}", path);
            let handle = asset_server.load(path);
            loading_map.create(path.clone(), pos, handle);
            loading_queue.push_back(path.clone());
        }
    }
    
    let mut index = 0;
    while index < loading_queue.len() {
        let path = &loading_queue[index];
        if let Some(loading_image) = loading_map.get_mut(path.to_string()) {
            if asset_server.load_state(loading_image.handle.clone()) == LoadState::Loaded {
                let img = images.get(loading_image.handle.clone()).unwrap();
                let pixels = &img.data;
                let mut bin_data = Vec::new();
                for (i, pixel) in pixels.chunks(4).enumerate() {
                    let alpha = pixel[3];
                    let w = img.width() as i32;
                    let h = img.height() as i32;
                    let x = i as i32 % w - w/2;
                    let y = -(i as i32 / w) + h/2;
                    if alpha != 0 {
                        for n in i32_to_u8_array(x) {
                            bin_data.push(n)
                        }
                        for n in i32_to_u8_array(y) {
                            bin_data.push(n)
                        }
                        for n in pixel {
                            bin_data.push(*n)
                        }
                    }
                }
                do_spawn_image_sprite(&mut cmds, &bin_data, &mut map, loading_image.pos);
                rigid_events.send(RigidizeEvent::new(1920, 1080, 1.));
                loading_image.set_loaded(bin_data);
                loading_queue.remove(index);
            }
        }
        index += 1;
    }
}


fn do_spawn_image_sprite(
    mut cmds: &mut Commands,
    data: &Vec<u8>,
    map: &mut ResMut<CellsMap>,
    pos: Po,
) {
    info!("do_spawn_image_sprite");
    for (_i, p) in data.chunks(12).enumerate() {
        // info!("======= {} {:?}", i, p);
        let x = u8_array_to_i32([p[0], p[1], p[2], p[3]]);
        let y = u8_array_to_i32([p[4], p[5], p[6], p[7]]);
        let color = Color::rgba_u8(p[8], p[9], p[10], p[11]);
        // TODO 读配置加载
        let cell_bundle = CellBundle {
            c: Cell::Sand,
            d: Density(1),
            cd: CellDir::None,
        };
        if let Some(e) = create_cell(&mut cmds, map, cell_bundle, pos.x + x as i32 * PIXEL_SIZE, pos.y + y as i32 * PIXEL_SIZE, color) {
            cmds.entity(e).insert(RigidMeterial(1.));
        }
    }
}

#[derive(Event, Debug)]
pub struct SpawnImageSpriteEvent {
    pos: Po, 
    path: String,
}

impl SpawnImageSpriteEvent {
    pub fn new(pos: Po, path: String) -> Self {
        Self {
            pos, path
        }
    }
}

pub struct LoadingImage {
    is_loaded: bool,
    bin_data: Vec<u8>,
    handle: Handle<Image>,
    pos: Po,
}

impl LoadingImage {
    fn set_loaded(&mut self, bin_data: Vec<u8>) {
        self.is_loaded = true;
        self.bin_data = bin_data;
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

#[derive(Default, Resource)]
pub struct LoadingImageMap {
    map: HashMap<String, LoadingImage>
}

impl LoadingImageMap {
    fn _exists(&self, path: &String) -> bool {
        self.map.contains_key(path)
    }

    fn create(&mut self, path: String, pos: Po, handle: Handle<Image>) {
        self.map.insert(path, LoadingImage {
            is_loaded: false,
            bin_data: Vec::new(),
            handle: handle,
            pos: pos,
        });
    }

    fn _get(&self, path: String) -> Option<&LoadingImage> {
        self.map.get(&path)
    }

    fn get_mut(&mut self, path: String) -> Option<&mut LoadingImage> {
        self.map.get_mut(&path)
    }
}