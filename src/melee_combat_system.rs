use crate::components::*;
use specs::{Join, ReadStorage, System, WriteStorage};
use std::cmp::max;

pub struct MeleeCombatSystem;

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        WriteStorage<'a, DamageDealer>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, DamageTaker>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut damage_dealers, names, combat_stats, mut damage_takers) = data;

        for (damage_dealer, dealer_name, dealer_stats) in
            (&damage_dealers, &names, &combat_stats).join()
        {
            if dealer_stats.hp <= 0 {
                // already dead, cannot deal damage
                continue;
            }
            let target_stats = combat_stats.get(damage_dealer.target).unwrap();
            if target_stats.hp <= 0 {
                // already dead, cannot take further damage
                continue;
            }
            let target_name = names.get(damage_dealer.target).unwrap();
            let damage_taken = max(0, dealer_stats.power - target_stats.defense);
            if damage_taken > 0 {
                println!(
                    "{} hits {} for {} hp.",
                    dealer_name.0, target_name.0, damage_taken
                );
                DamageTaker::take_damage(&mut damage_takers, damage_dealer.target, damage_taken);
            } else {
                println!("{} is unable to hurt {}", dealer_name.0, target_name.0);
            }
        }

        damage_dealers.clear();
    }
}
