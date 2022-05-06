#![allow(warnings)]
#![feature(const_type_name)]
mod component;
mod entity;
mod hashing;
mod typeable;
mod world;
use component::{ComponentDataType, ComponentInterface};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashMap;
use typeable::Identifiable;
#[derive(Serialize, Deserialize, Debug)]
struct Pos {
    x: u16,
    y: u16,
}

impl ComponentDataType for Pos {}

fn main() {
    let mut w = world::World {
        entities: HashMap::new(),
        components: Vec::new(),
    };
    let mut z = 0;
    for x in 0..10000 {
        z = w.add_component(Pos { x: 3, y: x });
        let mut cmp = w.get_component_by_entity_id_mut::<Pos>(z);
        match cmp {
            Some(entity) => {
                entity.data.x = 4;
                println!(
                    "The deserialized component is {}",
                    serde_json::to_string(&entity.data).unwrap()
                );
            }
            None => {}
        }
    }

    return;
}
