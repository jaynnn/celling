use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::comm::*;

#[derive(Resource, Eq, PartialEq, Clone)]
pub struct CellsMap {
    map: HashMap<Po, Entity>,
}

impl Default for CellsMap {
    fn default() -> Self {
        Self {
            map : HashMap::new()
        }
    }
}

impl Iterator for CellsMap {
    type Item = (Po, Entity);
    fn next(&mut self) -> Option<Self::Item> {
        let next_entry = self.map.iter().next().map(|(key, &value)| (key.clone(), value));
        if let Some((key, _)) = &next_entry {
            self.map.remove(key);
        }
        next_entry
    }
}

impl CellsMap {
    pub fn add(&mut self, p: &Po, e: &Entity) -> Option<Entity> {
        self.map.insert(*p, *e)
    }

    pub fn del(&mut self, p: &Po) -> Option<Entity> {
        self.map.remove(p)
    }

    pub fn get(&self, p: &Po) -> Option<&Entity> {
        self.map.get(p)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn get_neighbors<T: Sized>(&self, neihbor_po: [Po; 8]) -> Vec<Option<&Entity>> {
        neihbor_po.iter().map(|p| self.get(p)).collect()
    }

    pub fn show_debug_info(&self, cmds: &mut Commands, p: &Po) {
        let e = self.get(p);
        if let Some(e1) = e {
            cmds.spawn((Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!("{:?}", e1),
                        TextStyle {
                            color: Color::RED,
                            font_size: (PIXEL_SIZE*3) as f32,
                            ..default()
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (p.x + 5*PIXEL_SIZE) as f32, 
                    p.y as f32,
                    0.
                ),
                ..default()
            }, DebugMask));
        } else {
            println!("[ALARM] p={:?}->e is None", p);
        }
    }

    pub fn show_debug_info_all(&self, cmds: &mut Commands) {
        for (p, _e) in self.map.iter() {
            cmds.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat((PIXEL_SIZE+2) as f32)),
                        color: Color::rgba_u8(228, 76, 76, 50),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (p.x-1) as f32,
                        (p.y-1) as f32,
                        0.5,
                    ),
                    ..default()
                },
                DebugMask
            ));
        }
    }
}

#[derive(Component)]
pub struct DebugMask;