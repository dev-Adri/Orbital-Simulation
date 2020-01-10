
use crate::libm::atan2f;
use crate::libm::{cos, sin};

use std::num::*;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct CelestialObject {
    posx: f64,
    posy: f64,
    mass: f64,
    speed: f64,
    acceleration: f64
}

impl CelestialObject {
    pub fn new(pos_x: f64, pos_y: f64, mass: f64, speed: f64, acceleration: f64) -> Self {
        CelestialObject {
            posx: pos_x,
            posy: pos_y,
            mass: mass,
            speed: speed,
            acceleration: acceleration
    }
}

// ! v = sqrt(G * M/r) -> orbital velocity
// ! a = v^2/r -> orbital acceleration --- OR ---  a = Fnet / m
// ! T = sqrt(4 * π^2 * R^3/ G * m) -> Orbital period
// ! v = ΔX / Δt = 2piR / T

// todo: Rewrite the equations, because they are completely, utterly wrong
// todo: Rewrite the orbit function, with the new equations and hopefuly make it work

const G: f64 = 6.67e-11;
const PI: f64 = 3.1415;

pub fn orbit(planet: &mut CelestialObject, star: &mut CelestialObject) {
    // Speed up the simulation
    let mut time: f64 = 86400.0 * 7.0;
    // G as f32et the distance between planet and star
    let mut distance: f64 = ((&planet.pos_x - &star.pos_x).powi(2) + (&planet.pos_y - &star.pos_y).powi(2)).sqrt();
    // Speed 
    let mut orbital_velocity: f64 = (G * &planet.mass / &distance).sqrt() * time;
    // Net force
    let force: f64 = (G * (&planet.mass * &star.mass) / (&distance.powi(2))) * time;
    // Acceleration
    let mut orbital_acceleration: f64 = (force / planet.mass) * time;
    //let mut period = (4.0 * PI.powi(2) * distance.powi(3)) / (G * &planet.mass);
    //let mut speed = 2.0 * PI * distance / period;

    // A matrix that rotates vectors in 2D space
    //let mut rotation_matrix: Vec<f64> = vec![cos(i), sin(i), -sin(i), cos(i)];
    
    *&mut orbital_velocity += &orbital_acceleration * &force;
    //println!("Force {}\nVelocity {}\nAcceleration {}\n", &force, &orbital_velocity, &orbital_acceleration);

    planet.pos_x -= cos(orbital_velocity);
    planet.pos_y -= sin(orbital_velocity) * 2.0;

    println!("Pos x: {}\nPos y: {}", planet.pos_x, planet.pos_y);
}
