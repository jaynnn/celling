use bevy::prelude::*;

pub struct LoadingImage {
    is_loaded: bool,
    bin_data: Vec<u8>,
    handle: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct LoadingImageMap {
    map: HashMap<String, LoadingImage>
}

impl LoadingImageMap {
    fn exists(&self, path: &String) -> bool {
        self.map.contains_key(path)
    }

    fn create(&mut self, path: String, handle: Handle<Image>) {
        self.map.insert(path, LoadingImage {
            is_loaded: false,
            bin_data: Vec::new(),
            handle: handle,
        });
    }

    fn get(&self, path: String) -> Option<&LoadingImage> {
        self.map.get(&path)
    }
}