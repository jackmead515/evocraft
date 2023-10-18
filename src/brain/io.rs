#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum BrainInputTypes {
    Behavior,
    Movement,
    Rest
}

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
    BehaviorMove,
    BehaviorRest,
    BehaviorNothing,
    BehaviorContinue,
    MovementUp,
    MovementDown,
    MovementLeft,
    MovementRight,
    // Eat
    // Attack
    // Reproduce
    // Wander
    // Flee
    // Follow
    // etc...
}

impl BrainInputTypes {

    pub fn get_inputs(&self) -> Vec<InputTypes> {
        match self {
            BrainInputTypes::Behavior => vec![
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
            ],
            _ => panic!("Not implemented yet"),
            // BrainInputTypes::Movement => vec![],
            // BrainInputTypes::Rest => vec![],
        }
    }

    pub fn get_outputs(&self) -> Vec<OutputTypes> {
        match self {
            BrainInputTypes::Behavior => vec![
                OutputTypes::BehaviorMove,
                OutputTypes::BehaviorRest,
                OutputTypes::BehaviorNothing,
            ],
            BrainInputTypes::Movement => vec![
                OutputTypes::MovementUp,
                OutputTypes::MovementDown,
                OutputTypes::MovementLeft,
                OutputTypes::MovementRight,
            ],
            _ => panic!("Not implemented yet"),
            // BrainInputTypes::Rest => vec![],
        }
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

}