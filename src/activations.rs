// src/activations.rs
use neat::{activation_fn, activation::{batch_register_activation, ActivationScope}, ActivationFn};
use std::sync::Arc;
use std::f32::consts::E;

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

/// This function will be called once at startup to register our new functions.
pub fn register_custom_activations() {
    println!("Registering custom activation functions...");
    batch_register_activation(activation_fn! {
        sin_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        cos_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        gaussian_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        abs_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT,
        square_activation => ActivationScope::HIDDEN | ActivationScope::OUTPUT
    });
    println!("Custom activation functions registered.");
}