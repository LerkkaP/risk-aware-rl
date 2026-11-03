use std::collections::HashMap;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

pub struct MDP {
    pub terminal_states: [u32; 2],
    pub rewards: HashMap<(u32, u32, u32), i32>,
    pub transitions: HashMap<(u32, u32), HashMap<u32, f64>>,
    pub actlist: HashMap<u32, Vec<u32>>
}

impl MDP {
    pub fn r(&self, state: u32, action: u32, next_state: u32) -> i32 {
        *self
        .rewards.get(&(state, action, next_state))
        .expect("Reward not found.")
    } 

    pub fn t(&self, state: u32, action: u32) -> &HashMap<u32, f64> {
        self
        .transitions.get(&(state, action))
        .expect("Transition not found.")
    }

    pub fn valid_actions(&self, state: u32) -> &[u32] {
        self.actlist.get(&state)
        .expect("No valid actions.") 
    }

    pub fn step(&self, state: u32, action: u32) -> (u32, i32, bool) {
        let transitions = self.t(state, action);

        let next_states: Vec<u32> = transitions.keys().map(|k| k.to_owned()).collect();
        let probabilities: Vec<f64> = transitions.values().map(|v| v.to_owned()).collect();

        let next_state = self.sample_next_state(&next_states, &probabilities);

        let reward = self.r(state, action, next_state);
        
        let terminated = if self.terminal_states.contains(&next_state) {
            true
        } else {
            false
        };

        (next_state, reward, terminated)
    }
    
    pub fn sample_next_state(&self, next_states: &Vec<u32>, probabilities: &Vec<f64>) -> u32 {
        let dist = WeightedIndex::new(probabilities).unwrap();
        let mut rng = rand::rng();

        let random_index = dist.sample(&mut rng);

        next_states[random_index]
    }
}