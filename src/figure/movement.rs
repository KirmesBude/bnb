use bevy::prelude::*;
use hexx::Hex;

use crate::scenario::{
    command::{ScenarioCommand, ScenarioCommandTrait, ScenarionCommandExecuteResult},
    map::{HexGrid, HexPosition},
};

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub enum MovementKind {
    #[default]
    Default,
    Jump,
    Fly,
}

#[derive(Debug, Clone, Reflect)]
pub struct MoveCommand {
    entity: Entity,
    start: Option<Hex>,
    end: Hex,
    kind: MovementKind,
}

impl MoveCommand {
    pub fn new(entity: Entity, hex: Hex) -> Self {
        Self {
            entity,
            end: hex,
            start: Default::default(),
            kind: Default::default(),
        }
    }

    pub fn _with_kind(mut self, kind: MovementKind) -> Self {
        self.kind = kind;
        self
    }
}

impl ScenarioCommandTrait for MoveCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let hex_grid = {
            let parent = world.get::<Parent>(self.entity).unwrap();
            parent.get()
        };
        let [mut hex_grid, mut hex_position] =
            world.get_entity_mut([hex_grid, self.entity]).unwrap();

        let mut hex_grid = hex_grid.get_mut::<HexGrid>().unwrap();
        let mut hex_position = hex_position.get_mut::<HexPosition>().unwrap();

        self.start = Some(hex_position.hex());
        hex_position.update(self.end, self.entity, &mut hex_grid);

        println!("Move {} to {:?}", self.entity, self.end);

        /* Reactivity how? Via an event that is consumed and someone adds to the queue? */
        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        let hex_grid = {
            let parent = world.get::<Parent>(self.entity).unwrap();
            parent.get()
        };
        let [mut hex_grid, mut hex_position] =
            world.get_entity_mut([hex_grid, self.entity]).unwrap();

        let mut hex_grid = hex_grid.get_mut::<HexGrid>().unwrap();
        let mut hex_position = hex_position.get_mut::<HexPosition>().unwrap();

        hex_position.update(self.start.unwrap(), self.entity, &mut hex_grid);

        let command = Self {
            start: None,
            ..self
        };
        command.into()
    }
}
