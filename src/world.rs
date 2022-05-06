use crate::component;
use crate::component::Component;
use crate::entity;
use crate::typeable;
use crate::typeable::Typeable;
use entity::Entity;
use std::boxed::Box;
use std::collections::HashMap;
use std::ops::RangeBounds;
use typeable::Identifiable;
pub struct World {
    pub entities: HashMap<u64, Entity>,
    pub components: Vec<Box<dyn component::ComponentInterface>>,
}
impl World {
    /// Takes an entity instance id, and component data, and returns a component id if the entity exists
    ///
    /// # Arguments
    /// * `component_data` - The data of the new component.
    /// * `iid` - The instance id of the entity this is added to.
    pub fn add_component_to_entity_by_entity_id<
        DataType: component::ComponentDataType + 'static,
    >(
        &mut self,
        component_data: DataType,
        iid: u64,
    ) -> Option<u64> {
        let constructed_component = Component::new(component_data);
        match self.entities.get_mut(&iid) {
            Some(entity) => {
                entity.components.insert(
                    constructed_component.get_type_id(),
                    constructed_component.get_id(),
                );
                let ret_val = constructed_component.get_id();
                self.components.push(Box::new(constructed_component));
                Some(ret_val);
            }
            None => {
                return None;
            }
        }
        None
    }
    /// Adds the component to the world by creating a new entity. Returns the instance id of the new entity.
    /// # Arguments
    /// * `component_data` - The data of the new component.
    pub fn add_component<DataType: component::ComponentDataType + 'static>(
        &mut self,
        component_data: DataType,
    ) -> u64 {
        let mut new_entity = entity::Entity::new();
        let constructed_component = Component::new(component_data);
        new_entity.components.insert(
            constructed_component.get_type_id(),
            constructed_component.get_id(),
        );
        let ret_val = new_entity.get_id();
        self.entities.insert(new_entity.get_id(), new_entity);
        self.components.push(Box::new(constructed_component));
        ret_val
    }

    /// Takes an entity , and component data, and returns a component id if the entity exists
    ///
    /// # Arguments
    /// * `component_data` - The data of the new component.
    /// * `entity` - The instance reference of the entity this is added to. Entity must exist in the world.
    pub fn add_component_to_entity_by_entity<DataType: component::ComponentDataType + 'static>(
        &mut self,
        component_data: DataType,
        entity: &entity::Entity,
    ) -> Option<u64> {
        self.add_component_to_entity_by_entity_id(component_data, entity.get_id())
    }
    /// Creates a new empty entity with a random id and returns the instance id
    pub fn create_new_entity_in_world(&mut self) -> u64 {
        let entity = entity::Entity::new();
        let result = entity.get_id();
        self.entities.insert(result, entity);
        result
    }
    /// Creates a new empty entity with a specified id.
    /// # Arguments
    /// * `iid` - the instance id of the new entity
    pub fn create_new_entity_in_world_by_id(&mut self, iid: u64) {
        let entity = entity::Entity::new_with_id(iid);
        let result = entity.get_id();
        self.entities.insert(result, entity);
    }
    /// Returns a reference to a component based on a component id
    /// # Arguments
    /// * `cid` - The unique component instance id you are trying to retrieve
    pub fn get_component_by_component_id<ComponentType: component::ComponentDataType + 'static>(
        &self,
        cid: u64,
    ) -> Option<&component::Component<ComponentType>> {
        for c in &self.components {
            if (*c).get_id() == cid {
                return (*c)
                    .as_any()
                    .downcast_ref::<component::Component<ComponentType>>();
            }
        }
        None
    }
    /// Takes in an entity and tries to get a component of given type from it.
    /// # Arguments
    /// * `e` the entity to extract a component of the given type from.
    pub fn get_component_by_entity<ComponentType: component::ComponentDataType + 'static>(
        &self,
        e: &Entity,
    ) -> Option<&component::Component<ComponentType>> {
        let tp = component::get_type_id::<ComponentType>();
        match e.components.get(&tp) {
            Some(id) => Self::get_component_by_component_id(self, *id),
            None => None,
        }
    }
    /// Gets a component of a specific type from an entity by the entity's id as read only
    /// # Arguments
    /// * `iid` - The unique instance id of the entity you are retrieving the specific type of component from.
    pub fn get_component_by_entity_id<ComponentType: component::ComponentDataType + 'static>(
        &self,
        iid: u64,
    ) -> Option<&component::Component<ComponentType>> {
        match self.entities.get(&iid) {
            Some(entity) => self.get_component_by_entity(entity),
            None => None,
        }
    }
    
    /// Gets a component of a specific type from an entity by the entity's id as mutable
    /// # Arguments
    /// * `iid` - The unique instance id of the entity you are retrieving the specific type of component from.
    pub fn get_component_by_entity_id_mut<ComponentType: component::ComponentDataType + 'static>(
        &mut self,
        iid: u64,
    ) -> Option<&mut component::Component<ComponentType>> {
        let mut component_id = 0;
        match self.entities.get_mut(&iid) {
            Some(entity) => {
                let t = component::get_type_id::<ComponentType>();
                match entity.components.get(&t) {
                    Some(cid) => {
                        component_id = *cid;
                    }
                    None => {
                        return None;
                    }
                };
            }
            None => {
                return None;
            }
        };
        self.get_component_by_component_id_mut(component_id)
    }
    /// Returns true if the entity with the given ID exists. Please prefer using another method if you need to retrieve the entity too due to performance reasons. 
    /// # Arguments 
    /// * `iid` - The instance id of the entity you are checking existence for 
    pub fn entity_with_id_exists(&self, iid: u64) -> bool {
        self.entities.contains_key(&iid)
    }
    
    /// Gets a component reference of a specific type from an entity by the entity's read only reference.
    /// # Arguments
    /// * `e` - The entity you are retreiving from.
    pub fn get_component_by_entity_mut<ComponentType: component::ComponentDataType + 'static>(
        &mut self,
        e: &Entity,
    ) -> Option<&mut component::Component<ComponentType>> {
        let tp = component::get_type_id::<ComponentType>();
        match e.components.get(&tp) {
            Some(id) => Self::get_component_by_component_id_mut(self, *id),
            None => None,
        }
    }

    /// Gets a mutable component reference of a specific type from an entity by the entity's reference.
    /// # Arguments
    /// * `iid` - The instance id of the entity you are retreiving from.
    pub fn get_component_by_component_id_mut<
        ComponentType: component::ComponentDataType + 'static,
    >(
        &mut self,
        iid: u64,
    ) -> Option<&mut component::Component<ComponentType>> {
        for c in &mut self.components {
            if (*c).get_id() == iid {
                return (*c)
                    .as_any_mut()
                    .downcast_mut::<component::Component<ComponentType>>();
            }
        }
        None
    }
}
