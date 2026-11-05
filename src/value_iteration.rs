use std::collections::HashMap;
use crate::mdp;

pub fn optimal_value_function(mdp: &mdp::MDP) -> Vec<f64> {
    let mut values: Vec<f64> = vec![0.0; 6];
    let states = [0, 1, 2, 3, 4, 5];
    let discount_factor = 0.95;
    let threshold = 0.001;

    loop {
        let mut delta: f64 = 0.0;
        for (i, s) in states.iter().enumerate() {
            let v = values[i];
            let valid_actions = mdp.valid_actions(*s);
            let mut max_sum_over_actions = 0.0;

            for a in valid_actions {
                let mut sum_over_next_states = 0.0;

                for (next_state, probability) in mdp.t(*s, *a).into_iter() {
                    let r = mdp.r(*s, *a, *next_state);
                    let value_next_state = values[*next_state as usize];
                    sum_over_next_states += probability * (r as f64 + discount_factor * value_next_state);

                }
                if sum_over_next_states >= max_sum_over_actions {
                    max_sum_over_actions = sum_over_next_states;
                }
            }
            values[i] = max_sum_over_actions;
            let difference_abs = (v - max_sum_over_actions).abs();
            delta = delta.max(difference_abs);
        }
        if delta < threshold {
            break;
        } 
    }

    values
    
}

pub fn optimal_policy(mdp: mdp::MDP, optimal_value_function: Vec<f64>) -> HashMap<u32, u32> {
    let mut result: HashMap<u32, u32> = HashMap::new();
    let states = [0, 1, 2, 3];
    let discount_factor = 0.95;

    for (i, s) in states.iter().enumerate() {
        let valid_actions = mdp.valid_actions(*s);
        let mut max_sum_over_actions = 0.0;
        let mut best_action = 0;

        for a in valid_actions {
            let mut sum_over_next_states = 0.0;

            for (next_state, probability) in mdp.t(*s, *a).into_iter() {
                let r = mdp.r(*s, *a, *next_state);
                let value_next_state = optimal_value_function[*next_state as usize];
                sum_over_next_states += probability * (r as f64 + discount_factor * value_next_state);
            }
            if sum_over_next_states >= max_sum_over_actions {
                max_sum_over_actions = sum_over_next_states;
                best_action = *a;
            }
        }
        result.insert(*s, best_action);
    }
  
    result
}