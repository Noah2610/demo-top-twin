use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::{World, WorldExt};
use amethyst::ui::{FontAsset, FontHandle, TtfFormat};
use deathframe::amethyst;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

pub struct FontHandles<K>
where
    K: Eq + Hash,
{
    handles: HashMap<K, FontHandle>,
}

impl<K> FontHandles<K>
where
    K: Eq + Hash,
{
    pub fn load<P>(&mut self, name: P, world: &mut World)
    where
        P: AsRef<Path> + Into<K> + Clone,
    {
        let key = name.clone().into();
        let path = name.as_ref();

        let loader = world.read_resource::<Loader>();
        let font_handle = loader.load(
            path.to_str().unwrap(),
            TtfFormat,
            (),
            &world.read_resource::<AssetStorage<FontAsset>>(),
        );

        self.handles.insert(key, font_handle);
    }

    pub fn get(&self, name: &K) -> Option<&FontHandle> {
        self.handles.get(name)
    }
}

impl<K> Default for FontHandles<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
        }
    }
}
