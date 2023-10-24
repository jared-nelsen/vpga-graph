use rand::Rng;

pub struct Data {
    sr_count: i32,
    stimulus_width: i32,
    stimuli: Vec<i32>,
    response_width: i32,
    responses: Vec<i32>,
}

impl Data {

    pub fn random(sr_count: i32, stimulus_width: i32, response_width: i32) -> Self {
        let stimuli = Self::generate_stimuli(sr_count);
        let responses = Self::generate_responses(sr_count);
        Data {
            sr_count,
            stimulus_width,
            stimuli,
            response_width,
            responses,
        }
    }

    fn generate_stimuli(count: i32) -> Vec<i32> {
        Self::generate_random_binary_vector(count)
    }

    fn generate_responses(count: i32) -> Vec<i32> {
        Self::generate_random_binary_vector(count)
    }

    fn generate_random_binary_vector(count: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..count).map(|_| rng.gen_range(0..2)).collect()
    }
    
}