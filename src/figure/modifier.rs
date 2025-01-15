use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    utils::HashMap,
};

use crate::scenario::command::{
    ScenarioCommand, ScenarioCommandTrait, ScenarionCommandExecuteResult,
};

use super::{attack::PendingAttack, FigureId};

/*
    This defines ModifierTray (component) entities for each figure id
    and all modifiers
*/

#[derive(Debug, Clone, Copy, Reflect)]
pub enum Modifier {
    Add(i8),
    Multiply(i8),
}

impl Modifier {
    pub fn add(value: i8) -> Self {
        Self::Add(value)
    }

    pub fn multiply(value: i8) -> Self {
        Self::Multiply(value)
    }

    pub fn zero() -> Self {
        Self::Add(0)
    }

    pub fn plus_one() -> Self {
        Self::Add(1)
    }

    pub fn minus_one() -> Self {
        Self::Add(-1)
    }

    pub fn plus_two() -> Self {
        Self::Add(2)
    }

    pub fn minus_two() -> Self {
        Self::Add(-2)
    }

    pub fn miss() -> Self {
        Self::Multiply(0)
    }

    pub fn crit() -> Self {
        Self::Multiply(2)
    }
}

// TODO: Is this a component?
// TODO: I think this should not exist like this, there are no rolling right now
#[derive(Debug, Default, Clone, Reflect)]
pub struct ModifierStack(Vec<Modifier>);

impl ModifierStack {
    pub fn apply(&self, damage: usize) -> usize {
        self.0
            .iter()
            .fold(damage as isize, |acc, modifier| match modifier {
                Modifier::Add(x) => acc + *x as isize,
                Modifier::Multiply(x) => acc * *x as isize,
            })
            .max(0) as usize
    }

    pub fn push(&mut self, modifier: Modifier) {
        self.0.push(modifier);
    }

    pub fn pop(&mut self) -> Option<Modifier> {
        self.0.pop()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug, Reflect)]
pub enum ModifierTrayColumn {
    Minus,
    Neutral,
    Plus,
    Last,
}

impl ModifierTrayColumn {
    const len: usize = ModifierTrayColumn::Last as usize;
}

#[derive(Debug, Component, Reflect)]
#[component(on_add = ModifierTray::on_add, on_remove = ModifierTray::on_remove)]
pub struct ModifierTray {
    id: FigureId,
    active_row: usize,
    table: [[Modifier; ModifierTrayColumn::len]; ModifierTray::len],
}

impl ModifierTray {
    const len: usize = 6;

    pub fn new(id: FigureId, table: [[Modifier; ModifierTrayColumn::len]; Self::len]) -> Self {
        Self {
            id,
            table,
            active_row: 0,
        }
    }

    pub fn get(&self, column: ModifierTrayColumn) -> Modifier {
        self.table[self.active_row][column as usize]
    }

    pub fn next_row(&mut self) {
        self.active_row = (self.active_row + 1) % Self::len;
    }

    fn on_add(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let id = world.get::<ModifierTray>(entity).unwrap().id;
        let mut modifier_trays = world.get_resource_mut::<ModifierTrays>().unwrap();

        modifier_trays.0.insert(id, entity);
    }

    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let id = world.get::<ModifierTray>(entity).unwrap().id;
        let mut modifier_trays = world.get_resource_mut::<ModifierTrays>().unwrap();

        modifier_trays.0.remove(&id);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
pub struct ModifierTrays(HashMap<FigureId, Entity>);

impl ModifierTrays {
    pub fn get(&self, id: &FigureId) -> Option<Entity> {
        self.0.get(id).copied()
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct RollModifierCommand {
    entity: Entity,
    previous_row: Option<usize>,
}

impl RollModifierCommand {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            previous_row: None,
        }
    }
}

impl ScenarioCommandTrait for RollModifierCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let modifier_trays = world.get_resource::<ModifierTrays>().unwrap();
        let figure_id = world.get::<FigureId>(self.entity).unwrap();
        let modifier_tray_entity = modifier_trays.get(figure_id).unwrap();

        /* TODO: Randomly throw dice */
        let column = ModifierTrayColumn::Neutral;

        let mut modifier_tray = world.get_mut::<ModifierTray>(modifier_tray_entity).unwrap();
        self.previous_row = Some(modifier_tray.active_row);

        let modifier = modifier_tray.get(column);
        modifier_tray.next_row();

        let mut pending_attack = world.get_mut::<PendingAttack>(self.entity).unwrap();
        pending_attack.get_modifiers_mut().push(modifier);

        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        let modifier_trays = world.get_resource::<ModifierTrays>().unwrap();
        let figure_id = world.get::<FigureId>(self.entity).unwrap();
        let modifier_tray_entity = modifier_trays.get(figure_id).unwrap();

        let mut modifier_tray = world.get_mut::<ModifierTray>(modifier_tray_entity).unwrap();

        modifier_tray.active_row = self.previous_row.unwrap();

        let mut pending_attack = world.get_mut::<PendingAttack>(self.entity).unwrap();
        pending_attack.get_modifiers_mut().pop();

        let command = Self {
            previous_row: None,
            ..self
        };
        command.into()
    }
}

/* TODO: ClearModifierStack needs to be a command */
