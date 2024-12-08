use std::collections::HashMap;

use crate::utils;

type Antenna = (usize, usize, String);
type Distance = (usize, usize, i8, i8);
type Point = (usize, usize);

pub(crate) fn exercise_8_1() {

    utils::print_title("EXERCISE 8.1");

    let content = utils::read_to_string("EXERCISE_VIII_I.txt");

    let start = utils::now();
    let (matrix, antennas) = parse_data(&content);
    let total = calculate_nodes_min_distance(&matrix, &antennas);
    let end = utils::now();

    utils::print_result(total, start, end);

}

pub(crate) fn exercise_8_2() {

    utils::print_title("EXERCISE 8.2");

    let content = utils::read_to_string("EXERCISE_VIII_I.txt");

    let start = utils::now();
    let (matrix, antennas) = parse_data(&content);
    let total = calculate_nodes_max_distance(&matrix, &antennas);
    let end = utils::now();

    utils::print_result(total, start, end);

}

fn calculate_nodes_min_distance(matrix: &Vec<Vec<String>>, antennas: &HashMap<String, Vec<Antenna>>) -> usize {
    let nodes = calculate_nodes(matrix, antennas, false);
    print_data(&matrix, &nodes);
    nodes.len()
}

fn calculate_nodes_max_distance(matrix: &Vec<Vec<String>>, antennas: &HashMap<String, Vec<Antenna>>) -> usize {
    let nodes = calculate_nodes(matrix, antennas, true);
    print_data(&matrix, &nodes);
    nodes.len()
}

fn calculate_nodes(matrix: &Vec<Vec<String>>, antennas: &HashMap<String, Vec<Antenna>>, long_distance: bool) -> HashMap<String, bool>  {
    let mut nodes_map = HashMap::new();
    let mut visited = HashMap::new();
    for (_, group) in antennas {
        for antenna_a in group {
            for antenna_b in group {
                let (key_1, key_2) = make_visited_key(antenna_a, antenna_b);
                if antenna_a == antenna_b || visited.contains_key(&key_1) {
                    continue;
                }

                let distance = calculate_distance(antenna_a, antenna_b);
                for node in simulate_node(matrix, antenna_a, antenna_b, &distance, long_distance) {
                    let key = make_key(node);
                    nodes_map.insert(key.clone(), true);
                }

                visited.insert(key_1, true);
                visited.insert(key_2, true);
            }
        }
    }

    nodes_map
}

fn calculate_distance(antenna_a: &Antenna, antenna_b: &Antenna) -> Distance {
    let y_distance = antenna_a.0 as i64 - antenna_b.0 as i64;
    let x_distance = antenna_a.1 as i64 - antenna_b.1 as i64;

    let y = y_distance.abs() as usize;
    let x = x_distance.abs() as usize;

    let mut y_direction = 1;
    if y_distance < 0 {
        y_direction = -1
    }

    let mut x_direction = 1;
    if x_distance < 0 {
        x_direction = -1
    }
    
    (y, x, y_direction, x_direction)
}

fn simulate_node(matrix: &Vec<Vec<String>>, antenna_a: &Antenna, antenna_b: &Antenna, distance: &Distance, long_distance: bool) -> Vec<(usize, usize)> {
    let mut nodes = Vec::new();
    
    let mut last_distance = *distance;
    let mut count = 0;
    loop {
        let node_a = locate_node(matrix, antenna_a, &last_distance);
        if let Some(node) = node_a {
            count += 1;
            nodes.push(node);
        }
    
        let aux_distance = &(last_distance.0, last_distance.1, last_distance.2 * - 1, last_distance.3 * - 1);
        let node_b = locate_node(matrix, antenna_b, aux_distance);
        if let Some(node) = node_b {
            count += 1;
            nodes.push(node);
        }

        if !long_distance || (node_a.is_none() && node_b.is_none()) {
            break;
        }

        last_distance = (last_distance.0 + distance.0, last_distance.1 + distance.1, distance.2, distance.3);
    }

    if long_distance && count >= 1 {
        nodes.push((antenna_a.0, antenna_a.1));
        nodes.push((antenna_b.0, antenna_b.1));
    }

    nodes
}

fn locate_node(matrix: &Vec<Vec<String>>, antenna: &Antenna, distance: &Distance) -> Option<Point> {
    let y = antenna.0 as i64 + (distance.0 as i64 * distance.2 as i64);
    if y < 0 || y >= matrix.len() as i64 {
        return None;
    }

    let x = antenna.1 as i64 + (distance.1 as i64 * distance.3 as i64);
    if x < 0 || x >= matrix[0].len() as i64 {
        return None;
    }

    Some((y as usize, x as usize))
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> (Vec<Vec<String>>, HashMap<String, Vec<Antenna>>) {
    let sanitized = content.replace("\r\n", "\n");
    let lines = sanitized
        .trim()
        .split("\n");

    let mut antenna_map = HashMap::new();
    let mut matrix = Vec::new();
    for (y, line) in lines.enumerate() {
        let row = line.trim().split("")
            .filter(|i| !i.is_empty())
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        matrix.push(row);

        let antennas = line
            .char_indices()
            .filter(|(_, c)| *c != '.');
        for (x, antenna) in antennas {
            let antenna = antenna.to_string();
            if !antenna_map.contains_key(&antenna) {
                antenna_map.insert(antenna.clone(), Vec::new());
            }
            antenna_map.get_mut(&antenna)
                .expect("Cannot succed")
                .push((y, x, antenna));
        }
    }

    (matrix, antenna_map)
}

fn make_key(point: Point) -> String {
    format!("{}#{}", point.0, point.1)
}

fn make_visited_key(antenna_a: &Antenna, antenna_b: &Antenna) -> (String, String) {
    (format!("{}#{}-{}#{}", antenna_a.0, antenna_a.1, antenna_b.1, antenna_b.2),
     format!("{}#{}-{}#{}", antenna_b.0, antenna_b.1, antenna_a.1, antenna_a.2))
}

fn print_data(matrix: &Vec<Vec<String>>, nodes: &HashMap<String, bool>) {
    let mut buffer = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, entity) in row.iter().enumerate() {
            let frequency = nodes.get(&make_key((y, x)));
            if frequency.is_none() {
                buffer.push(entity.clone());
                continue;
            }

            if entity != "." {
                buffer.push(utils::ConsoleColors::CONSOLE_SUCESS.wrap(entity));
            } else {
                buffer.push(utils::ConsoleColors::CONSOLE_RESULT.wrap("#"));
            }
        }
        buffer.push(String::from("\n"));
    }
    print!("{}", buffer.join(""))
}
