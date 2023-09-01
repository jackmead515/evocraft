use rand::Rng;

use crate::models::brain::*;

impl Neuron {

    pub fn new(activation: ActivationFunction, bias: f32, weights: Vec<f32>) -> Neuron {
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
            let input = inputs[index];
            total += weight * input;
        }

        // sigmoid activation
        return 1.0 / (1.0 + (-total).exp());
    }

}


impl Brain {


    pub fn random() -> Brain {
        let mut thread_rand = rand::thread_rng();

        let mut input_options = InputTypes::variants();
        let mut output_options = OutputTypes::variants();

        let total_inputs = thread_rand.gen_range(1..=input_options.len());
        let total_hidden = thread_rand.gen_range(1..=10);
        let total_outputs = thread_rand.gen_range(1..=output_options.len());

        // select random input types
        let mut input_types: Vec<InputTypes> = Vec::with_capacity(total_inputs);
        for _ in 0..total_inputs {
            let index = thread_rand.gen_range(0..input_options.len());
            let input_type = input_options.remove(index);
            input_types.push(input_type);
        }

        let total_input_length = InputTypes::total_inputs(&input_types);

        // select random hidden neurons
        let mut hidden_neurons: Vec<Neuron> = Vec::with_capacity(total_hidden);
        for _ in 0..total_hidden {
            let activation = thread_rand.gen_range(0..=2);
            let bias = thread_rand.gen_range(-1.0..=1.0);
            let mut weights: Vec<f32> = Vec::with_capacity(total_input_length);
            for _ in 0..total_input_length {
                weights.push(thread_rand.gen_range(-1.0..=1.0));
            }
            hidden_neurons.push(Neuron::new(ActivationFunction::from(activation), bias, weights));
        }

        // select random output types
        let mut output_types: Vec<OutputTypes> = Vec::with_capacity(total_outputs);
        for _ in 0..total_outputs {
            let index = thread_rand.gen_range(0..output_options.len());
            let output_type = output_options.remove(index);
            output_types.push(output_type);
        }

        // select random output neurons
        let mut output_neurons: Vec<Neuron> = Vec::with_capacity(total_outputs);
        for _ in 0..total_outputs {
            let activation = thread_rand.gen_range(0..=2);
            let bias = thread_rand.gen_range(-1.0..=1.0);
            let mut weights: Vec<f32> = Vec::with_capacity(total_hidden);
            for _ in 0..total_hidden {
                weights.push(thread_rand.gen_range(-1.0..=1.0));
            }
            output_neurons.push(Neuron::new(ActivationFunction::from(activation), bias, weights));
        }

        return Brain {
            input_types: input_types,
            hidden_neurons: hidden_neurons,
            output_types: output_types,
            output_neurons: output_neurons,
            activation: ActivationFunction::Softmax,
            total_input_length: total_input_length
        };
    }


    pub fn compute(&self, inputs: Vec<f32>) -> (Vec<f32>, OutputTypes) {
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

        return (outputs, OutputTypes::from(max_index));
    }

}


// impl Genes for Brain {

//     fn code(&self) -> Vec<String> {

//         let mut code: Vec<u8> = Vec::new();

//         // get input types
//         for input_type in &self.input_types {
//             code.push(*input_type as u8);
//         }

//         // get hidden neurons
//         for neuron in &self.hidden_neurons {
//             code.push(neuron.activation as u8);
//             code.extend_from_slice(&neuron.bias.to_le_bytes());
//             for weight in &neuron.weights {
//                 code.extend_from_slice(&weight.to_le_bytes());
//             }
//         }

//         // get output types
//         for output_type in &self.output_types {
//             code.push(*output_type as u8);
//         }

//         // get output neurons
//         for neuron in &self.output_neurons {
//             code.push(neuron.activation as u8);
//             code.extend_from_slice(&neuron.bias.to_le_bytes());
//             for weight in &neuron.weights {
//                 code.extend_from_slice(&weight.to_le_bytes());
//             }
//         }

//         // convert vec<u8> to hex string
//         let mut code_string = String::new();
//         for byte in code {
//             code_string.push_str(&format!("{:02x}", byte));
//         }
        
//         // split string into 16 character chunks
//         let mut code_chunks: Vec<String> = Vec::new();
//         for chunk in code_string.as_bytes().chunks(16) {
//             code_chunks.push(String::from_utf8(chunk.to_vec()).unwrap());
//         }

//         return code_chunks;
//     }


//     fn mutate(&mut self) {
//         panic!("Not implemented");
//     }
// }

