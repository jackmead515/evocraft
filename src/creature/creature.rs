use macroquad::rand::gen_range;

use crate::models::brain::*;
use crate::models::Creature;


fn overflow_add(a: f32, b: f32, max: f32, min: f32) -> f32 {
    let mut result = a + b;
    if result > max {
        result = min + (result - max);
    }
    if result < min {
        result = max - (min - result);
    }
    return result;
}


impl Neuron {

    pub fn new(activation: ActivationFunction, bias: Vec<f32>, weights: Vec<f32>) -> Neuron {
        Neuron {
            activation: activation,
            bias: bias,
            weights: weights,
        }
    }

    /// Computes the sigmoid output total given the input vector
    pub fn compute(&self, inputs: &Vec<f32>) -> f32 {
        let mut total = 0.0;

        for index in 0..self.weights.len() {
            let weight = self.weights[index];
            let bias = self.bias[index];
            let input = inputs[index];
            total += weight * input + bias;
        }

        // sigmoid activation
        return 1.0 / (1.0 + (-total).exp());
    }


    pub fn mutate(&mut self) {
        for bias in self.bias.iter_mut() {
            if gen_range(0, 100) < 10 {
                *bias = overflow_add(*bias, gen_range(-0.1, 0.1), 1.0, -1.0);
            }
        }

        for weight in self.weights.iter_mut() {
            if gen_range(0, 100) < 10 {
                *weight = overflow_add(*weight, gen_range(-0.1, 0.1), 1.0, -1.0);
            }
        }
    }

}


impl Brain {


    pub fn random() -> Brain {
        let mut input_options = InputTypes::variants();
        let mut output_options = OutputTypes::variants();

        let total_inputs = gen_range(4, input_options.len());
        let total_hidden = gen_range(1, 30);
        let total_outputs = output_options.len(); //gen_range(3,);

        // select random input types
        let mut input_types: Vec<InputTypes> = Vec::with_capacity(total_inputs+3);

        // all creatures will know:
        // - passage of time
        // - have a random input
        // - know their current age
        input_types.push(InputTypes::RandomInput);
        input_types.push(InputTypes::TimeSinoidInput);
        input_types.push(InputTypes::CurrentAge);

        for _ in 0..total_inputs {
            let index = gen_range(0, input_options.len());
            input_types.push(input_options.remove(index));
        }

        let total_input_length = InputTypes::total_inputs(&input_types);

        // select random hidden neurons
        let mut hidden_neurons: Vec<Neuron> = Vec::with_capacity(total_hidden);
        for _ in 0..total_hidden {
            let activation = gen_range(0, 2);
            let mut bias: Vec<f32> = Vec::with_capacity(total_input_length);
            let mut weights: Vec<f32> = Vec::with_capacity(total_input_length);
            for _ in 0..total_input_length {
                weights.push(gen_range(-1.0, 1.0));
                bias.push(gen_range(-1.0, 1.0));
            }
            hidden_neurons.push(Neuron::new(ActivationFunction::from(activation), bias, weights));
        }

        // select random output types
        let mut output_types: Vec<OutputTypes> = Vec::with_capacity(total_outputs);
        for _ in 0..total_outputs {
            let index = gen_range(0, output_options.len());
            output_types.push(output_options.remove(index));
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

        return Brain {
            input_types: input_types,
            hidden_neurons: hidden_neurons,
            output_types: output_types,
            output_neurons: output_neurons,
            activation: ActivationFunction::Softmax,
            last_decisions: Vec::with_capacity(InputTypes::LastDecisions.input_amount()),
            total_input_length: total_input_length,
            last_decision_time: 0.0,
            decision_speed: 0.3,
        };
    }


    pub fn compute(&mut self, inputs: Vec<f32>) -> (Vec<f32>, OutputTypes) {
        let hidden_size = self.hidden_neurons.len();
        let output_size = self.output_neurons.len();

        let mut hidden_buffer = Vec::with_capacity(hidden_size);
        let mut output_buffer = Vec::with_capacity(output_size);

        // compute the hidden neurons
        for i in 0..hidden_size {
            hidden_buffer.push(self.hidden_neurons[i].compute(&inputs));
        }

        // compute the output neurons
        for i in 0..output_size {
            output_buffer.push(self.output_neurons[i].compute(&hidden_buffer));
        }

        // compute the softmax decision!!
        let mut exps = Vec::with_capacity(output_size);
        let mut exp_sum = 0.0;

        // compute the natural exponential
        // and the sum of the exponentials
        // at the same time
        for output in output_buffer.iter() {
            let exp = output.exp();
            exp_sum += exp;
            exps.push(exp);
        }

        let mut max = 0.0;
        let mut max_index: u8 = 0;
        let mut outputs = Vec::with_capacity(output_size);
        for i in 0..output_buffer.len() {
            // output of softmax is = exp / sum(exp)
            let output = exps[i] / exp_sum;

            // max decision based on max value
            if output > max {
                max = output;
                max_index = i as u8;
            }
            outputs.push(output);
        }

        let decision = OutputTypes::from(max_index);

        // save the last decision, remove the oldest one if necessary
        self.last_decisions.push(decision);
        if self.last_decisions.len() > InputTypes::LastDecisions.input_amount() {
            self.last_decisions.remove(0);
        }

        return (outputs, decision);
    }


    pub fn mutate(&mut self) {
        for neuron in self.hidden_neurons.iter_mut() {
            neuron.mutate();
        }

        for neuron in self.output_neurons.iter_mut() {
            neuron.mutate();
        }
    }


    pub fn can_decide(&self, elapsed: f64) -> bool {
        return elapsed - self.last_decision_time > self.decision_speed as f64;
    }
}


impl Creature {

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

}

