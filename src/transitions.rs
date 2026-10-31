use std::collections::HashMap;

pub fn get_transitions() -> HashMap<(i32, i32), HashMap<i32, f64>> {
    HashMap::from([
        ((0, 0), HashMap::from([(1, 1.0)])),
        ((0, 1), HashMap::from([(2, 1.0)])),
        ((0, 2), HashMap::from([(3, 1.0)])),
        ((1, 0), HashMap::from([(3, 0.8), (4, 0.2)])),
        ((1, 1), HashMap::from([(3, 0.6), (4, 0.4)])),
        ((2, 0), HashMap::from([(3, 0.8), (5, 0.2)])),
        ((2, 1), HashMap::from([(3, 0.7), (5, 0.3)])),
        ((3, 0), HashMap::from([(4, 0.9), (5, 0.1)])),
        ((3, 1), HashMap::from([(4, 0.8), (5, 0.2)])),
    ])
}