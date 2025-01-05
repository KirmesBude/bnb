use bevy::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct CalculatedHealth(usize);
