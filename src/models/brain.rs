#[derive(Debug, Clone)]
pub struct Brain {
    pub input_types: Vec<InputTypes>,
    pub output_types: Vec<OutputTypes>,
    pub hidden_neurons: Vec<Neuron>,
    pub output_neurons: Vec<Neuron>,
    pub activation: ActivationFunction,
    pub total_input_length: usize,
}

#[derive(Debug, Clone)]
pub struct Neuron {
    /// the activation function of the neuron
    pub activation: ActivationFunction,

    /// the bias of the neuron
    pub bias: f32,

    /// the weights of the neuron
    pub weights: Vec<f32>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum InputTypes {
    CurrentPosition,
    CurrentHealth,
    CurrentEnergy,
    NearCreatures,
    PlayerPosition,
    PlayerHealth,
    PlayerEnergy,
}


#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OutputTypes {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Nothing
}


#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    ReLU,
    Softmax,
}

impl InputTypes {
    pub fn from(value: u8) -> InputTypes {
        match value {
            0 => InputTypes::CurrentPosition,
            1 => InputTypes::NearCreatures,
            2 => InputTypes::PlayerPosition,
            3 => InputTypes::CurrentHealth,
            4 => InputTypes::CurrentEnergy,
            5 => InputTypes::PlayerEnergy,
            6 => InputTypes::PlayerHealth,
            _ => panic!("Invalid input type"),
        }
    }

    pub fn variants() -> Vec<InputTypes> {
        vec![
            InputTypes::CurrentPosition,
            InputTypes::NearCreatures,
            InputTypes::PlayerPosition,
            InputTypes::CurrentHealth,
            InputTypes::CurrentEnergy,
            InputTypes::PlayerEnergy,
            InputTypes::PlayerHealth,
        ]
    }

    pub fn input_amount(&self) -> usize {
        match self {
            InputTypes::CurrentPosition => 2,
            InputTypes::NearCreatures => 16,
            InputTypes::PlayerPosition => 2,
            InputTypes::CurrentHealth => 1,
            InputTypes::CurrentEnergy => 1,
            InputTypes::PlayerEnergy => 1,
            InputTypes::PlayerHealth => 1,
        }
    }

    pub fn total_inputs(input_types: &Vec<InputTypes>) -> usize {
        let mut total_inputs = 0;
        for input_type in input_types {
            total_inputs += input_type.input_amount();
        }
        return total_inputs;
    }
}


impl OutputTypes {
    pub fn from(value: u8) -> OutputTypes {
        match value {
            0 => OutputTypes::MoveUp,
            1 => OutputTypes::MoveDown,
            2 => OutputTypes::MoveLeft,
            3 => OutputTypes::MoveRight,
            4 => OutputTypes::Nothing,
            _ => panic!("Invalid output type"),
        }
    }

    pub fn variants() -> Vec<OutputTypes> {
        vec![
            OutputTypes::MoveUp,
            OutputTypes::MoveDown,
            OutputTypes::MoveLeft,
            OutputTypes::MoveRight,
            OutputTypes::Nothing,
        ]
    }
}

impl ActivationFunction {
    pub fn from(value: u8) -> ActivationFunction {
        match value {
            0 => ActivationFunction::Sigmoid,
            1 => ActivationFunction::Tanh,
            2 => ActivationFunction::ReLU,
            3 => ActivationFunction::Softmax,
            _ => panic!("Invalid activation function"),
        }
    }
}