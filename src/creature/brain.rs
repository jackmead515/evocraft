use macroquad::rand::gen_range;
use macroquad::prelude::*;

use crate::util;
use crate::creature::*;


#[derive(Debug, Clone)]
pub struct Brain {
    pub input_types: Vec<InputTypes>,
    pub output_types: Vec<OutputTypes>,
    pub hidden_neurons: Vec<Neuron>,
    pub output_neurons: Vec<Neuron>,
    pub activation: ActivationFunction,
    pub last_decisions: Vec<OutputTypes>,
    pub last_decision_time: f64,
    pub decision_speed: f32,
    hidden_buffer: Vec<f32>,
    output_buffer: Vec<f32>,
}


#[derive(Debug, Clone)]
pub struct Neuron {
    /// the activation function of the neuron
    pub activation: ActivationFunction,

    /// the bias of the neuron
    pub bias: Vec<f32>,

    /// the weights of the neuron
    pub weights: Vec<f32>,
}


#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    ReLU,
    Softmax,
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

    pub fn compute(&self, value: f32) -> f32 {
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-value).exp()),
            ActivationFunction::Tanh => value.tanh(),
            ActivationFunction::ReLU => if value > 0.0 { value } else { 0.0 },
            ActivationFunction::Softmax => value.exp(),
        }
    }
}


impl Neuron {

    pub fn new(activation: ActivationFunction, bias: Vec<f32>, weights: Vec<f32>) -> Neuron {
        Neuron {
            activation: activation,
            bias: bias,
            weights: weights,
        }
    }

    // compute the output of the neuron with the given activation function
    pub fn compute(&self, inputs: &Vec<f32>) -> f32 {
        let mut total = 0.0;

        for index in 0..self.weights.len() {
            let weight = self.weights[index];
            let bias = self.bias[index];
            let input = inputs[index];
            total += weight * input + bias;
        }

        return self.activation.compute(total);
    }


    pub fn mutate(&mut self) {
        for bias in self.bias.iter_mut() {
            if gen_range(0, 100) < 10 {
                *bias = util::overflow_add(*bias, gen_range(-0.1, 0.1), 1.0, -1.0);
            }
        }

        for weight in self.weights.iter_mut() {
            if gen_range(0, 100) < 10 {
                *weight = util::overflow_add(*weight, gen_range(-0.1, 0.1), 1.0, -1.0);
            }
        }
    }

}


impl Brain {

    pub fn random(decision_speed: f32) -> Brain {

        let input_options = InputTypes::get_inputs();
        let output_options = OutputTypes::get_outputs();

        let total_inputs = input_options.iter().map(|x| InputTypes::total_inputs(x)).sum();
        let total_hidden = gen_range(1, 30);
        let total_outputs = output_options.len();

        // select random hidden neurons
        let mut hidden_neurons: Vec<Neuron> = Vec::with_capacity(total_hidden);
        for _ in 0..total_hidden {
            let activation = gen_range(0, 2);
            let mut bias: Vec<f32> = Vec::with_capacity(total_inputs);
            let mut weights: Vec<f32> = Vec::with_capacity(total_inputs);
            for _ in 0..total_inputs {
                weights.push(gen_range(-1.0, 1.0));
                bias.push(gen_range(-1.0, 1.0));
            }
            hidden_neurons.push(Neuron::new(ActivationFunction::from(activation), bias, weights));
        }

        // select random output neurons
        let mut output_neurons: Vec<Neuron> = Vec::with_capacity(total_outputs);
        for _ in 0..total_outputs {
            let activation = gen_range(0, 2);
            let mut bias: Vec<f32> = Vec::with_capacity(total_hidden);
            let mut weights: Vec<f32> = Vec::with_capacity(total_hidden);
            for _ in 0..total_hidden {
                weights.push(gen_range(-1.0, 1.0));
                bias.push(gen_range(-1.0, 1.0));
            }
            output_neurons.push(Neuron::new(ActivationFunction::from(activation), bias, weights));
        }

        let mut hidden_buffer = Vec::with_capacity(hidden_neurons.len());
        hidden_buffer.resize(hidden_neurons.len(), 0.0);

        let mut output_buffer = Vec::with_capacity(output_neurons.len());
        output_buffer.resize(output_neurons.len(), 0.0);

        return Brain {
            input_types: input_options,
            hidden_neurons: hidden_neurons,
            output_types: output_options,
            output_neurons: output_neurons,
            activation: ActivationFunction::Softmax,
            last_decisions: Vec::new(),
            last_decision_time: 0.0,
            decision_speed: decision_speed,
            hidden_buffer: hidden_buffer,
            output_buffer: output_buffer,
        };
    }


    pub fn compute(&mut self, inputs: Vec<f32>) -> (Vec<f32>, OutputTypes) {
        let hidden_size = self.hidden_neurons.len();
        let output_size = self.output_neurons.len();

        // compute the hidden neurons
        for i in 0..hidden_size {
            self.hidden_buffer[i] = self.hidden_neurons[i].compute(&inputs);
        }

        // compute the output neurons
        for i in 0..output_size {
            self.output_buffer[i] = self.output_neurons[i].compute(&self.hidden_buffer)
        }

        // compute the softmax decision!!
        let mut exps = Vec::with_capacity(output_size);
        let mut exp_sum = 0.0;

        // compute the natural exponential
        // and the sum of the exponentials
        // at the same time
        for output in self.output_buffer.iter() {
            let exp = output.exp();
            exp_sum += exp;
            exps.push(exp);
        }

        let mut max = 0.0;
        let mut max_index: u8 = 0;
        let mut outputs = Vec::with_capacity(output_size);
        for i in 0..self.output_buffer.len() {
            // output of softmax is = exp / sum(exp)
            let output = exps[i] / exp_sum;

            // max decision based on max value
            if output > max {
                max = output;
                max_index = i as u8;
            }
            outputs.push(output);
        }

        let decision = self.output_types[max_index as usize];

        // save the last decision, remove the oldest one if necessary
        self.last_decisions.push(decision);
        if self.last_decisions.len() > InputTypes::total_inputs(&InputTypes::LastDecisions) {
            self.last_decisions.remove(0);
        }

        return (outputs, decision);
    }

    pub fn can_decide(&self, elapsed: f64) -> bool {
        return elapsed - self.last_decision_time > self.decision_speed as f64;
    }
}

