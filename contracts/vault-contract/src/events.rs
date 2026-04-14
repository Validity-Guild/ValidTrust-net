#![no_std]

use soroban_sdk::{Env, Address, Symbol, symbol_short};

pub fn emit_deposit(env: &Env, user: &Address, amount: i128) {
    let topics = (symbol_short!("deposit"), user);
    env.events().publish(topics, amount);
}

pub fn emit_withdraw(env: &Env, user: &Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), user);
    env.events().publish(topics, amount);
}

pub fn emit_reward(env: &Env, user: &Address, amount: i128) {
    let topics = (symbol_short!("reward"), user);
    env.events().publish(topics, amount);
}

// TODO: Add an event for administrative configuration updates (e.g., setting a new reward rate)
