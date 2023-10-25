

#[derive(Hash, Eq, Clone, PartialEq, PartialOrd)]
pub struct Encoding {
    pub fitness: i32,
    width: i32,
    pub encoding: Vec<i32>,
}

impl Encoding {

    pub fn new_random(width: i32) -> Self {
        let encoding: Vec<i32> = (0..width).map(|_| rand::random::<i32>() % 2).collect();
        Encoding {
            fitness: i32::MAX,
            width,
            encoding,
        }
    }

    pub fn mutate(&mut self) {
        let idx = rand::random::<usize>() % self.width as usize;
        self.encoding[idx] = 1 - self.encoding[idx];
    }

}