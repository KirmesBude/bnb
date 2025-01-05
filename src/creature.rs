use bevy::prelude::*;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Creature>()
            .register_type::<Health>()
            .register_type::<Conditions>()
            .register_type::<Retaliate>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Creature {
    health: Health,
    conditions: Conditions,
    shield: usize,
    retaliate: Retaliate,
}

/* TODO: Computed shield and retaliate based on active bonuses I guess */

#[derive(Debug, Default, Component, Reflect)]
pub struct Retaliate {
    damage: usize,
    range: usize,
}

#[derive(Debug, Component, Reflect)]
pub struct Health {
    current: usize,
    max: usize,
}

impl Health {
    pub fn new(health: usize) -> Self {
        Self {
            current: health,
            max: health,
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn suffer_damage(&mut self, damage: usize) {
        self.current = self.current.saturating_sub(damage);
    }

    pub fn heal(&mut self, heal: usize) {
        self.current = (self.current + heal).min(self.max);
    }
}

impl Creature {
    pub fn new(health: usize) -> Self {
        Self {
            health: Health::new(health),
            conditions: Conditions::default(),
            shield: 0,
            retaliate: Retaliate::default(),
        }
    }

    pub fn health(&self) -> &Health {
        &self.health
    }

    pub fn conditions(&self) -> &Conditions {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: ConditionKind) {
        self.conditions.add_condition(condition);
    }

    pub fn remove_condition(&mut self, condition: ConditionKind) {
        self.conditions.remove_condition(condition);
    }

    pub fn suffer_damage(&mut self, damage: usize) {
        self.health.suffer_damage(damage);
    }

    pub fn heal(&mut self, heal: usize) {
        if !self.conditions.poison {
            self.health.heal(heal);
        }
        self.remove_condition(ConditionKind::Wound);
        self.remove_condition(ConditionKind::Poison);
    }

    pub fn calculate_attack_damage(&self, attack: &Attack) -> usize {
        let poison = if self.conditions.poison { 1 } else { 0 };

        (attack.damage + poison).saturating_sub(self.shield.saturating_sub(attack.pierce))
    }
}

#[derive(Default, Debug, Reflect)]
pub struct Conditions {
    invisible: bool,
    strengthen: bool,
    wound: bool,
    poison: bool,
    immobilize: bool,
    disarm: bool,
    muddle: bool,
}

impl Conditions {
    pub fn add_condition(&mut self, condition: ConditionKind) {
        match condition {
            ConditionKind::Invisible => self.invisible = true,
            ConditionKind::Strengthen => self.strengthen = true,
            ConditionKind::Wound => self.wound = true,
            ConditionKind::Poison => self.poison = true,
            ConditionKind::Immobilize => self.immobilize = true,
            ConditionKind::Disarm => self.disarm = true,
            ConditionKind::Muddle => self.muddle = true,
        }
    }

    pub fn remove_condition(&mut self, condition: ConditionKind) {
        match condition {
            ConditionKind::Invisible => self.invisible = false,
            ConditionKind::Strengthen => self.strengthen = false,
            ConditionKind::Wound => self.wound = false,
            ConditionKind::Poison => self.poison = false,
            ConditionKind::Immobilize => self.immobilize = false,
            ConditionKind::Disarm => self.disarm = false,
            ConditionKind::Muddle => self.muddle = false,
        }
    }
}

pub enum ConditionKind {
    Invisible,
    Strengthen,
    Wound,
    Poison,
    Immobilize,
    Disarm,
    Muddle,
}

/* TODO: This or some other struct may need to contain number of targets or AOE also range */
pub struct Attack {
    source: Entity,
    damage: usize,
    pierce: usize,
    push: usize,
    pull: usize,
}
