     // todo : create firearm using enums and traits
// todo : create model of the gun --finished

// todo : caliber -- finished
// todo magazine size -- finished
// todo: rate of fire 
// todo : allow user to check ammo count 

//todo : create a function to fire the gun and decrease ammo count
//todo : create a function to reload the gun and reset ammo count
// Caliber enum - can't use strings as variant names, use words instead

// todo : create funttions for all the models of guns to stop the warnings in complier.
enum Caliber {
    Mm762,   // 7.62mm
    Mm556,   // 5.56mm
    Mm9,     // 9mm
}

impl Caliber {
    fn damage(&self) -> u8 {
        match self {
            Caliber::Mm762 => 25,
            Caliber::Mm556 => 20,
            Caliber::Mm9 => 10,
        }
    }
}

enum Enemy {
    Testdummy { health: u8 },
}

impl Enemy {
    fn take_damage(&mut self, damage: u8) {
        match self {
            Enemy::Testdummy { health } => {
                *health = health.saturating_sub(damage);
                println!("Enemy took {} damage! Health remaining: {}", damage, health);
            }
        }
    }

    fn is_alive(&self) -> bool {
        match self {
            Enemy::Testdummy { health } => *health > 0,
        }
    }
}

enum Model {
    Ak47 { caliber: Caliber, magazine_size: u32, rate_of_fire: u32, ammo_count: u32 },
    M4   { caliber: Caliber, magazine_size: u32, rate_of_fire: u32, ammo_count: u32 },
    Glock{ caliber: Caliber, magazine_size: u32, rate_of_fire: u32, ammo_count: u32 },
}

trait Firearm {
    fn fire(&mut self, enemy: &mut Enemy);
    fn full_auto(&mut self, enemy: &mut Enemy);
    fn reload(&mut self);
    fn check_ammo(&self);
}

impl Firearm for Model {
    fn fire(&mut self, enemy: &mut Enemy) {
        match self {
            Model::Ak47 { caliber, ammo_count, .. } |
            Model::M4   { caliber, ammo_count, .. } |
            Model::Glock{ caliber, ammo_count, .. } => {
                if *ammo_count > 0 {
                    *ammo_count -= 1;
                    if enemy.is_alive() {
                        enemy.take_damage(caliber.damage());
                    } else {
                        println!("Enemy is already defeated!");
                    }
                } else {
                    println!("Out of ammo! Please reload.");
                }
            }
        }
    }

    fn full_auto(&mut self, enemy: &mut Enemy) {
        match self {
            Model::Ak47 { caliber, ammo_count, rate_of_fire, .. } |
            Model::M4   { caliber, ammo_count, rate_of_fire, .. } |
            Model::Glock{ caliber, ammo_count, rate_of_fire, .. } => {
                if *ammo_count > 0 {
                    let shots = 7;
                    *ammo_count = ammo_count.saturating_sub(7);

                // Higher rate of fire = less accurate = lower damage %
                // e.g. rate_of_fire 600 -> 40% accuracy, 300 -> 70% accuracy
                    let variance = rand::random::<u32>() % 21; // 0 to 20
                    let accuracy = (100u32.saturating_sub(*rate_of_fire / 10) + variance).saturating_sub(10).max(30);
                    let raw_damage = caliber.damage() as u32 * shots;
                    let total_damage = (raw_damage * accuracy / 100) as u8;

                if enemy.is_alive() {
                    enemy.take_damage(total_damage.try_into().unwrap());
                    println!(
                        "Full auto! Fired {} shots. Accuracy: {}%. Damage dealt: {}",
                        shots, accuracy, total_damage
                    );
                } else {
                    println!("Enemy is already defeated!");
                }
            } else {
                println!("Out of ammo! Please reload.");
            }
        }
    }
}

    fn reload(&mut self) {
        match self {
            Model::Ak47 { magazine_size, ammo_count, .. } |
            Model::M4   { magazine_size, ammo_count, .. } |
            Model::Glock{ magazine_size, ammo_count, .. } => {
                *ammo_count = *magazine_size;
                println!("Reloaded! Ammo count: {}", ammo_count);
            }
        }
    }

    fn check_ammo(&self) {
        match self {
            Model::Ak47 { ammo_count, .. } => println!("Ak47 ammo: {}", ammo_count),
            Model::M4   { ammo_count, .. } => println!("M4 ammo: {}", ammo_count),
            Model::Glock{ ammo_count, .. } => println!("Glock ammo: {}", ammo_count),
        }
    }
}

fn main() {
    let mut ak47 = Model::Ak47 {
        caliber: Caliber::Mm762,
        magazine_size: 30,
        rate_of_fire: 600,
        ammo_count: 30,
    };

    let mut enemy = Enemy::Testdummy { health: 100 };

    ak47.fire(&mut enemy);
    ak47.check_ammo();
    ak47.full_auto(&mut enemy);
    ak47.check_ammo();
    ak47.reload();
    ak47.check_ammo();
}