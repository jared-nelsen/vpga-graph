
use std::collections::{HashSet, VecDeque};

use crate::encoding::Encoding;
use crate::vpga_spec::VPGASpec;
use crate::vpga::VPGA;
use crate::data::Data;

pub struct Simulation {
    sr_count: i32,
    vpga_spec: VPGASpec,
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
        Simulation { sr_count, vpga_spec, vpga, data, best_encoding }
    }

    pub fn run(&mut self) {
        let tabu_size = 10;
        let mut current_solution = self.best_encoding.clone();
        let mut best_solution = current_solution.clone();
        let mut tabu_list: VecDeque<Encoding> = VecDeque::new();

        for _ in 0..1000 {
            let neighbor_solution = self.get_best_neighbor(&mut current_solution, &tabu_list);
    
            if tabu_list.len() > tabu_size {
                tabu_list.pop_front();
            }
            tabu_list.push_back(current_solution.clone());
    
            current_solution = neighbor_solution.clone();
    
            if neighbor_solution.fitness < best_solution.fitness {
                best_solution = neighbor_solution.clone();
            }
        }
    
        self.best_encoding = best_solution;
    }

    fn get_best_neighbor(&mut self, encoding: &Encoding, tabu_set: &VecDeque<Encoding>) -> Encoding {
        let neighbor_generate_count = 100;
        let mut best_neighbor = encoding.clone();
        let mut best_fitness = i32::MAX;
        
        for _ in 0..neighbor_generate_count {
            let mut neighbor = encoding.clone();
            neighbor.mutate();
            
            if tabu_set.contains(&neighbor) {
                continue;
            }
    
            self.vpga.apply_encoding_to_vpga(&neighbor);
            self.vpga.evaluate(&self.data);
    
            let neighbor_fitness = self.vpga.fitness;
    
            if neighbor_fitness < best_fitness {
                best_neighbor = neighbor.clone();
                best_fitness = neighbor_fitness;
            }
        }
    
        best_neighbor.fitness = best_fitness;
        best_neighbor
    }
    
}