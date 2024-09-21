#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum InputTypes {
    CurrentPosition,
    CurrentHealth,
    CurrentEnergy,
    PlayerPosition,
    PlayerHealth,
    PlayerEnergy,
    RandomInput,
    TimeSinoidInput,
    LastDecisions,
    CurrentAge,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OutputTypes {
    BehaviorWander,
    BehaviorRest,
    BehaviorNothing,
    BehaviorContinue,
    // Eat
    // Attack
    // Reproduce
    // Wander
    // Flee
    // Follow
    // etc...
}

impl OutputTypes {
    pub fn get_outputs() -> Vec<OutputTypes> {
        return vec![
            OutputTypes::BehaviorWander,
            OutputTypes::BehaviorRest,
            OutputTypes::BehaviorNothing,
            OutputTypes::BehaviorContinue,
        ];
    }
}



impl InputTypes {

    pub fn total_inputs(input_type: &InputTypes) -> usize {
        return match input_type {
            InputTypes::CurrentPosition => 2,
            InputTypes::CurrentHealth => 1,
            InputTypes::CurrentEnergy => 1,
            InputTypes::PlayerPosition => 2,
            InputTypes::PlayerHealth => 1,
            InputTypes::PlayerEnergy => 1,
            InputTypes::RandomInput => 1,
            InputTypes::TimeSinoidInput => 1,
            InputTypes::LastDecisions => 5,
            InputTypes::CurrentAge => 1,
            _ => panic!("Not implemented yet"),
        }
    }

    pub fn get_inputs() -> Vec<InputTypes> {
        return vec![
            InputTypes::CurrentPosition,
            InputTypes::CurrentHealth,
            InputTypes::CurrentEnergy,
            InputTypes::PlayerPosition,
            InputTypes::PlayerHealth,
            InputTypes::PlayerEnergy,
            InputTypes::RandomInput,
            InputTypes::TimeSinoidInput,
            InputTypes::LastDecisions,
            InputTypes::CurrentAge,
        ];
    }

}