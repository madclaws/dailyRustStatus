// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mut new_health: u32 = self.health;
        let mut new_mana: Option<u32> = self.mana;
        if self.health == 0 {
            new_health = 100;
        } else {
            return None;
        }

        if self.level >= 10 {
            new_mana = Some(100);
        } else {
            new_mana = None;
        }
        Some(Player{health: new_health, mana: new_mana, level: self.level})
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = i32::max(0, (self.health as i32 - mana_cost as i32)) as u32;
                0
            },
            Some(mana_pool) => if mana_pool < mana_cost {
                0
            } else {
                self.mana = Some(mana_pool - mana_cost);
                mana_cost * 2
            }
        }
    }
}
