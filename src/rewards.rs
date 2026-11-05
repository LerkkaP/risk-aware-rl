use std::collections::HashMap;

pub fn get_rewards_original() -> HashMap<(u32, u32, u32), i32> {
    HashMap::from([
        ((0, 0, 1), 0),
        ((0, 1, 2), 0),
        ((0, 2, 3), 0),

        ((1, 0, 3), 0),
        ((1, 0, 4), -5),
        ((1, 1, 3), 12),
        ((1, 1, 4), -10),

        ((2, 0, 3), 0),
        ((2, 0, 5), -2),
        ((2, 1, 3), 4),
        ((2, 1, 5), -3),

        ((3, 0, 4), 4),
        ((3, 0, 5), 2),
        ((3, 1, 4), 5),
        ((3, 1, 5), 1),
    ])
}

pub fn penalize_loss(scale: i32) -> HashMap<(u32, u32, u32), i32> {
    let mut rewards_original = get_rewards_original();

    let loss_penalized:  HashMap<(u32, u32, u32), i32> = rewards_original
        .into_iter()
        .map(|(k, v)| {
            let transformed_value = if v < 0 {
                v * scale
            } else {
                v
            };

            (k, transformed_value)
        })
        .collect();

    loss_penalized
}


pub fn amplify_gain(scale: i32) -> HashMap<(u32, u32, u32), i32> {
    let mut rewards_original = get_rewards_original();

    let gain_amplified:  HashMap<(u32, u32, u32), i32> = rewards_original
        .into_iter()
        .map(|(k, v)| {
            let transformed_value = if v > 0 {
                v * scale
            } else {
                v
            };

            (k, transformed_value)
        })
        .collect();

    gain_amplified
}