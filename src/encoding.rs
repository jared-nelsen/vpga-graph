

#[derive(Hash, Eq, Clone, PartialEq, PartialOrd)]
pub struct Encoding {
    pub fitness: i32,
    width: i32,
    pub encoding: Vec<i32>,
}

impl Encoding {

    pub fn new_random(width: i32) -> Self {
        let encoding: Vec<i32> = (0..width).map(|_| if rand::random() { 1 } else { 0 }).collect();
        Encoding {
            fitness: i32::MAX,
            width,
            encoding,
        }
    }

    pub fn mutate(&mut self) {
        let mutation_type = rand::random::<f32>();
        
        if mutation_type < 0.33 {
            // Single bit flip
            let idx = rand::random::<usize>() % self.width as usize;
            self.encoding[idx] = if self.encoding[idx] == 0 { 1 } else { 0 };
        } else if mutation_type < 0.66 {
            // Swap mutation
            let idx1 = rand::random::<usize>() % self.width as usize;
            let idx2 = rand::random::<usize>() % self.width as usize;
            self.encoding.swap(idx1, idx2);
        } else {
            // Inversion mutation
            let start = rand::random::<usize>() % self.width as usize;
            let end = rand::random::<usize>() % self.width as usize;
            if start < end {
                self.encoding[start..=end].reverse();
            } else {
                self.encoding[end..=start].reverse();
            }
        }
    }

}