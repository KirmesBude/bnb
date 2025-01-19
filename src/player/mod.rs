use bevy::prelude::*;

pub mod action;

/*
    Now:
    Player Hand,
    Discard Pile,
    Lost Pile,
    Later:
    Player Active Area
    Items?
*/

/* Cards are going to be entities */
/* So it make only sense to make all these areas entities as well, that I parent them to? */
/* The idea is to handle the layout of the entities under the parent entity here */
/* But how to handle movement and stuff? */

#[derive(Debug, Component, Reflect)]
#[require(Transform, Visibility)]
pub struct Hand;

#[derive(Debug, Component, Reflect)]
#[require(Transform, Visibility)]
pub struct DiscardPile;

#[derive(Debug, Component, Reflect)]
#[require(Transform, Visibility)]
pub struct LostPile;
