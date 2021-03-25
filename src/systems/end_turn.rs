use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    //queeries
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>()); // (1)
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state, // (3)
    };

    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        // (4)
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
