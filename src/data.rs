use rand::Rng;

pub struct Data {
    pub sr_count: i32,
    stimulus_width: i32,
    pub stimuli: Vec<i32>,
    response_width: i32,
    pub responses: Vec<i32>,
}

impl Data {

    pub fn random(sr_count: i32, stimulus_width: i32, response_width: i32) -> Self {
        let stimuli = Self::generate_stimuli(sr_count, stimulus_width);
        let responses = Self::generate_responses(sr_count, response_width);
        Data {
            sr_count,
            stimulus_width,
            stimuli,
            response_width,
            responses,
        }
    }

    fn generate_stimuli(count: i32, stimulus_width: i32) -> Vec<i32> {
        Self::generate_random_binary_vector(count, stimulus_width)
    }

    fn generate_responses(count: i32, response_width: i32) -> Vec<i32> {
        Self::generate_random_binary_vector(count, response_width)
    }

    fn generate_random_binary_vector(count: i32, width: i32) -> Vec<i32> {
        let total_length = count * width;
        let mut rng = rand::thread_rng();
        (0..total_length).map(|_| rng.gen_range(0..2)).collect()
    }

    pub fn diff_output(&self, response_index: i32, output: Vec<i32>) -> i32 {
        let mut error = 0;
        let response_start_index = (response_index * self.response_width) as usize;
        let response_end_index = (response_start_index + self.response_width as usize) as usize;
        let mut output_index = 0;
        for response_index in response_start_index..response_end_index {
            let output_value = output[output_index];
            let response_value = self.responses[response_index];
            if output_value != response_value {
                error += 1;
            }
            output_index += 1;
        }
        error
    }

}