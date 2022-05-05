use specs::{Component, VecStorage, NullStorage};
use std::fmt::{Formatter, Display};
use std::fmt;
use std::collections::HashMap;


#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub position: Position,
    pub resource_template_path: &'static str,
    pub resource_template_data: HashMap<String, String>
}

impl Renderable {
    pub fn from(position: Position, resource_template_path: &'static str, resource_template_data: HashMap<String, String>) -> Self {
        Renderable { position, resource_template_path, resource_template_data }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

impl Movable {
    pub fn new() -> Self {
        Movable {}
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Blocking;

impl Blocking {
    pub fn new() -> Self {
        Blocking {}
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Directional {
    pub direction: Direction
}

impl Directional {
    pub fn from(direction: Direction) -> Self {
        Directional { direction }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Box;

impl Box {
    pub fn new() -> Self {
        Box {}
    }
}

#[derive(Copy, Clone)]
pub enum BoxSpotColor {
    Black,
}

impl Display for BoxSpotColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Black => write!(f, "black"),
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Wall;

impl Wall {
    pub fn new() -> Self {
        Wall {}
    }
}

#[derive(Copy, Clone)]
pub enum WallType {
    Hee,
    Hew,
    Hin,
    His,
    Hw,
    Urc,
    Ulc,
    Brc,
    Blc,
    Ven,
    Ves,
    Vie,
    Viw,
    Vw,
}

impl Display for WallType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Hee => write!(f, "horizontal-end-east"),
            Self::Hew => write!(f, "horizontal-end-west"),
            Self::Hin => write!(f, "horizontal-intersect-north"),
            Self::His => write!(f, "horizontal-intersect-south"),
            Self::Hw => write!(f, "horizontal-wall"),
            Self::Urc => write!(f, "upper-right-corner"),
            Self::Ulc => write!(f, "upper-left-corner"),
            Self::Brc => write!(f, "bottom-right-corner"),
            Self::Blc => write!(f, "bottom-left-corner"),
            Self::Ven => write!(f, "vertical-end-north"),
            Self::Ves => write!(f, "vertical-end-south"),
            Self::Vie => write!(f, "vertical-intersect-east"),
            Self::Viw => write!(f, "vertical-intersect-west"),
            Self::Vw => write!(f, "vertical-wall"),
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Spot;

impl Spot {
    pub fn new() -> Self {
        Spot {}
    }
}

#[derive(Copy, Clone)]
pub enum FloorMaterial {
    Grass,
}

impl Display for FloorMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Grass => write!(f, "grass"),
        }
    }
}
