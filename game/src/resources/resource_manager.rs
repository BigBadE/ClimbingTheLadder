use std::collections::HashMap;
use std::any::TypeId;
use std::sync::Arc;
use anyhow::Error;
use json::object::Object;
use tokio::task::JoinSet;
use crate::mods::mod_manager::ModManager;
use crate::mods::mods::GameMod;
use crate::rendering::assets::AssetReference;
use crate::rendering::renderer::Renderer;
use crate::util::alloc_handle::AllocHandle;

//An arc mutex of functions to return the created type.
//It's a mouthful, but the best solution I can think of given the circumstance.
pub type TypeInstantiator = HashMap<String,
    fn(&mut ResourceManager, &Object) -> Result<Result<(TypeId, Box<dyn NamedType>), String>, Error>>;

pub struct ResourceManager {
    //Instantiators
    pub(crate) instantiators: TypeInstantiator,
    //Map of types and named types of that type
    pub(crate) types: HashMap<TypeId, Vec<usize>>,
    //Map of types to their name
    pub(crate) named_types: HashMap<String, usize>,
    pub(crate) all_types: Vec<Arc<AllocHandle>>,
    #[cfg(feature = "renderer")]
    pub asset_manager: Box<dyn AssetReference>,
    pub renderer: Arc<dyn Renderer>,
    pub _mods: ModManager,
}

impl ResourceManager {
    pub fn new(mods: JoinSet<Result<GameMod, Error>>,
               #[cfg(feature = "renderer")]asset_manager: Box<dyn AssetReference>,
               #[cfg(feature = "renderer")]renderer: Arc<dyn Renderer>) -> Self {
        return ResourceManager {
            instantiators: HashMap::new(),
            types: HashMap::new(),
            named_types: HashMap::new(),
            all_types: Vec::new(),
            #[cfg(feature = "renderer")]
            asset_manager,
            #[cfg(feature = "renderer")]
            renderer,
            _mods: ModManager::new(mods),
        };
    }

    pub fn get_type<T>(&self, name: &str) -> Option<&T> where T: NamedType + 'static {
        return match self.named_types.get(name) {
            Some(value) => Some(self.all_types[*value].read()),
            None => None
        };
    }

    pub fn get_all_of_type<T>(&self) -> Vec<&T> where T: 'static {
        let mut output = Vec::new();
        for value in self.types.get(&TypeId::of::<T>()).unwrap() {
            output.push(self.all_types[*value].read());
        }
        return output;
    }
}

pub trait NamedType: Send {
    fn name(&self) -> String;
}