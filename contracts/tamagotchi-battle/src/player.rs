use crate::{
    utils::{generate_power, get_random_value},
    Move, Pair, Player,
};
use gstd::{exec, prelude::*, ActorId};
pub trait PlayerFunc {
    fn update_structure(&mut self, min_power: u16, max_power: u16, winner: bool);
    fn decrease_health(&mut self, damage: u16);
    fn decrease_power(&self, opponent_defence: u16) -> u16;
}

impl PlayerFunc for Player {
    fn update_structure(&mut self, min_power: u16, max_power: u16, winner: bool) {
        if winner {
            self.victories = self.victories.saturating_add(1);
        }
        self.power = generate_power(min_power, max_power, self.tmg_id);
        self.defence = max_power - self.power;
    }
    fn decrease_health(&mut self, damage: u16) {
        self.health = self.health.saturating_sub(damage);
    }
    fn decrease_power(&self, opponent_defence: u16) -> u16 {
        self.power.saturating_sub(opponent_defence)
    }
}