use bevy::{prelude::*, utils::HashSet};

/* Each figure has a set of possible conditions and  */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum ConditionKind {
    Invisible,
    Strengthen,
    Wound,
    Poison,
    Immobilize,
    Disarm,
    Muddle,
}

/* You want conditions and immunities on the same struct */
#[derive(Debug, Component, Reflect)]
pub struct Conditions {
    conditions: HashSet<ConditionKind>,
    /* Immunities are immutable and can only be specified at creation */
    immunities: HashSet<ConditionKind>,
}

impl Conditions {
    pub fn new(immunities: &[ConditionKind]) -> Self {
        Self {
            conditions: HashSet::new(),
            immunities: immunities.iter().copied().collect(),
        }
    }

    pub fn add_condition(&mut self, condition: ConditionKind) {
        if !self.immunities.contains(&condition) {
            self.conditions.insert(condition);
        }
    }

    pub fn remove_condition(&mut self, condition: ConditionKind) {
        self.conditions.remove(&condition);
    }

    pub fn contains(&self, condition: ConditionKind) -> bool {
        self.conditions.contains(&condition)
    }
}
