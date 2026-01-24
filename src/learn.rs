use std::process::{Command, Stdio};

fn calculate_bmi(height: f32, weight: f32) -> f32 {
    let result = weight / height.powi(2);
    (result * 2.0).round() / 2.0
}

#[test]
fn test_calculate_bmi() {
    assert_eq!(calculate_bmi(1.7, 65.0), 22.5);
}

pub fn learn() {}
