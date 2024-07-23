use std::io::{self, BufRead};
use rand::Rng;
use std::process;
use std::thread::sleep;
use std::time::Duration;

mod structs;
use structs::{PersonType, Person};

// Fight between 2 persons
fn fight(hero: &mut Person, monster: &mut Person, score: &mut u8) {
    println!("Вы идете по району и к вам подкатывает какое-то {:?}", monster.kind);
    sleep(Duration::from_secs(1));
    println!("Началась драка! {:?} против {:?}", hero, monster);
    sleep(Duration::from_secs(1));

    let mut rng = rand::thread_rng();  // Create a random number generator
    loop {
        if "f" == get_command("Нажмите f чтобы драться") {
            print("Удар!");

            // Choosing
            match rng.gen_range(0..2) {
                0 => {
                    println!("{:?} бьет {:?} на {}", hero.kind, monster.kind, hero.attack);
                    hit(hero, monster);
                },
                1 => {
                    println!("{:?} бьет {:?} на {}", monster.kind, hero.kind, monster.attack);
                    hit(monster,hero);
                }
                _ => println!("???")
            }
            sleep(Duration::from_secs(1));

            //println!("Монстр жизней {} {:?} {:?}", monster.health, monster, hero);
            // Checking health and ending program if our hero is died
            if 0 == monster.health {
                *score += 1;
                println!("{:?} подох", monster.kind);
                break
            } else if 0 == hero.health {
                match score {
                    0 => println!("Ты жалко подох никого не убив"),
                    _ => println!("Ты гордо подох убив {} хуил", score)
                }
                sleep(Duration::from_secs(50));
                process::exit(0);
            }
        }
    }
}

// Hitting
fn hit(hitter: &mut Person, hitted: &mut Person) {
    if hitter.attack > hitted.health {
        hitted.health = 0;
    } else {
        hitted.health -= hitter.attack
    }
    println!("Лог после удара: {:?} {:?}", hitter, hitted)
    //println!("{:?} бьет {:?} на {}", hitter.kind, hitted.kind, hitter.health)
}

fn get_command(text: &str) -> String {
    println!("{text}");
    let mut command: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut command).unwrap();
    command.trim().to_string()
}

fn print(text: &str) {
    println!("{text}");
    sleep(Duration::from_secs(1));
}

fn main() {
    let mut hero: Option<Person> = None;
    let mut score: u8 = 0;

    loop {
        if hero.is_none() {
            let command = get_command("Кем вы хотите играть? 1 - Крутяк, 2 - Дохляк");

            // println!(command);
            if "1" == command {
                hero = Some(Person {
                    kind: PersonType::Krutyak,
                    health: 100,
                    attack: 100,
                });
                print("Крутой выбор!");
            } else if "2" == command {
                hero = Some(Person {
                    kind: PersonType::Dohlyak,
                    health: 1,
                    attack: 1,
                });
                print("Тупой выбор. Но ладно...")
            }
        }

        if hero.is_some() {
            let mut monster = Person {
                kind: PersonType::Huilo,
                health: 50,
                attack: 50,
            };

            if let Some(ref mut h) = hero {
                fight(h, &mut monster, &mut score);
            }
            // fight(&hero.unwrap(), &monster)
        }
  
        //println!("{}", line)
    }
}