use bevy::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct CalculatedHealth(usize);

#[derive(Debug, Component, Reflect)]
pub struct Health {
    current: usize,
    max: usize,
}

impl Health {
    pub fn new(max: usize) -> Self {
        Self { max, current: max }
    }

    pub fn suffer(&mut self, damage: usize) -> usize {
        let actual_damage = self.current.min(damage);

        self.current = self.current.saturating_sub(damage);

        actual_damage
    }

    pub fn heal(&mut self, heal: usize) -> usize {
        let new_current = self.current.saturating_add(heal);
        let actual_heal = new_current - self.current;

        self.current = new_current;
        actual_heal
    }
}

/* This is fired whenever an entity is healed */
#[derive(Debug, Event, Reflect)]
pub struct Healed {
    pub entity: Entity,
}
