use crate::typeable;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hasher;
/// Represents a list of components assosiated with an object.
pub struct Entity {
    /// A globally unique instance id for the entity.
    pub id: u64,
    /// hash relating component type id to component instance id.
    pub components: HashMap<u64, u64>,
}

impl typeable::Identifiable for Entity {
    /// Returns the global unique instance id for the entity.
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl Entity {
    /// Creates a new entity with no components and a random instance id and returns it
    pub fn new() -> Self {
        Self::new_with_id(
            std::collections::hash_map::RandomState::new()
                .build_hasher()
                .finish(),
        )
    }
    /// Creates a new entity with no components and an explicit instance id and returns it
    /// #Arguments
    /// * `iid` - The instance id of the new entity.
    pub fn new_with_id(iid: u64) -> Self {
        Entity {
            id: iid,
            components: HashMap::new(),
        }
    }
}
