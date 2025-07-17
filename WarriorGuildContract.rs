#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

#[contract]
pub struct WarriorGuildContract;

#[contractimpl]
impl WarriorGuildContract {
    pub fn welcome_warrior(env: Env, warrior_name: Symbol) {
        log!(&env, "Welcome, Warrior {}!", warrior_name);
    }
}
