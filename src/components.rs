use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

// #[derive(Component)] is a macro that says "from my basic data, please derive the boilerplate needed for x";
// in this case, the x is a Component.
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}