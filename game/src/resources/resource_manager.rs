use std::collections::HashMap;
use std::fs;
use std::any::TypeId;
use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Error;
use json::JsonValue;
use json::object::Object;
use tokio::runtime::Handle;
use tokio::task::{JoinError, JoinSet};
use crate::{ContentPack, error, TaskManager};
use crate::rendering::assets::AssetReference;
use crate::resources::resource_loader::ResourceLoader;
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
    pub asset_manager: Box<dyn AssetReference>
}

impl ResourceManager {
    pub fn new(
        #[cfg(feature = "renderer")]asset_manager: Box<dyn AssetReference>) -> Self {
        return ResourceManager {
            instantiators: HashMap::new(),
            types: HashMap::new(),
            named_types: HashMap::new(),
            all_types: Vec::new(),
            #[cfg(feature = "renderer")]
            asset_manager
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

    pub fn load_all(reference: &Arc<Mutex<Self>>, task_manager: &mut TaskManager,
                    loading: &Box<dyn ContentPack>) -> Arc<Mutex<ResourceLoader>> {
        let resource_loader = Arc::new(Mutex::new(
            ResourceLoader::new(reference.clone())));
        for json in loading.types() {
            let loader = task_manager.get_runtime(true).spawn(Self::load_json(json));
            task_manager.queue(false,
                Self::load_types(loader, resource_loader.clone(),
                                 task_manager.get_runtime(false).clone()));
        }
        return resource_loader;
    }

    async fn load_json(path: PathBuf) -> Result<JsonValue, Error> {
        return Ok(json::parse(String::from_utf8(fs::read(path)?)?.as_str())?);
    }

    async fn load_types(loading: impl Future<Output=Result<Result<JsonValue, Error>, JoinError>>,
                        loader: Arc<Mutex<ResourceLoader>>, runtime: Handle) -> AllocHandle {
        let found = match loading.await {
            Ok(value) => match value {
                Ok(value) => value,
                Err(error) => {
                    error!("Error loading JSON: {}", error);
                    return AllocHandle::empty();
                }
            },
            Err(error) => {
                error!("Error joining thread: {}", error);
                return AllocHandle::empty();
            }
        };

        let mut join_set = JoinSet::new();
        for found in Self::get_types(found) {
            join_set.spawn_on(ResourceLoader::spawn(loader.clone(), found), &runtime);
        }

        while let Some(value) = join_set.join_next().await {
            match value {
                Ok(result) => match result {
                    Ok((id, named_type)) => loader.lock().unwrap().finish(id, named_type),
                    Err(error) => error!("Error loading JSON resource:\n{}", error)
                }
                Err(error) => error!("Error joining resource loading thread:\n{}", error)
            }
        }

        return AllocHandle::empty();
    }

    fn get_types(found: JsonValue) -> Vec<Object> {
        let mut types = Vec::new();
        match found {
            JsonValue::Array(values) => {
                for value in values {
                    match value {
                        JsonValue::Object(object) => types.push(object),
                        _ => {
                            error!("Tried to load JSON with unknown top level object: {}", value);
                            continue;
                        }
                    }
                }
            }
            JsonValue::Object(object) => types.push(object),
            _ => {
                error!("Tried to load JSON with unknown top level object: {}", found);
                return Vec::new();
            }
        }
        return types;
    }
}

pub trait NamedType: Send {
    fn name(&self) -> String;
}