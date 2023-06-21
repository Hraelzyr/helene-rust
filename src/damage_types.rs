use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum DamageType{
    Physical,
    Fire
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Damage{
    pub(crate) _type: DamageType,
    pub(crate) value: u16
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Resistances{
    pub(crate) phys_res: i16,
    pub(crate) fire_res: i16,
}