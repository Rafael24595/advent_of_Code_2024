use crate::utils;

pub(crate) fn exercise_2_1() {

    utils::print_title("EXERCISE 2.1");

    let content = utils::read_to_string("EXERCISE_II_I.txt");

    let start = utils::now();
    let total = check_levels(&content);
    let end = utils::now();

    utils::print_result(total, start, end);
}

pub(crate) fn exercise_2_2() {

    utils::print_title("EXERCISE 2.2");

    let content = utils::read_to_string("EXERCISE_II_I.txt");

    let start = utils::now();
    let total = check_levels_with_exceptions(&content);
    let end = utils::now();

    utils::print_result(total, start, end);

}

fn check_levels(content: &str) -> usize {
    let reports = content.split("\n").collect::<Vec<&str>>();

    let mut safe = reports.len();

    for levels_str in reports {
        let levels = levels_str.split(" ")
            .map(|l| l.trim().parse::<i64>().expect("Not a number"))
            .collect::<Vec<i64>>();
        
        let (row, ok) = evalue_row(&levels);
        if ok {
            println!("✔️  {}", row);
            continue;
        }

        safe -= 1;

        println!("❌ {}", row);
    }

    safe
}

fn check_levels_with_exceptions(content: &str) -> usize {
    let reports = content.split("\n").collect::<Vec<&str>>();

    let mut safe = reports.len();

    for levels_str in reports {

        let levels = levels_str.split(" ")
            .map(|l| l.trim().parse::<i64>().expect("Not a number"))
            .collect::<Vec<i64>>();
        
        let (row, ok) = evalue_row(&levels);
        if ok {
            println!("✔️  {}", row);
            continue;
        }
        
        let (rows, ok) = evalue_exceptions(levels);

        let mut status = "❌";
        if ok {
            status = "✔️ ";
        }

        println!("{} {}\n{}", status, row, rows);
        if !ok {
            safe -= 1;
        }

    }

    safe
}

fn evalue_exceptions(levels: Vec<i64>) -> (String, bool) {
    let mut buffer: Vec<String> = Vec::new();
    for i in 0..levels.len() {
        let mut levels_clone = levels.clone();
        levels_clone.remove(i);
        let (row, ok) = evalue_row(&levels_clone);
        if ok {
            buffer.push(format!("    ✔️  {}", row));
            return (buffer.join("\n"), true);
        }
        buffer.push(format!("    ❌ {}", row));
    }
    return (buffer.join("\n"), false);
}

fn evalue_row(levels: &Vec<i64>) -> (String, bool) {
    let mut last_level = None;
    let mut last_direction = None;

    let mut buffer: Vec<String> = Vec::new();

    let mut iter = levels.iter();   
    while let Some(level) = iter.next() {
        if last_level.is_none() {
            last_level = Some(level);
            buffer.push(level.to_string());
            continue;
        }
    
        let level_aux = last_level.expect("Cannot happen!");
        let direction = level > level_aux;
    
        if evalue_levels(last_direction, *level_aux, *level) {
            let mut rest = iter.map(|l| l.to_string()).collect::<Vec<String>>();
            buffer.push(utils::ConsoleColors::CONSOLE_FAIL.wrap(level));
            buffer.append(&mut rest);
            return (buffer.join(" "), false);
        }
    
        last_level = Some(level);
        last_direction = Some(direction);

        buffer.push(level.to_string());
    }

    return (buffer.join(" "), true);
}

fn evalue_levels(last_direction: Option<bool>, last_level: i64, level: i64) -> bool {
    let direction = level > last_level;
    let differ = (level - last_level).abs();
    return last_direction.unwrap_or(direction) != direction || differ == 0 || differ > 3
}
