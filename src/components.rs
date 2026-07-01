use bracket_lib::color::RGB;
use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::Component;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Chasing {
    pub target: Option<Point>,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Monster;

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component, Debug)]
pub struct TileBlocking;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
pub struct DamageDealer {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct DamageTaker {
    pub amount: Vec<i32>,
}

impl DamageTaker {
    pub fn take_damage(storage: &mut WriteStorage<DamageTaker>, target: Entity, amount: i32) {
        if let Some(damage_taker) = storage.get_mut(target) {
            damage_taker.amount.push(amount);
        } else {
            storage
                .insert(
                    target,
                    DamageTaker {
                        amount: vec![amount],
                    },
                )
                .expect("Unable to insert damage");
        }
    }
}
