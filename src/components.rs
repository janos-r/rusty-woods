pub mod door;
pub mod player;
pub mod sign;
pub mod ui;
pub use {door::*, player::*, sign::*, ui::*};

use bevy::prelude::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
