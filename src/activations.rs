// src/activations.rs
use neat::{
    ActivationFn,
    activation::{ActivationScope, batch_register_activation},
    activation_fn,
};
use std::f32::consts::E;
use std::sync::Arc;

/// Sine activation function for periodic patterns.
pub fn sin_activation(n: f32) -> f32 {
    n.sin()
}

/// Cosine activation function for periodic patterns.
pub fn cos_activation(n: f32) -> f32 {
    n.cos()
}

/// Gaussian activation function for smooth bumps.
pub fn gaussian_activation(n: f32) -> f32 {
    // A simplified Gaussian function: e^(-n^2)
    E.powf(-n.powi(2))
}

/// Absolute value activation function for sharp corners and mirrored patterns.
pub fn abs_activation(n: f32) -> f32 {
    n.abs()
}

/// Square activation function for strong non-linear curves.
pub fn square_activation(n: f32) -> f32 {
    n * n
}

/// Heaviside step function. Output is 1.0 if input > 0, else 0.0.
pub fn step_activation(n: f32) -> f32 {
    if n > 0.0 { 1.0 } else { 0.0 }
}

/// Clamps the input between -1.0 and 1.0.
pub fn clamp_activation(n: f32) -> f32 {
    n.clamp(-1.0, 1.0)
}

/// Creates a repeating square wave pattern.
pub fn pulse_activation(n: f32) -> f32 {
    if n.fract() > 0.5 { 1.0 } else { 0.0 }
}

/// Quantizes the input into 5 discrete steps (-1.0, -0.5, 0.0, 0.5, 1.0).
pub fn staircase_activation(n: f32) -> f32 {
    (n.clamp(-1.0, 1.0) * 2.0).round() / 2.0
}

/// This function will be called once at startup to register our new functions.
pub fn register_custom_activations() {
    println!("Registering custom activation functions...");
    batch_register_activation(activation_fn! {
        sin_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        cos_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        gaussian_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        abs_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        square_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        step_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        clamp_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        pulse_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        staircase_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT
    });
    println!("Custom activation functions registered.");
}
