use std::collections::HashMap;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

pub struct MDP {
    pub terminal_states: [i32; 2],
    pub rewards: HashMap<(i32, i32, i32), i32>,
    pub transitions: HashMap<(i32, i32), HashMap<i32, f64>>,
    pub actlist: HashMap<i32, Vec<i32>>
}

impl MDP {
    pub fn r(&self, state: i32, action: i32, next_state: i32) -> i32 {
        *self
        .rewards.get(&(state, action, next_state))
        .expect("Reward not found.")
    } 

    pub fn t(&self, state: i32, action: i32) -> &HashMap<i32, f64> {
        self
        .transitions.get(&(state, action))
        .expect("Transition not found.")
    }

    pub fn valid_actions(&self, state: i32) -> &[i32] {
        self.actlist.get(&state)
        .expect("No valid actions.") 
    }

    pub fn step(&self, state: i32, action: i32) -> (i32, i32, bool) {
        let transitions = self.t(state, action);

        let next_states: Vec<i32> = transitions.keys().map(|k| k.to_owned()).collect();
        let probabilities: Vec<f64> = transitions.values().map(|v| v.to_owned()).collect();

        let dist = WeightedIndex::new(&probabilities).unwrap();
        let mut rng = rand::rng();

        let random_index = dist.sample(&mut rng);

        let next_state = next_states[random_index];

        println!("{}", next_state);

        let reward = self.r(state, action, next_state);
        let terminated = if self.terminal_states.contains(&next_state) {
            true
        } else {
            false
        };

        (next_state, reward, terminated)
    }
}