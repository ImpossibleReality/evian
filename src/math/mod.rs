pub mod vec2;
pub mod pursuit;

pub use vec2::Vec2;
pub use pursuit::*;

use core::f64::consts::{FRAC_2_PI, PI};
use num_traits::real::Real;

/// Constrain an angle in radians from -π to +π.
///
/// This preserves the angle's direction while keeping it within minimum constrains,
/// allowing certain operations to be performed easier.
pub fn normalize_angle(mut angle: f64) -> f64 {
    angle = angle % FRAC_2_PI;
    angle = (angle + FRAC_2_PI) % FRAC_2_PI;

    if angle > PI {
        angle -= FRAC_2_PI;
    }

    angle
}

pub fn normalize_motor_voltages(mut voltages: (f64, f64), max_voltage: f64) -> (f64, f64) {
    let largest_voltage = voltages.0.abs().max(voltages.1.abs())/ max_voltage;

    if largest_voltage > 1.0 {
        voltages.0 /= largest_voltage;
        voltages.1 /= largest_voltage;
    }

    return voltages;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize() {
        assert_eq!(normalize_angle(FRAC_2_PI), 0.0);
    }
}
