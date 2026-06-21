use bracket_lib::color::RGB;
use bracket_lib::prelude::FontCharType;
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
pub struct Player;
