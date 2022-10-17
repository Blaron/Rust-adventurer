use rand::Rng;
use std::io;
use std::io::prelude::*;
use colored::*;

 //Player class type:
 enum Pclass {
     MAGE,
     WARRIOR,
     ARCHER,
 }

// layer structure :
#[allow(dead_code)]
struct Player {
    name: String,
    ataque: i32,
    defensa: i32,
    max_vida: i32,
    vida: i32,
    level: u16,
    clase: Pclass,
}

#[allow(unused_variables)]
fn main(){
    
    let mut hero_name:String = String::new();
    #[allow(unused_assignments)]
    let mut hero_class:String = String::new();
    println!("{}","Welcome to Rust adventure".cyan());
    println!("Enter the name of the {}","Hero".red());
    let b1_name = std::io::stdin().read_line(&mut hero_name).unwrap();
    println!();
    use terminal_menu::{menu,label,button,run,mut_menu};
    let menu = menu(vec![
        label(""),
        label("Select the class for the Hero"),
        label("----------------------"),
        label("Use wasd or arrow keys"),
        label("[Enter] To select"),
        label("[q] or [esc] to exit"),
        label(""),
        button("Warrior"),
        button("Mage"),
        button("Archer"),
    ]);
    run(&menu);
    hero_class = mut_menu(&menu).selected_item_name().to_string();
    // println!("Perfecto {}. ¿ Qué clase eres ?",hero_class);
    // let b2_clase:usize = std::io::stdin().read_line(&mut hero_class).unwrap();
    println!("Get ready for the adventure, Look your current stats:");
    #[allow(unused_mut)]
    let mut jugador = Player{
        name:hero_name,
        ataque: {
            match hero_class.as_str() {
                "Warrior" => 2,
                "Mage" => 5,
                "Archer" => 3,
                _ => 1,
            }
        },
        defensa: {
            match hero_class.as_str() {
                "Warrior" => 5,
                "Mage" => 1,
                "Archer" => 2,
                _ => 1,
            }
        },
        max_vida: {
            match hero_class.as_str() {
                "Warrior" => 6,
                "Mage" => 3,
                "Archer" => 3,
                _ => 1,
            }
        },
        vida: {
            match hero_class.as_str() {
                "Warrior" => 6,
                "Mage" => 3,
                "Archer" => 3,
                _ => 1,
            }
        },
        level: 1,
        clase: get_class(hero_class),
    };
    println!("{}{}","attack: ".cyan(),jugador.ataque);
    println!("{}{}","defense: ".cyan(),jugador.defensa);
    println!("{}{}","Max Health: ".cyan(),jugador.max_vida);
    println!("{}{}","Health: ".cyan(),jugador.vida);
    println!("{}{}","Lvl: ".cyan(),jugador.level);
    println!("{}{}","Class: ".cyan(),mut_menu(&menu).selected_item_name().to_string());
    pause();
    println!("{}{}","Now go to fight ".bold().red(),jugador.name.yellow());
    println!();
    play_run(jugador);
    

}

 fn get_class(tipe: String) -> Pclass{
     match tipe.as_str() {
         "Warrior"=>Pclass::WARRIOR,
         "Mage"=>Pclass::MAGE,
         "Archer"=>Pclass::ARCHER,
         _ =>Pclass::WARRIOR,
     }
 }


// UPDATE STATS
#[allow(dead_code)]
fn add_life(current_life:i16) -> i16 {
    let current : i16 = current_life;
    let add : i16 = 1;
    return current + add;
}
#[allow(dead_code)]
fn remove_life(vida_actual:i32, hit:i32) -> i32 {
    let current : i32 = vida_actual;
    let remove : i32 = hit;
    return current - remove;
}
#[allow(dead_code)]
fn add_max_life(current_life:i32) -> i32 {
    let current : i32 = current_life;
    let add : i32 = 1;
    return current + add;
}
#[allow(dead_code)]
fn add_attack(current_attack:i32,set_attack:i32) -> i32 {
    let current : i32 = current_attack;
    let add : i32 = set_attack;
    return current + add;
}
#[allow(dead_code)]
fn add_defense(current_defense:i32,set_defense:i32) -> i32 {
    let current : i32 = current_defense;
    let add : i32 = set_defense;
    return current + add;
}
#[allow(dead_code)]
fn add_level(current_level:u16,set_level:u16) -> u16 {
    let current : u16 = current_level;
    let add : u16 = set_level;
    return current + add;
}

fn end_run(jugador:Player){
    println!("The adventurer's {} life is over...",jugador.name);
    println!("The adventurer reached level: {}",jugador.level);
    println!("Try again to reach the maximum level [50]");
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    //write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn combat_enemy(mut jugador:Player){
    
                let mut new_atack = 1;
                let mut new_defense = 1;
                let mut new_max_vida = 1;
                let mut new_vida = 1;
            
            // 5-10
            if jugador.level >= 5  && jugador.level <= 10{
                new_atack = rand::thread_rng().gen_range(1..5);
                new_defense = rand::thread_rng().gen_range(1..5);
                new_max_vida = rand::thread_rng().gen_range(1..5);
                new_vida = rand::thread_rng().gen_range(1..5);
            }
            // 11 - 14
            if jugador.level >= 11  && jugador.level <= 14{
                new_atack = rand::thread_rng().gen_range(8..14);
                new_defense = rand::thread_rng().gen_range(8..14);
                new_max_vida = rand::thread_rng().gen_range(8..14);
                new_vida = rand::thread_rng().gen_range(8..14);
            }
            // 15 - 19
            if jugador.level >= 15  && jugador.level <= 19{
                new_atack = rand::thread_rng().gen_range(10..19);
                new_defense = rand::thread_rng().gen_range(10..19);
                new_max_vida = rand::thread_rng().gen_range(10..19);
                new_vida = rand::thread_rng().gen_range(10..19);
            }
            // 20 - 29
            if jugador.level >= 20  && jugador.level <= 30{
                new_atack = rand::thread_rng().gen_range(19..29);
                new_defense = rand::thread_rng().gen_range(19..29);
                new_max_vida = rand::thread_rng().gen_range(19..29);
                new_vida = rand::thread_rng().gen_range(19..29);
            }
            // 30 - 39
            if jugador.level >= 30  && jugador.level <= 39{
                new_atack = rand::thread_rng().gen_range(30..39);
                new_defense = rand::thread_rng().gen_range(30..39);
                new_max_vida = rand::thread_rng().gen_range(30..39);
                new_vida = rand::thread_rng().gen_range(30..39);
            }
            // 40 - 50
            if jugador.level >= 40  && jugador.level <= 50{
                new_atack = rand::thread_rng().gen_range(40..50);
                new_defense = rand::thread_rng().gen_range(40..50);
                new_max_vida = rand::thread_rng().gen_range(40..50);
                new_vida = rand::thread_rng().gen_range(40..50);
            }
            
            
            let mut enemy = Player{
                name: String::from("Enemy"),
                ataque: new_atack,
                defensa: new_defense,
                max_vida: new_max_vida,
                vida: new_vida,
                level: {jugador.level},
                clase: Pclass::WARRIOR,
                };
            
            let start_combat:u32 = rand::thread_rng().gen_range(1..10);
            
            println!("You face the following enemy:");
            println!();
            println!("{}{}","attack: ".cyan(),enemy.ataque);
            println!("{}{}","defense: ".cyan(),enemy.defensa);
            println!("{}{}","Max Health: ".cyan(),enemy.max_vida);
            println!("{}{}","Health: ".cyan(),enemy.vida);
            println!("{}{}","Level: ".cyan(),enemy.level);
            pause();
            
            if start_combat <= 5 {
                println!("You start by attacking the enemy.");
                pause();
                
                // Start the combat.
                while jugador.vida > 0 || enemy.vida > 0 {
                    
                    if enemy.defensa >= jugador.ataque{
                        enemy.defensa -= jugador.ataque;
                        println!("{} succeeded in blocking the attack",enemy.name);
                        pause();
                    } else {
                    println!("You attack the enemy, inflicting: {} points to the enemy.",jugador.ataque);
                    
                    enemy.vida = remove_life(enemy.vida, jugador.ataque);
                    
                    println!("The enemy now has {} hit points.", enemy.vida);
                    pause();
                    }
                    if enemy.vida <= 0 {
                        println!("The enemy is dead.");                        
                        println!("Your defense has increased.");
                        println!();
                        
                        jugador.defensa += 1;
                        break;
                    }
                        pause();
                        if jugador.defensa >= enemy.ataque {
                            jugador.defensa -= enemy.ataque;
                            println!("{} succeeded in blocking the attack",jugador.name);
                            pause();
                        } else {
                                                
                        println!("The enemy attacks you, he has inflicted {} points of damage.",enemy.ataque);
                        
                        jugador.vida = remove_life(jugador.vida, enemy.ataque);
                        
                        println!("{} hour has {} life points.",jugador.name, jugador.vida);
                        pause();
                    }
                        if jugador.vida <= 0 {
                            println!("You died");
                            break;
                        }
                        pause();
                    
                }
                if jugador.vida <= 0 {
                    println!("You died");
                    end_run(jugador);
                } else{
                    println!("You have survived combat with {} hit points.",jugador.vida);
                    println!("You have gained one level in combat.");
                    pause();
                    
                    jugador.max_vida = add_max_life(jugador.max_vida);
                    jugador.defensa = add_defense(jugador.defensa, rand::thread_rng().gen_range(1..3));
                    jugador.ataque = add_attack(jugador.ataque, rand::thread_rng().gen_range(1..3));
                    jugador.level += 1;
                    
                    println!("{}{}{}","You now have maximum life: ".cyan(), jugador.max_vida," points.".cyan());
                    println!("{}{}{}","You now have: ".cyan(),jugador.ataque," attack points.".cyan());
                    println!("{}{}{}","You now have: ".cyan(),jugador.defensa," defense points.".cyan());
                    println!("{}{}","You now have the level: ".cyan(),jugador.level);
                    println!();
                    pause();
                    play_run(jugador);
                    
                }
            } else {
                println!("The enemy started attacking !");
                pause();
                
                // Start the combat.
                while jugador.vida > 0 || enemy.vida > 0 {
                    
                    if jugador.defensa >= enemy.ataque {
                        jugador.defensa -= enemy.ataque;
                        println!("{} succeeded in blocking the attack",jugador.name);
                        pause();
                    } else {
                        
                        println!("The enemy attacks you, he has inflicted {} points of damage.",enemy.ataque);
                        
                        jugador.vida = remove_life(jugador.vida, enemy.ataque);
                        
                        println!("{} hour has {} life points.",jugador.name, jugador.vida);
                        pause();
                    }
                    if jugador.vida <= 0 {
                        println!("You died.");
                        pause();
                        break;
                    }
                    pause();
                    
                    if enemy.defensa >= jugador.ataque{
                        enemy.defensa -= jugador.ataque;
                        println!("{} succeeded in blocking the attack",enemy.name);
                        pause();
                    } else {
                        
                        println!("You attack the enemy, inflicting: {} points to the enemy.",jugador.ataque);
                        
                        enemy.vida = remove_life(enemy.vida, jugador.ataque);
                        
                        println!("The enemy now has {} hit points.", enemy.vida);
                        pause();
                    }
                    if enemy.vida <= 0 {
                        println!("The enemy is dead.");
                        println!("Your defense has increased.");
                        println!();
                        pause();
                        jugador.defensa += 1;
                        break;
                    }
                }
                
                if jugador.vida <= 0 {
                    println!("You died.");
                    end_run(jugador);
                } else{
                    println!("You have survived combat with {} hit points.",jugador.vida);
                    println!("You have gained one level in combat.");
                    pause();
                    
                    jugador.max_vida = add_max_life(jugador.max_vida);
                    jugador.defensa = add_defense(jugador.defensa, rand::thread_rng().gen_range(1..3));
                    jugador.ataque = add_attack(jugador.ataque, rand::thread_rng().gen_range(1..3));
                    jugador.level += 1;
                    println!("{}{}{}","You now have maximum life: ".cyan(), jugador.max_vida," points.".cyan());
                    println!("{}{}{}","You now have: ".cyan(),jugador.ataque," attack points.".cyan());
                    println!("{}{}{}","You now have: ".cyan(),jugador.defensa," defense points.".cyan());
                    println!("{}{}","You now have the level: ".cyan(),jugador.level);
                    println!();
                    play_run(jugador);
                    
                }
                
            }
}


#[allow(dead_code)]
#[allow(unused_variables)]
fn play_run(mut jugador:Player){
    let selection = ["camino","ciudad","bosque","enemigo"];
    let rand_way = selection[rand::thread_rng().gen_range(0..3)].to_string();
    
    match rand_way.as_str() {
        "camino" => {
            println!("You find yourself on a path, you start walking it.");
            pause();
            let num_rand:u32= rand::thread_rng().gen_range(0..50);
            
            if num_rand < 35{
                println!("You travel several kilometers along the road without any problems.");
                pause();
                play_run(jugador);
            } else {
                println!("Oh no ! You find an enemy.");
                pause();
                combat_enemy(jugador);
            }
        },
        "ciudad" => {
            println!("You have found a city and decide to enter. You head to an inn to rest.");
            println!("Recover all your life points.");
            pause();
            jugador.vida = jugador.max_vida;
            play_run(jugador);
        },
        "bosque" => {
            println!("You find yourself in a lush forest, it seems somewhat dangerous; you continue...");
            let num_rand:u32= rand::thread_rng().gen_range(0..50);
            if num_rand < 25{
                println!("You cross the forest without any problems.");
                pause();
                play_run(jugador);
            } else if num_rand >= 25 && num_rand <= 49 {
                println!("Oh no ! You find an enemy.");
                pause();
                combat_enemy(jugador);
            } else {
                println!("While walking distracted, you hear the branches of the nearest tree crackle, with the bad luck that it falls on you and you are crushed to death.");
                pause();
                end_run(jugador);
            }
        },
        "enemigo" => {
            println!("One enemy, fight back!");
            pause();
            combat_enemy(jugador);
        },
        _=>println!("You got lost... better go back."),
    }
    
    
    
}