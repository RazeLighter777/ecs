pub trait Identifiable {
    fn get_id(&self) -> u64;
}
pub trait Typeable {
    fn get_type_id(&self) -> u64;
}
