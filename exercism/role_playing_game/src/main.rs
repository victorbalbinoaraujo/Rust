#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // If health == 0 and lvl >= 10 create a new player otherwise none
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // If player doesn't have mana pool reduce health by mana cost of the spell | damage = 0
        // If player has mana pool but not enough mana | damage = 0
        // Else mana cost deducted from mana pool | damage 2 * mana cost
        let mut damage: u32 = 0;

        match self.mana {
            None => self.health = self.health.saturating_sub(mana_cost),
            Some(mana) => {
                if mana_cost <= mana {
                    damage = mana_cost * 2;
                    self.mana = Some(mana.saturating_sub(mana_cost));
                }
            }
        }
        damage
    }
}

fn main() {
    let mut not_a_wizard_yet: Player = Player {
        health: 79,
        mana: None,
        level: 9,
    };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
    assert_eq!(not_a_wizard_yet.mana, None);

    let mut low_mana_wizard: Player = Player {
        health: 93,
        mana: Some(3),
        level: 12,
    };
    assert_eq!(low_mana_wizard.cast_spell(10), 0);
    assert_eq!(low_mana_wizard.health, 93);
    assert_eq!(low_mana_wizard.mana, Some(3));

    let mut wizard: Player = Player {
        health: 123,
        mana: Some(30),
        level: 18,
    };
    assert_eq!(wizard.cast_spell(10), 20);
    assert_eq!(wizard.health, 123);
    assert_eq!(wizard.mana, Some(20));
}
