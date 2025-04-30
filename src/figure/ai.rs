// Monster AI

use bevy::{prelude::*, utils::{hashbrown::hash_map::Entry, HashMap}};
use hexx::Hex;

use crate::scenario::{command::ScenarioCommand, map::HexGrid};

use super::{condition::{ConditionKind, Conditions}, movement::MovementKind, Team};

// TODO: "Monster" kind as well?
#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct Initiative {
    value: usize,
    summon: bool,
    id: usize,
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct EnemyFocusInfo {
    hex: Hex,
    initiative: Initiative,
}

// Inputs:  HexPosition
//          Team
//          Attack Range        (0 is treated as melee)
//          Conditions          (disarm overwrites attack range to 0; muddle finds focus with disadvantage)
//          Movement Kind       (to determine which negative hexes are treated as obstacles)
//                              (to determine whether movement can end in negative hex)
//                              (TODO: Teleport?)
//          Map                 (empty, negative, obstacle, difficult terrain, figure; TODO: Icy Terrain?)
//          Enemy Focus Info    (Position, Initiative, Summon?)
//                              (Prefiltered for Inivisble)
fn focus(hex: Hex, team: Team, range: u32, conditions: Conditions, movement_kind: MovementKind, map: HexGrid, enemies: Vec<EnemyFocusInfo>) -> Option<Hex> {
    // Adjust range for certain circumstances
    let range = if conditions.has(ConditionKind::Disarm) {
        0
    } else {
        range
    }.max(1);
    
    let mut foci_move: HashMap<EnemyFocusInfo, Vec<Hex>> = HashMap::new();

    for enemy in enemies {
        // Retrieve hexes to move to that allow "attack" on enemy
        let move_to = hex.rings(1..range); // TODO: exclusive range?

        foci_move.insert(enemy, move_to.flatten().collect());
    }

    //TODO: Determine hex cost on the grid

    //TODO: pathfind keeping the lowest one for that target
    //TODO: Already consider disadvantage on range attack here?

    //TODO: Filter foci_move my lowest movement cost
    
    //TODO: Resolve any ties
    let focus = Hex::ZERO;

    Some(focus)
}

//TODO: If no focus can be found, no longer consider negative hexes obstacles (if not flying movement)

//After Focus found
//TODO: Determine target movement hex for that focus, based on movement, etc.

fn focus_negative(hex: Hex, range: usize, movement_kind: MovementKind) -> Option<Hex> {

}

// Inputs:    Attack Range (if any; otherwise assume range 1)
//            Movement points
//            Movement kind
//            HexPosition
// Outputs:   Hex to use the ability from (current if no movement)
//            Hexes that are targeted (if any)
fn ai(movement: usize, range: usize, targets: usize) -> (Hex, Vec<Hex>) {
    let mut commands = vec![];

    /* TODO: Ordered by initiative */
    let figures: Vec<Entity> = vec![];
    /* TODO: Determine focus based on least movement used given range of the attack */
    let focus = figures
        .iter()
        .map(|figure| {
            /* TODO: Get area of hexes from enemy position within range */

            /* TODO: Filter out occupied hexes, but consider always flying on monster */

            /* TODO: Calculate movement necessary using pathfinding */

            /* TODO: Take minimum */

            figure
        })
        .min();

    /* Only move if there is a focus */
    if let Some(focus) = focus {
        if movement > 0 {
            /* TODO: Determine all valid hexes from where focus can be attack given movement and range */

            /* TODO: If attack is multi target it gets a bit more complicated */
            if targets > 1 {
            } else {
                /* TODO: Just take the hex with lowest movement */
            }
        } else {
            /* TODO: In this case I might want to do the reverse and go through the hexes I can attack around me */
        }
    }

    commands
}


fn ai_new() {
    let foci = {
        let foci = find_foci(true);
        if foci.is_empty() {
            find_foci(false)
        } else {
            foci
        }
    };
}

#[derive(Debug, Default, Clone)]
pub struct Focus {
    from: Hex,
    target: Hex,
}

fn find_foci(negative_is_obstacle: bool) -> Vec<Focus> {
    vec![]   
}

fn foci_tiebreaker_range(foci: Vec<Focus>, origin: Hex) -> Vec<Focus> {
    let min_range = foci.iter().min_by(|x, y| {
        x.target.unsigned_distance_to(origin).cmp(&y.target.unsigned_distance_to(origin))
    }).unwrap().target.unsigned_distance_to(origin);

    foci.into_iter().filter(|f| {
        f.target.unsigned_distance_to(origin) == min_range
    }).collect()
}

fn foci_tiebreaker_initiative(foci: Vec<Focus>, figure_id: HashMap<Hex, u32>, initiative: HashMap<u32, u32>) -> Vec<Focus> {
    foci.sort_by(|f1, f2| {
        let f1_initiative = initiative.get(figure_id.get(&f1.target).unwrap()).unwrap();
        let f2_initiative = initiative.get(figure_id.get(&f2.target).unwrap()).unwrap();

        f1_initiative.cmp(f2_initiative)
    });


}

fn foci_tiebreaker_final(foci: Vec<Focus>) -> Focus {
    foci[0]
}