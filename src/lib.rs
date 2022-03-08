use String;

pub struct Hero {
    _name: String,
    _alignment: Alignment
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            _name: String::new(),
            _alignment: Alignment::Neutral
        }
    }

    pub fn name(&self) -> &String {
        return &self._name;
    }

    pub fn set_name(&mut self, name: String) {
        self._name = name;
    }

    pub fn alignment(&self) -> &Alignment {
        return &self._alignment;
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        self._alignment = alignment;
    }

    pub fn armor_class(&self) -> u32 {
        return 10;
    }

    pub fn hit_points(&self) -> i32 {
        return 5;
    }
}

pub enum Alignment { Good, Neutral, Evil }

pub struct Attack {
    _attacker: Hero,
    _defender: Hero
}

impl Attack {
    pub fn between(attacker: Hero, defender: Hero) -> Attack {
        Attack {
            _attacker: attacker,
            _defender: defender
        }
    }

    pub fn resolve(&self, roll: u32) -> bool {
        let hit = roll >= self._defender.armor_class();
        return hit;
    }
}
