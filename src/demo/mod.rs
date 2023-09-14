use crate::models::*;

pub mod random_creatures;

pub fn update(game_state: &mut GameState) {

    match game_state.demo {
        DemoType::RandomCreatures1 => {
            random_creatures::update(game_state);
        }
    }

}

pub async fn generate(demo: DemoType) -> GameState {
    let game_state = match demo {
        DemoType::RandomCreatures1 => random_creatures::generate(),
        _ => panic!("Unknown demo type"),
    };

    return game_state.await;
}