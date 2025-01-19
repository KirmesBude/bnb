use std::collections::VecDeque;

use bevy::prelude::*;
use enum_dispatch::enum_dispatch;

use crate::figure::{
    attack::{ApplyAttackCommand, AttackCommand},
    condition::{AddConditionCommand, RemoveConditionCommand},
    health::SufferDamageCommand,
    modifier::RollModifierCommand,
    movement::{MoveCommand, MovementKind},
};

/* Everything that happens in the scenario needs to be recorded (and maybe this is the source of truth?) */
/* Every "action" needs to be reversible */
/* E.g. move, attack need enough information to be 2 way */
/* Do I even want to be able to undo every move? Should I have the possibility? */
/* It definitely needs to be atomic commands like move, but for redo they should be grouped? */
/* Should moving be a group of move commands or should the move command include a list of hexes to move through? */
/* The problem is that a single hex move might have a reaction that needs processing, which in turn creates another command on the stack */

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScenarioCommandQueue>();
        app.register_type::<ScenarioCommand>()
            .register_type::<MovementKind>()
            .register_type::<ScenarioCommandQueue>()
            .register_type::<MoveCommand>()
            .register_type::<AttackCommand>()
            .register_type::<AddConditionCommand>()
            .register_type::<RemoveConditionCommand>();

        app.add_systems(Update, step_commands);
    }
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ScenarioCommandQueue {
    history: Vec<ScenarioCommand>,
    pending: VecDeque<ScenarioCommand>,
}

impl ScenarioCommandQueue {
    pub fn undo(&mut self, world: &mut World) {
        if let Some(command) = self.history.pop() {
            self.pending.clear();

            let command = command.undo(world);
            self.pending.push_back(command);
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        if let Some(mut command) = self.pending.pop_front() {
            match command.execute(world) {
                ScenarionCommandExecuteResult::Pending => (),
                ScenarionCommandExecuteResult::Done(commands) => {
                    let mut commands: VecDeque<_> = commands.into();
                    self.history.push(command);

                    commands.append(&mut self.pending);
                    self.pending = commands;
                }
            }
        }
    }

    pub fn queue(&mut self, commands: Vec<ScenarioCommand>) {
        self.pending.extend(commands);
    }

    /// History recent to oldest
    pub fn history(&self) -> impl Iterator<Item = &ScenarioCommand> {
        self.history.iter().rev()
    }
}

fn step_commands(world: &mut World) {
    /* SAFETY: Need to ensure disjoint queries, more importantly commands are not allowed to modify ButtonInput<KeyCode> or CommandQueue through World */
    unsafe {
        let world = world.as_unsafe_world_cell();
        let keyboard_input = world.get_resource::<ButtonInput<KeyCode>>().unwrap();
        let mut command_queue = world.get_resource_mut::<ScenarioCommandQueue>().unwrap();

        if keyboard_input.just_pressed(KeyCode::Enter) {
            println!("Enter");

            command_queue.execute(world.world_mut());
        } else if keyboard_input.just_pressed(KeyCode::Backspace) {
            println!("Backspace");

            command_queue.undo(world.world_mut());
        }
    }
}

pub enum ScenarionCommandExecuteResult {
    Pending,
    Done(Vec<ScenarioCommand>),
}

#[enum_dispatch(ScenarioCommand)]
pub trait ScenarioCommandTrait {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult;

    fn undo(self, world: &mut World) -> ScenarioCommand;
}

#[allow(clippy::enum_variant_names)]
#[enum_dispatch]
#[derive(Debug, Clone, Reflect)]
pub enum ScenarioCommand {
    MoveCommand,
    AttackCommand,
    ApplyAttackCommand,
    SufferDamageCommand,
    AddConditionCommand,
    RemoveConditionCommand,
    RollModifierCommand,
}
