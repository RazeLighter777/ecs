use crate::hashing;
use crate::typeable;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::hash::BuildHasher;
use std::hash::Hasher;
pub trait ComponentDataType: Serialize + DeserializeOwned {}
pub const fn get_type_id<DataType: 'static + ComponentDataType>() -> u64 {
    hashing::string_hash(std::any::type_name::<DataType>())
}
pub trait ComponentInterface: typeable::Typeable + typeable::Identifiable {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl<T: typeable::Typeable + typeable::Identifiable + 'static> ComponentInterface for T {
    /// Returns an any trait reference
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    /// Returns an any mutable trait reference
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

/// A struct that represents an stateful aspect of an entity.
pub struct Component<DataType>
where
    DataType: ComponentDataType,
{
    /// A unique component instance id for the component
    id: u64,
    /// A shared type id for the component generated from a hash of the name of the data type at compilation time.
    tid: u64,
    /// A struct implementing the ComponentDataType trait that can be serialized and deserialized at will.
    pub data: DataType,
}

impl<DataType: 'static> Component<DataType>
where
    DataType: ComponentDataType,
{
    /// Constructor for new component that takes in a id and the component's data information and returns an object.
    /// # Arguments
    /// * `id` the unique instance id of the component to be created.
    /// * `data` a ComponentDataType struct that the new component will be created from.
    pub fn new_with_id(id: u64, data: DataType) -> Self {
        Self {
            id: id,
            tid: hashing::string_hash(std::any::type_name::<DataType>()),
            data: data,
        }
    }
    /// Generates a new component with a random instance id from a DataType struct.
    /// # Arguments
    /// * `data` a ComponentDataType struct that the new component will be created from.
    pub fn new(data: DataType) -> Self {
        Self::new_with_id(
            std::collections::hash_map::RandomState::new()
                .build_hasher()
                .finish(),
            data,
        )
    }
}

impl<T> typeable::Identifiable for Component<T>
where
    T: ComponentDataType,
{
    /// Returns the unique instance id of the component
    fn get_id(&self) -> u64 {
        self.id
    }
}
impl<T> typeable::Typeable for Component<T>
where
    T: ComponentDataType,
{
    /// Returns the type id of the component.
    fn get_type_id(&self) -> u64 {
        self.tid
    }
}
