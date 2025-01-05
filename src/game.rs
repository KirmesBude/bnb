use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<ScenarioState>()
            .add_sub_state::<RoundState>();

        app.register_type::<AppState>()
            .register_type::<ScenarioState>()
            .register_type::<RoundState>()
            .register_type::<Round>();

        AppState::setup(app);
        ScenarioState::setup(app);
        RoundState::setup(app);

        app.add_plugins(StateInspectorPlugin::<AppState>::default());
        app.add_plugins(StateInspectorPlugin::<ScenarioState>::default());
        app.add_plugins(StateInspectorPlugin::<RoundState>::default());
        app.add_plugins(MeshPickingPlugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

impl AppState {
    fn setup(app: &mut App) -> &mut App {
        app.add_systems(Update, menu_transition.run_if(in_state(AppState::Menu)))
    }
}

fn menu_transition(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::InGame);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(AppState = AppState::InGame)]
pub enum ScenarioState {
    #[default]
    Begin,
    Play,
    End,
}

impl ScenarioState {
    fn setup(app: &mut App) -> &mut App {
        app.add_systems(
            Update,
            begin_transition.run_if(in_state(ScenarioState::Begin)),
        )
        .add_systems(
            Update,
            play_transition.run_if(in_state(ScenarioState::Play)),
        )
        .add_systems(Update, end_transition.run_if(in_state(ScenarioState::End)))
    }
}

fn begin_transition(
    mut next_state: ResMut<NextState<ScenarioState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        next_state.set(ScenarioState::Play);
    }
}

fn play_transition(
    mut next_state: ResMut<NextState<ScenarioState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        next_state.set(ScenarioState::End);
    }
}

fn end_transition(
    mut next_state: ResMut<NextState<ScenarioState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        next_state.set(ScenarioState::Begin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates, Reflect)]
#[source(ScenarioState = ScenarioState::Play)]
pub enum RoundState {
    #[default]
    Init,
    StartOfRoundEffects,
    CardSelection,
    OrderingInitiative,
    CharacterAndMonsterTurns,
    EndOfRound,
}

impl RoundState {
    fn setup(app: &mut App) -> &mut App {
        app.add_systems(OnEnter(RoundState::Init), init_on_enter)
            .add_systems(Update, init_transition.run_if(in_state(RoundState::Init)))
            .add_systems(
                OnEnter(RoundState::StartOfRoundEffects),
                start_of_round_effects_on_enter,
            )
            .add_systems(
                Update,
                start_of_round_effects_transition.run_if(in_state(RoundState::StartOfRoundEffects)),
            )
            .add_systems(
                Update,
                card_selection_transition.run_if(in_state(RoundState::CardSelection)),
            )
            .add_systems(
                Update,
                ordering_initiative_transition.run_if(in_state(RoundState::OrderingInitiative)),
            )
            .add_systems(
                Update,
                character_and_monster_turns_transition
                    .run_if(in_state(RoundState::CharacterAndMonsterTurns)),
            )
            .add_systems(
                Update,
                end_of_round_transition.run_if(in_state(RoundState::EndOfRound)),
            )
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Round(usize);

fn init_on_enter(mut commands: Commands) {
    commands.insert_resource(Round::default());
}

fn init_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::StartOfRoundEffects);
    }
}

fn start_of_round_effects_on_enter(mut round: ResMut<Round>) {
    round.0 += 1;
}

fn start_of_round_effects_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::CardSelection);
    }
}

fn card_selection_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::OrderingInitiative);
    }
}

fn ordering_initiative_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::CharacterAndMonsterTurns);
    }
}

fn character_and_monster_turns_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::EndOfRound);
    }
}

fn end_of_round_transition(
    mut next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(RoundState::StartOfRoundEffects);
    }
}
