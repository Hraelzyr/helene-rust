use serde::{Deserialize, Serialize};
use crate::damage_types;
use damage_types::Damage;
use crate::damage_types::{DamageType, Resistances};
use crate::factions::Faction;
use std::fs;

pub(crate) trait Vulnerable{
    fn lower_hp(&mut self, damage: Damage) -> bool; //is dead?
    fn raise_hp(&mut self, heal: u16) -> (); //heals are type-less; no affinities
}

pub(crate) trait Movable{
    fn set_loc(&mut self, x: u8, y: u8) -> ();
    fn mov(&mut self, dx: i8, dy: i8) -> ();
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Actor {
    mhp: u16,
    hp: i16,
    x: u8,
    y: u8,
    res: Resistances,
    fac: Faction,
    atk: Vec<Damage>,
}

impl Actor{
    pub fn new_player() -> Self{
        ron::from_str::<Actor>(fs::read_to_string("src/config/player.ron").
            expect("Player config file absent").as_str()).unwrap()
    }
    pub fn get_loc(&self) -> (u8, u8){
        (self.x, self.y)
    }
}

impl Vulnerable for Actor {
    fn lower_hp(self: &mut Actor, damage: Damage) -> bool {
        match damage._type {
            DamageType::Physical => {
                self.hp-=(32767-self.res.phys_res)*damage.value as i16/32767;
            }
            DamageType::Fire => {
                self.hp-=(32767-self.res.fire_res)*damage.value as i16/32767;
            }
        }
        self.hp<=0
    }

    fn raise_hp(self: &mut Actor, heal: u16) -> () {
        self.hp+=heal as i16;
        if self.hp as u16>=self.mhp {
            self.hp=self.mhp as i16;
        }
    }
}

impl Movable for Actor {
    fn set_loc(&mut self, x: u8, y: u8) -> () {
        self.x=x;
        self.y=y;
    }

    fn mov(&mut self, dx: i8, dy: i8) -> () {
        self.x=(self.x as i8+dx) as u8;
        self.y=(self.y as i8+dy) as u8;
    }
}