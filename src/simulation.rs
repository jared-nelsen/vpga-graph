
use std::collections::{VecDeque, HashMap};

use crate::encoding::Encoding;
use crate::vpga_spec::VPGASpec;
use crate::vpga::VPGA;
use crate::data::Data;

pub struct Simulation {
    vpga: VPGA,
    data: Data,
    best_encoding: Encoding
}

impl Simulation {

    pub fn new() -> Self {
        let sr_count = 10;
        let vpga_spec = VPGASpec::default();
        let vpga = VPGA::new(vpga_spec);
        let data = Data::random(
            sr_count, 
            vpga_spec.input_block_width, 
            vpga_spec.output_block_width,
        );
        let best_encoding = Encoding::new_random(vpga.get_encoding_length());
        Simulation { vpga, data, best_encoding }
    }

    pub fn run(&mut self) {
        let tabu_size = 10;
        let mut current_solution = self.best_encoding.clone();
        let mut best_solution = current_solution.clone();
        let mut tabu_list: VecDeque<Encoding> = VecDeque::new();
        let mut known_fitness_values: HashMap<Encoding, i32> = HashMap::new();

        for iteration in 0..10000 {
            println!("Iteration: {}", iteration);
            let neighbor_solution = self.get_best_neighbor(&mut current_solution, &tabu_list, &mut known_fitness_values);
    
            if tabu_list.len() > tabu_size {
                tabu_list.pop_front();
            }
            tabu_list.push_back(current_solution.clone());
    
            current_solution = neighbor_solution.clone();
    
            if neighbor_solution.fitness < best_solution.fitness {
                best_solution = neighbor_solution.clone();
                println!("New best fitness found: {}", best_solution.fitness);
            }
        }
    
        self.best_encoding = best_solution;
    }

    fn get_best_neighbor(&mut self, encoding: &Encoding, tabu_set: &VecDeque<Encoding>, known_fitness_values: &mut HashMap<Encoding, i32>) -> Encoding {
        let neighbor_generate_count = 1000;
        let mut best_neighbor = encoding.clone();
        let mut best_fitness = i32::MAX;
        
        for _ in 0..neighbor_generate_count {
            let mut neighbor = encoding.clone();
            neighbor.mutate();
            
            if tabu_set.contains(&neighbor) {
                continue;
            }
    
            let neighbor_fitness;
            if known_fitness_values.contains_key(&neighbor) {
                neighbor_fitness = *known_fitness_values.get(&neighbor).unwrap();
            } else {
                self.vpga.apply_encoding_to_vpga(&neighbor);
                self.vpga.evaluate(&self.data);
                neighbor_fitness = self.vpga.fitness;
                known_fitness_values.insert(neighbor.clone(), neighbor_fitness);
            }

            if neighbor_fitness < best_fitness {
                best_neighbor = neighbor;
                best_fitness = neighbor_fitness;
            }
        }
    
        best_neighbor.fitness = best_fitness;
        best_neighbor
    }
    
}