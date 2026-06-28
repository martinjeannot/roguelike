use crate::components::*;
use specs::{Entity, Join, System, World, WorldExt, WriteStorage};

pub struct DamageSystem;

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, CombatStats>, WriteStorage<'a, DamageTaker>);

    fn run(&mut self, (mut combat_stats, mut damage_takers): Self::SystemData) {
        for (stats, damage) in (&mut combat_stats, &damage_takers).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }
        damage_takers.clear();
    }
}

pub fn remove_corpses(ecs: &mut World) {
    let mut corpses: Vec<Entity> = Vec::new();
    // scoping the ecs-borrowed entities smart pointer to drop the immutable borrow of ecs before we
    // try to mutably borrow it to delete the dead entities
    {
        let entities = ecs.entities();
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp <= 0 {
                let player = players.get(entity);
                match player {
                    Some(_) => println!("Player is dead"),
                    None => corpses.push(entity),
                }
            }
        }
    }
    for corpse in corpses {
        ecs.delete_entity(corpse).expect("Failed to delete entity");
    }
}
