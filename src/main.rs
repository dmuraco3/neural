use core::{borrow, panic};
use std::{rc::Rc, cell::{RefCell, Ref}, ops::{Mul, Add, Neg}, fs::File, borrow::Borrow, process::Output};

use rustgrad::{new_tensor, tensor::{Tensor, TensorTrait, utils::{alloc_box_buffer}, ops::matmul::{matmul, scalar_matmul}}};

use rand::{Rng, prelude::Distribution, distributions::Standard};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Neuron<T: TensorTrait<T>> {
    inner  :  Tensor<T>,
    weights:  Option<Box<[T]>>,
    bias   :  Option<T>,
    gradient: Option<T>

}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Layer<T: TensorTrait<T>> {
    neurons: Box<[Neuron<T>]>,
    activation_function: Option<fn(input: &mut Tensor<T>)>
}

pub struct InputLayer<T: TensorTrait<T>> {
    nuerons: Box<[Neuron<T>]>,
}

pub struct DenseLayer<T: TensorTrait<T>> {
    neurons: Box<[Neuron<T>]>,
    activation_function: fn(input: &mut Tensor<T>)
}

pub struct ActivationLayer<T: TensorTrait<T>> {
    neurons: Box<[Neuron<T>]>,
    activation_function: fn(input: &mut Box<[Neuron<T>]>)
}

impl <T: TensorTrait<T> + Mul<Output = T>> Layer<T> 
where 
    Standard: Distribution<T>
{

    
    pub fn new_input(size: usize, inner_shape: Vec<usize>) -> Self {
        let mut layer_alloc = alloc_box_buffer::<Neuron<T>>(size);

        let neuron_length = inner_shape.iter().fold(1, |acc, x| acc * x);

        for x in 0..size {
            layer_alloc[x] = Neuron {
                inner: Tensor::new(alloc_box_buffer(neuron_length), inner_shape.clone().into_boxed_slice()),
                weights: None,
                bias: None,
                gradient: None
            }
        }
        
        Self {
            neurons: layer_alloc,
            activation_function: None
        }
    }

    pub fn new_dense_layer(size: usize, inner_shape: Vec<usize>, activation_function: fn(input: &mut Tensor<T>)) -> Self {
        let mut layer_alloc = alloc_box_buffer::<Neuron<T>>(size);

        let mut rng = rand::thread_rng();

        let neuron_length = inner_shape.iter().fold(1, |acc, x| acc * x);

        for x in 0..size {
            layer_alloc[x] = Neuron {
                inner: Tensor::new(alloc_box_buffer(neuron_length), inner_shape.clone().into_boxed_slice()),
                weights: None,
                bias: Some(rng.gen()),
                gradient: Some(rng.gen())
            }
        }
    
        Self {
            neurons: layer_alloc,
            activation_function: Some(activation_function)
        }
    }

}

fn tanh<T: TensorTrait<T>>(input: &mut Tensor<T>)
{   
    input.inner.iter_mut().for_each(|inner_val| {
        *inner_val = (inner_val.exp() - (inner_val.neg()).exp()) / (inner_val.exp() + (inner_val.neg()).exp())
    });
}

fn relu<T: TensorTrait<T>>(input: &mut Tensor<T>)
{
    input.inner.iter_mut().for_each(|inner_val| {
        *inner_val = inner_val.max(T::zero());
    });
}

fn softmax<T: TensorTrait<T>>(input: &mut Tensor<T>)
{
    let normalization = input.inner.iter().fold(T::zero(), |acc, x| acc + x.exp());

    println!("{:?}", normalization);
}

struct Model<T: TensorTrait<T>> {
    layers: Box<[Rc<RefCell<Layer<T>>>]>
}

impl <T:TensorTrait<T> + Mul<Output = T> + Add<Output = T>> Model<T> 
where   
    Standard: Distribution<T>
{
    pub fn add_layer(&mut self, layer: Layer<T>) {
        if self.layers.len() == 0 {
            // ading input layer
            let mut layer_alloc = self.layers.to_vec();
            
            layer_alloc.push(Rc::new(RefCell::new(layer)));

            self.layers = layer_alloc.into_boxed_slice();

        } else {
            // adding dense layer
            let mut layer_alloc = self.layers.to_vec();

            let mut layer = layer;

            let prev_layer = self.layers[self.layers.len()-1].try_borrow().unwrap();

            let mut rng = rand::thread_rng();

            for neuron in layer.neurons.iter_mut() {
                let mut weight_alloc = alloc_box_buffer::<T>(prev_layer.neurons.len());

                for weight in weight_alloc.iter_mut() {
                    *weight = rng.gen();
                }

                neuron.weights = Some(weight_alloc);
            }

            layer_alloc.push(Rc::new(RefCell::new(layer)));

            drop(prev_layer);

            self.layers = layer_alloc.into_boxed_slice();
        }
    }


    pub fn feed_forward(&mut self, input: Box<[Tensor<T>]>) {
        let mut input_layer = self.layers[0].borrow_mut();

        // make sure input tensor shape matches input_layer tensor shape
        assert_eq!(input_layer.neurons[0].inner.shape, input[0].shape);


        for (index, neuron) in input_layer.neurons.iter_mut().enumerate() {
            neuron.inner = input.get(index).unwrap().to_owned();
        }

        drop(input_layer);

        for layer_index in 1..self.layers.len() {

            let feed = self.layers[layer_index-1].try_borrow().unwrap();

            let mut curr_layer = self.layers[layer_index].borrow_mut();

            let activation = curr_layer.activation_function.unwrap();

            curr_layer.neurons.iter_mut().for_each(|neuron| {

                feed.neurons.iter().zip(neuron.weights.as_ref().unwrap().iter()).for_each(|(feed_neuron, weight)| {

                    neuron.inner = &neuron.inner + &feed_neuron.inner * *weight;

                });

                neuron.inner = &neuron.inner + neuron.bias.unwrap();

                //activate neuron input

                activation(&mut neuron.inner);
            });

        }   
    }
}

impl <T:TensorTrait<T>> Default for Model<T> {
    fn default() -> Self {
        let layers_alloc = alloc_box_buffer::<Rc<RefCell<Layer<T>>>>(0);

        Self {
            layers: layers_alloc
        }
    }
}


fn main() {

    let l0: Layer<f32> = Layer::new_input(784, vec![1]);

    let l1: Layer<f32> = Layer::new_dense_layer(128, vec![1], tanh);

    let l2: Layer<f32> = Layer::new_dense_layer(64 , vec![1], tanh);

    let l3: Layer<f32> = Layer::new_dense_layer(10 , vec![1], softmax);

    let mut model: Model<f32> = Model::default();

    model.add_layer(l0);

    model.add_layer(l1);

    model.add_layer(l2);

    model.add_layer(l3);

    let file = File::open("/Users/dmuraco/rust_projects/neural/train.csv").unwrap();

    let mut reader = csv::Reader::from_reader(file);

    let mut test_set: Vec<(String, Box<[Tensor<f32>]>)> = Vec::new();

    let record = reader.records().next().unwrap().unwrap();

    let label = record[0].to_string();

    let mut values: Vec<f32> = Vec::with_capacity(784);

    for value_index in 1..record.len() {
        values.push(record[value_index].parse::<f32>().unwrap() / 255.0 ) // scale between 0 - 1 by dividing 255
    }

    test_set.push((label, convert_box_tensor(values.into_boxed_slice(), vec![1].into_boxed_slice())));

    let input = test_set[0].1.clone();

    model.feed_forward(input);

}

fn read_csv() -> Vec<(String, Box<[f32]>)> {
    let file = File::open("/Users/dmuraco/rust_projects/neural/train.csv").unwrap();

    let mut reader = csv::Reader::from_reader(file);

    let mut MNIST_DIGITS: Vec<(String, Box<[f32]>)> = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();

        let label = record[0].to_string();

        let mut values: Vec<f32> = Vec::with_capacity(784);

        for value_index in 1..record.len() {
            values.push(record[value_index].parse::<f32>().unwrap())
        }

        MNIST_DIGITS.push((label, values.into_boxed_slice()));
    }

    MNIST_DIGITS

}

fn convert_box_tensor<T:TensorTrait<T>>(input: Box<[T]>, shape: Box<[usize]>) -> Box<[Tensor<T>]> {
    

    input.iter().map(|val| Tensor {
        inner: Box::new([val.clone()]),
        shape: shape.clone()
    }).collect::<Vec<_>>().into_boxed_slice()
    
}   