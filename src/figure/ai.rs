// Monster AI

use bevy::prelude::*;

use crate::scenario::command::ScenarioCommand;

// Inputs:    Attack Range (if any; otherwise assume range 1)
//            Movement points
//            Movement kind
//            HexPosition
// Outputs:   In the form of commands?
//            HexPosition to move to
//            Targets to attack
fn ai(movement: usize, range: usize, targets: usize) -> Vec<ScenarioCommand> {
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
