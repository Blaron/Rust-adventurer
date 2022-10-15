use rand::Rng;
use std::io;
use std::io::prelude::*;


// Player class type:
// enum Pclass {
//     MAGE,
//     WARRIOR,
//     ARCHER,
// }

// layer structure :
#[allow(dead_code)]
struct Player {
    name: String,
    ataque: i32,
    defensa: i32,
    max_vida: i32,
    vida: i32,
    level: u16,
    //clase: Pclass,
}

#[allow(unused_variables)]
fn main(){
    
    let mut hero_name:String = String::new();
    //let mut hero_class:String = String::new();
    println!("Bienvendido al mundo aventurero, dime tu nombre.");
    let b1_name = std::io::stdin().read_line(&mut hero_name).unwrap();
    // println!("Perfecto {}. ¿ Qué clase eres ?",hero_class);
    // let b2_clase:usize = std::io::stdin().read_line(&mut hero_class).unwrap();
    println!("Ahora parte hacia tu aventura, este es tus stats iniciales:");
    
    #[allow(unused_mut)]
    let mut jugador = Player{
        name:hero_name,
        ataque: 2,
        defensa: 1,
        max_vida: 3,
        vida: 3,
        level: 1,
        //clase: get_class(hero_class),
    };
    println!("name: {}",jugador.name);
    println!("atack: {}",jugador.ataque);
    println!("defense: {}",jugador.defensa);
    println!("max_health: {}",jugador.max_vida);
    println!("health: {}",jugador.vida);
    println!("Lvl: {}",jugador.level);
    //println!("class: {}",&b1_name);
    pause();
    println!("Es hora de la aventura, {}. ",jugador.name);
    play_run(jugador);
    

}

// fn get_class(tipe: String) -> Pclass{
//     match tipe.as_str() {
//         "warrior"=>Pclass::WARRIOR,
//         "mage"=>Pclass::MAGE,
//         "archer"=>Pclass::ARCHER,
//         _ =>Pclass::WARRIOR,
//     }
// }


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
    println!("A finalizado la vida del aventurero: {}",jugador.name);
    println!("El aventurero llegó al nivel: {}",jugador.level);
    println!("Prueba de nuevo para llegar al nivel maximo [50]");
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
                //clase: Pclass::WARRIOR,
                };
            
            let start_combat:u32 = rand::thread_rng().gen_range(1..10);
            
            println!("Te enfrentas al siguiente enemigo:");
            println!("name: {} comun",enemy.name);
            println!("atack: {}",enemy.ataque);
            println!("defense: {}",enemy.defensa);
            println!("max_health: {}",enemy.max_vida);
            println!("health: {}",enemy.vida);
            println!("Lvl: {}",enemy.level);
            pause();
            
            if start_combat <= 5 {
                println!("Empiezas atacando al enemigo.");
                pause();
                
                // Start the combat.
                while jugador.vida > 0 || enemy.vida > 0 {
                    
                    if enemy.defensa >= jugador.ataque{
                        enemy.defensa -= jugador.ataque;
                        println!("{} consiguió bloquear el ataque",enemy.name);
                        pause();
                    } else {
                    println!("Atacas al enemigo, infligiendo: {} puntos al enemigo.",jugador.ataque);
                    
                    enemy.vida = remove_life(enemy.vida, jugador.ataque);
                    
                    println!("El enemigo ahora tiene {} puntos de vida.", enemy.vida);
                    pause();
                    }
                    if enemy.vida <= 0 {
                        println!("El enemigo ha muerto.");
                        println!("Tu defensa a aumentado");
                        
                        jugador.defensa += 1;
                        break;
                    }
                        pause();
                        if jugador.defensa >= enemy.ataque {
                            jugador.defensa -= enemy.ataque;
                            println!("{} consiguió bloquear el ataque",jugador.name);
                            pause();
                        } else {
                                                
                        println!("El enemigo te ataca, te ha infligido {} puntos de daño.",enemy.ataque);
                        
                        jugador.vida = remove_life(jugador.vida, enemy.ataque);
                        
                        println!("{} hora tiene {} puntos de vida.",jugador.name, jugador.vida);
                        pause();
                    }
                        if jugador.vida <= 0 {
                            println!("Has muerto");
                            break;
                        }
                        pause();
                    
                }
                if jugador.vida <= 0 {
                    println!("Has muerto");
                    end_run(jugador);
                } else{
                    println!("Has sobrevivido al combate con {} puntos de vida",jugador.vida);
                    println!("Has subido un nivel por el combate");
                    pause();
                    
                    jugador.max_vida = add_max_life(jugador.max_vida);
                    jugador.defensa = add_defense(jugador.defensa, rand::thread_rng().gen_range(1..3));
                    jugador.ataque = add_attack(jugador.ataque, rand::thread_rng().gen_range(1..3));
                    jugador.level += 1;
                    
                    println!("Ahora tienes de vida maxima: {} puntos", jugador.max_vida);
                    println!("Ahora tienes: {} puntos de ataque.",jugador.ataque);
                    println!("Ahora tienes: {} puntos de defensa.",jugador.defensa);
                    println!("Ahora tienes: {} puntos de nivel.",jugador.level);
                    pause();
                    play_run(jugador);
                    
                }
            } else {
                println!("¡ El enemigo empezó atacando !");
                pause();
                
                // Start the combat.
                while jugador.vida > 0 || enemy.vida > 0 {
                    
                    if jugador.defensa >= enemy.ataque {
                        jugador.defensa -= enemy.ataque;
                        println!("{} consiguió bloquear el ataque",jugador.name);
                        pause();
                    } else {
                        
                        println!("El enemigo te ataca, te ha infligido {} puntos de daño.",enemy.ataque);
                        
                        jugador.vida = remove_life(jugador.vida, enemy.ataque);
                        
                        println!("{} ahora tiene {} puntos de vida.",jugador.name, jugador.vida);
                        pause();
                    }
                    if jugador.vida <= 0 {
                        println!("Has muerto.");
                        pause();
                        break;
                    }
                    pause();
                    
                    if enemy.defensa >= jugador.ataque{
                        enemy.defensa -= jugador.ataque;
                        println!("{} consiguió bloquear el ataque",enemy.name);
                        pause();
                    } else {
                        
                        println!("Atacas al enemigo, infligiendo: {} puntos al enemigo.",jugador.ataque);
                        
                        enemy.vida = remove_life(enemy.vida, jugador.ataque);
                        
                        println!("El enemigo ahora tiene {} puntos de vida.", enemy.vida);
                        pause();
                    }
                    if enemy.vida <= 0 {
                        println!("El enemigo ha muerto.");
                        println!("Tu defensa a aumentado");
                        pause();
                        jugador.defensa += 1;
                        break;
                    }
                }
                
                if jugador.vida <= 0 {
                    println!("Has muerto");
                    end_run(jugador);
                } else{
                    println!("Has sobrevivido al combate con {} puntos de vida",jugador.vida);
                    println!("Has subido un nivel por el combate");
                    pause();
                    
                    jugador.max_vida = add_max_life(jugador.max_vida);
                    jugador.defensa = add_defense(jugador.defensa, rand::thread_rng().gen_range(1..3));
                    jugador.ataque = add_attack(jugador.ataque, rand::thread_rng().gen_range(1..3));
                    jugador.level += 1;
                    println!("Ahora tienes de vida maxima: {} puntos", jugador.max_vida);
                    println!("Ahora tienes: {} puntos de ataque.",jugador.ataque);
                    println!("Ahora tienes: {} puntos de defensa.",jugador.defensa);
                    println!("Ahora tienes: {} puntos de nivel.",jugador.level);
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
            println!("Te encuentras en un camino, empiezas a recorrerlo.");
            pause();
            let num_rand:u32= rand::thread_rng().gen_range(0..50);
            
            if num_rand < 35{
                println!("Recorres varios kilometros por el camino sin problemas.");
                pause();
                play_run(jugador);
            } else {
                println!("¡ Oh no !, te encuentras un enemigo.");
                pause();
                combat_enemy(jugador);
            }
        },
        "ciudad" => {
            println!("Has encontrado una ciudad y decides entrar. Te diriges a una posada para descansar.");
            println!("Recuperas todos tus puntos de vida");
            pause();
            jugador.vida = jugador.max_vida;
            play_run(jugador);
        },
        "bosque" => {
            println!("Te encuentras en un frondoso bosque, parece algo peligroso; continuas...");
            let num_rand:u32= rand::thread_rng().gen_range(0..50);
            if num_rand < 25{
                println!("Atraviesas el bosque sin problemas.");
                pause();
                play_run(jugador);
            } else if num_rand >= 25 && num_rand <= 49 {
                println!("¡ Oh no !, te encuentras un enemigo.");
                pause();
                combat_enemy(jugador);
            } else {
                println!("Mientras caminabas distraido, escuchas crujir las ramas del arbol mas cercano, con la mala suerte de que te cae encima y mueres estrujado.");
                pause();
                end_run(jugador);
            }
        },
        "enemigo" => {
            println!("¡ Un enemigo, a luchar !");
            pause();
            combat_enemy(jugador);
        },
        _=>println!("Te has perdido... mejor volver."),
    }
    
    
    
}