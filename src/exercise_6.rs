use std::{collections::HashMap, env, io::{self, Write}, thread, time::Duration};

use crate::utils;

const INITIAL: &str = "X";

const VOID: u8 = 1;
const OBSTACLE: u8 = 2;

pub(crate) fn exercise_6_1() {

    utils::print_title("EXERCISE 6.1");

    let content = utils::read_to_string("EXERCISE_VI_I.txt");

    let start = utils::now();
    let (tango, matrix) = parse_data(&content);
    let total = calculate_route(tango, matrix, &|| utils::print_title("EXERCISE 6.1") );
    let end = utils::now();

    utils::print_result(total, start, end);

}

pub(crate) fn exercise_6_2() {

    utils::print_title("EXERCISE 6.2");

    let content = utils::read_to_string("EXERCISE_VI_I.txt");

    let start = utils::now();
    let (tango, matrix) = parse_data(&content);
    let total = calculate_route_with_obstacule(tango, matrix, &|| utils::print_title("EXERCISE 6.2") );
    let end = utils::now();

    utils::print_result(total, start, end);

}

fn calculate_route<T>(tango: (usize, usize, &str), matrix: Vec<Vec<u8>>, title: &T) -> usize where T: Fn() {
    let mut tango = tango;

    let is_print = env::var("PRINT_6_1").unwrap_or_default().parse::<bool>().unwrap_or(false);
    let speed = env::var("SPEED_6_1").unwrap_or_default().parse::<u64>().unwrap_or(350);

    let mut movements = HashMap::new();
    movements.insert(make_movements_key(tango), vec![INITIAL]);

    let mut patrol = true;
    while patrol {
        let aux_tango = calculate_movement(tango, &matrix);
        if aux_tango.is_none() {
            patrol = false;
            continue;
        }

        let aux_tango = aux_tango.unwrap();
        
        let key = make_movements_key(aux_tango);
        if !movements.contains_key(&key) {
            movements.insert(key.clone(), Vec::new());
        }

        movements.get_mut(&key).expect("msg").push(aux_tango.2);
        
        tango = aux_tango;

        if is_print {
            thread::sleep(Duration::from_millis(speed));
            clean_screen();
            title();
            print_data(tango, &matrix, title, &movements, &HashMap::new());
            reestore_cursor();
        }
    }

    if !is_print {
        print_data(tango, &matrix, title, &movements, &HashMap::new());
    }

    movements.len()
}

fn calculate_movement<'a>(tango: (usize, usize, &'a str), matrix: &'a Vec<Vec<u8>>) -> Option<(usize, usize, &'a str)> {
    let next = try_next(tango, matrix);
    if next.is_none() {
        return None;
    }

    let next = next.unwrap();

    match matrix[next.0][next.1] {
        VOID => Some(next),
        OBSTACLE => Some((tango.0, tango.1, turn_right(tango))),
        _ => panic!("Undefined entity")
    }
}


/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/


fn calculate_route_with_obstacule<T>(tango: (usize, usize, &str), matrix: Vec<Vec<u8>>, title: &T) -> usize where T: Fn() {
    let mut tango = tango;

    let is_print = env::var("PRINT_6_2").unwrap_or_default().parse::<bool>().unwrap_or(false);
    let speed = env::var("SPEED_6_2").unwrap_or_default().parse::<u64>().unwrap_or(350);

    let mut movements = HashMap::new();
    movements.insert(make_movements_key(tango), vec![INITIAL]);

    let mut obstacles = HashMap::new();

    let mut in_route = true;
    while in_route {
        let aux_tango = calculate_movement_with_obstacule(tango, &matrix, &movements, &obstacles);
        if aux_tango.is_none() {
            in_route = false;
            continue;
        }

        let (aux_tango, obstacule) = aux_tango.unwrap();

        let key = make_movements_key(aux_tango);
        if !movements.contains_key(&key) {
            movements.insert(key.clone(), Vec::new());
        }

        movements.get_mut(&key).expect("msg").push(aux_tango.2);
        
        tango = aux_tango;

        if obstacule {
            let key = make_obstacle_key(tango.0, tango.1);
            obstacles.insert(key, true);
        }

        if is_print {
            thread::sleep(Duration::from_millis(speed));
            clean_screen();
            title();
            print_data(tango, &matrix, title, &movements, &obstacles);
            reestore_cursor();
        }
    }

    if !is_print {
        print_data(tango, &matrix, title, &movements, &obstacles);
    }

    obstacles.len()
}

fn calculate_movement_with_obstacule<'a>(tango: (usize, usize, &'a str), matrix: &'a Vec<Vec<u8>>, movements: &HashMap<String, Vec<&str>>, obstacules: &HashMap<String, bool>) -> Option<((usize, usize, &'a str), bool)> {
    let next = try_next(tango, matrix);
    if next.is_none() {
        return None;
    }

    let mut next = next.unwrap();
    next = match matrix[next.0][next.1] {
        VOID => next,
        OBSTACLE => (tango.0, tango.1, turn_right(tango)),
        _ => panic!("Undefined entity")
    };

    let key_obstacle = make_obstacle_key(next.0, next.1);
    let is_checked = obstacules.get(key_obstacle.as_str()).is_some();

    let key_movements = make_movements_key(next);
    let is_path = movements.get(key_movements.as_str()).is_some();

    let can_place_obstacle = can_place_obstacle(next, movements);
    if is_checked || is_path || !can_place_obstacle {
        return Some((next, false))
    }

    let temp_obstacle = (next.0, next.1);
    Some((next, check_obstacules(tango, &matrix, movements, temp_obstacle)))

}

fn check_obstacules(tango: (usize, usize, &str), matrix: &Vec<Vec<u8>>, movements: &HashMap<String, Vec<&str>>, temp_obstacle: (usize, usize)) -> bool {
    let mut tango = (tango.0, tango.1, turn_right(tango));
    if verify_path(tango, movements) {
        return true;
    }

    let mut simulation_movements = Vec::new();
    loop {
        let aux_tango = try_next(tango, matrix);
        if aux_tango.is_none() {
            return false;
        }

        let aux_tango = aux_tango.unwrap();
        let is_temp_obstacule = aux_tango.0 == temp_obstacle.0 && aux_tango.1 == temp_obstacle.1;

        let entity = matrix[aux_tango.0][aux_tango.1];
        if entity == VOID && !is_temp_obstacule {
            tango = aux_tango;
            if verify_path(aux_tango, movements) {
                return true;
            }
        } else if entity == OBSTACLE || is_temp_obstacule {
            tango = (tango.0, tango.1, turn_right(tango));
            if verify_path(tango, movements) {
                return true;
            }

            let key = make_movement_key(tango);
            if simulation_movements.contains(&key) {
                return true;
            }

            simulation_movements.push(key);
        } else {
            panic!("Undefined entity")
        }
    }
}

fn verify_path(tango: (usize, usize, &str), movements: &HashMap<String, Vec<&str>>) -> bool {
    let key = make_movements_key(tango);
    let route = movements.get(key.as_str());
    if route.is_none() {
        return false;
    }
    let route = route.unwrap();
    return route.contains(&tango.2) ;
}

fn can_place_obstacle(tango: (usize, usize, &str), movements: &HashMap<String, Vec<&str>>) -> bool {
    let key = make_movements_key(tango);
    let route = movements.get(key.as_str());
    if route.is_none() {
        return true;
    }
    let route = route.unwrap();
    return !route.contains(&INITIAL);
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> ((usize, usize, &str), Vec<Vec<u8>>) {
    let mut tango = (0, 0, "N");
    let mut matrix = Vec::new();
    for (y, row_str) in content.replace("\r\n", "\n").trim().split("\n").enumerate() {
        let mut row = Vec::new();
        for (x, entity) in row_str.split("").map(|e| e.trim()).filter(|e| !e.is_empty()).enumerate() {
            match entity {
                "^" => {
                    tango = (y, x, "N");
                    row.push(VOID);
                },
                "#" => row.push(OBSTACLE),
                _ => row.push(VOID),
            }
        }
        matrix.push(row);
    }
    (tango, matrix)
}

fn make_movements_key(tango: (usize, usize, &str)) -> String {
    make_movements_key_from_raw(tango.0, tango.1)
}

fn make_movements_key_from_raw(y: usize, x: usize) -> String {
    format!("{}#{}", y, x)
}

fn make_obstacle_key(y: usize, x: usize) -> String {
    format!("{}#{}", y, x)
}

fn make_movement_key(tango: (usize, usize, &str)) -> String {
    format!("{}#{}#{}", tango.0, tango.1, tango.2)
}

fn try_next<'a>(tango: (usize, usize, &'a str), matrix: &'a Vec<Vec<u8>>) -> Option<(usize, usize, &'a str)> {
    let y = tango.0;
    let x = tango.1;
    let d = tango.2;
    
    match d.to_ascii_uppercase().as_str() {
        "N" => {
            if y == 0 {
                return None
            }
            Some((y - 1, x, d))
        }
        "S" => {
            if y == matrix.len() - 1 {
                return None
            }
            Some((y + 1, x, d))
        }
        "E" => {
            if x == matrix[0].len() - 1 {
                return None
            }
            Some((y, x + 1, d))
        }
        "W" => {
            if x == 0 {
                return None
            }
            Some((y, x - 1, d))
        }
        _ => panic!("Undefined direction")
    }
}

fn turn_right(tango: (usize, usize, &str)) -> &str {
    match tango.2.to_uppercase().as_str() {
        "N" => "E",
        "S" => "W",
        "E" => "S",
        "W" => "N",
        _ => panic!("Undefined direction")
    }
}

fn print_data<T>(tango: (usize, usize, &str), matrix: &Vec<Vec<u8>>, title: T, movements: &HashMap<String, Vec<&str>>, obstacles: &HashMap<String, bool>) where T: Fn() {    
    let mut buffer = String::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, entity) in row.iter().enumerate() {
            let styled_entity = match *entity {
                OBSTACLE => String::from("#"),
                _ => {
                    let is_tango = tango.0 == y && tango.1 == x;

                    let key_obstacle = make_obstacle_key(y, x);
                    let is_obstacule =  obstacles.get(key_obstacle.as_str()).is_some();

                    let key_movement = make_movements_key_from_raw(y, x);

                    if is_tango && is_obstacule {
                        utils::ConsoleColors::CONSOLE_SUCESS.wrap("O")
                    } else if is_tango {
                        let styled_tango = styled_tango(tango);
                        utils::ConsoleColors::CONSOLE_SUCESS.wrap(styled_tango)
                    } else if is_obstacule {
                        utils::ConsoleColors::CONSOLE_RESULT.wrap("O")
                    } else if let Some(movement) = movements.get(key_movement.as_str()) {
                        styled_movement(movement)
                    } else {
                        String::from(".")
                    }
                }
            };
            buffer = format!("{buffer}{styled_entity}");
        }
        buffer = format!("{buffer}\n");
    }
    
    println!("{buffer}");
    io::stdout().flush().unwrap();
}

fn styled_movement(movement: &Vec<&str>) -> String {
    let is_vertical = is_vertical(movement);
    let is_horizontal = is_horizontal(movement);

    if movement.contains(&INITIAL) {
        String::from(INITIAL)
    } else if is_horizontal {
        String::from("-")
    } else if is_vertical {
        String::from("|")
    } else {
        String::from("+")
    }
}

fn is_vertical(movement: &Vec<&str>) -> bool {
    (movement.contains(&"N") || movement.contains(&"S")) && !(movement.contains(&"E") || movement.contains(&"W"))
}

fn is_horizontal(movement: &Vec<&str>) -> bool {
    (movement.contains(&"E") || movement.contains(&"W")) && !(movement.contains(&"N") || movement.contains(&"S")) 
}

fn styled_tango(tango: (usize, usize, &str)) -> &str {
    match tango.2.to_uppercase().as_str() {
        "N" => "^",
        "S" => "v",
        "E" => ">",
        "W" => "<",
        _ => panic!("Undefined direction")
    }
}

fn clean_screen() { 
    print!("\x1B[2J\x1B[H");
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}

fn reestore_cursor() {
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}
