use std::env;

use crate::utils;

type Group = (usize, bool, Vec<i64>);

pub(crate) fn exercise_9_1() {

    utils::print_title("EXERCISE 9.1");

    let content = utils::read_to_string("EXERCISE_IX_I.txt");

    let start = utils::now();
    let groups = parse_data(&content);
    let total = allocate_memory(groups);
    let end = utils::now();

    utils::print_result(total, start, end);

}

pub(crate) fn exercise_9_2() {

    utils::print_title("EXERCISE 9.2");

    let content = utils::read_to_string("EXERCISE_IX_I.txt");

    let start = utils::now();
    let groups = parse_data(&content);
    let total = allocate_memory_fit(groups);
    let end = utils::now();

    utils::print_result(total, start, end);

}

fn allocate_memory(mut groups: Vec<Group>) -> i64 {
    let mut source_index = find_source(groups.len(), &groups).expect("Is filled");
    let mut source = groups.get(source_index).expect("Not found").clone();

    let print = env::var("EXERCISE_9_1_PRINT").unwrap_or_default().parse::<bool>().unwrap_or(false);

    if print {
        print_data(&groups);
    }

    loop {
        let (finish, aux_source, aux_groups) = allocate_source_memory(source, groups, print);
        let index = aux_source.0;
        groups = aux_groups;
        groups[index] = aux_source;
        if finish {
            return calculate_memory(&groups);
        }

        let aux_source_index = find_source(groups.len() - 1, &groups);
        if aux_source_index.is_none() {
            todo!()
        }

        source_index = aux_source_index.unwrap();
        source = groups.get(source_index).expect("Not found").clone();
    }
}

fn allocate_source_memory(mut source: Group, mut groups: Vec<Group>, print: bool) -> (bool, Group, Vec<Group>) {
    let mut position = source.2.len();

    while let Some(target_data) = find_target(source.0, &groups) {
        let mut free_index = target_data.0;
        
        for index_source in (0..position).rev() {
            let target = groups.get_mut(target_data.1).expect("Not found");
            let len = target.2.len();

            target.2[free_index] = source.2[index_source];
            source.2[index_source] = -1;
            free_index += 1;
            position = index_source;

            if print {
                print_movement(source.clone(), groups.clone());
            }
            
            if free_index == len {
                break;
            }
        }

        if position == 0 {
            return (false, source, groups);
        }
    }

    (true, source, groups)
}

fn find_source(last_source_index: usize, groups: &Vec<Group>) -> Option<usize> {
    for index in (0..last_source_index).rev() {
        let (_, _, group) = &groups[index];
        if let Some(_) = group.iter().position(|&i| i != -1) {
            return Some(index);
        }
    }
    return None;
}

fn find_target(source_id: usize, groups: &Vec<Group>) -> Option<(usize, usize)> {
    for (id, _, group) in groups {
        if *id == source_id {
            return None;
        }
        if let Some(index) = group.iter().position(|&i| i == -1) {
            return Some((index, *id));
        }
    }
    return None;
}


/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/


fn allocate_memory_fit(mut groups: Vec<Group>) -> i64 {
    let print = env::var("EXERCISE_9_2_PRINT").unwrap_or_default().parse::<bool>().unwrap_or(false);

    if print {
        print_data(&groups);
    }

    for index_source in (0..groups.len()).rev() {
        let source = groups.get(index_source).expect("Not found");
        if source.1 {
            continue;
        }
        for index_target in 0..groups.len() {
            let target = groups.get(index_target).expect("Not found");
            if source == target {
                break;
            }
            if !target.1 {
                continue;
            }
            if source.2.len() <= target.2.iter().filter(|&i| *i == -1).count() {
                let result = fill_spaces(target.clone(), source.2.len(), source.2[0]);
                groups[index_source] = (source.0, source.1, vec![-1; source.2.len()]);
                groups[index_target] = result;
                if print {
                    print_data(&groups);
                }
                break;
            }
        }
    }

    calculate_memory(&groups)
}

fn fill_spaces(mut group: Group, spaces: usize, value: i64) -> Group {
    let mut count = 0;
    for element in group.2.iter_mut() {
        if *element != -1 {
            continue;
        }

        *element = value;
        count += 1;

        if count == spaces {
            break;
        }
    }
    group
}

fn calculate_memory(groups: &Vec<Group>) -> i64 {
    let mut result = 0;
    let mut index = 0;
    for (_, _, group) in groups {
        for item in group {
            if *item == -1 {
                index += 1;
                continue;
            }
            result += index as i64 * *item as i64;
            index += 1;
        }
    }
    result
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> Vec<Group> {
    let sanitized = content.replace("\r\n", "\n");
    let fragments = sanitized
        .trim()
        .split("")
        .filter(|i| !i.is_empty());

    
    let mut groups = Vec::new();
    let mut count = 0;
    for (i, fragment) in fragments.enumerate() {
        let is_space = i % 2 != 0;
        let id = i;
        let value = fragment.parse::<usize>().expect("Not a number");

        let group = match is_space {
            true => vec![-1; value],
            false => vec![count as i64; value],
        };

        if !is_space {
            count += 1;
        }

        groups.push((id, is_space, group));
    }

    groups
}

fn print_movement(source: Group, groups: Vec<Group>) {
    let mut aux_groups = groups.clone();
    aux_groups[source.0] = source.clone();
    print_data(&aux_groups);
}

fn print_data(groups: &Vec<Group>) {
    for (_, _, group) in groups {
        for item in group {
            let styled_item = match item {
                -1 => String::from("."),
                _ => item.to_string()
            };
            print!("{styled_item}");
        }
    }
    println!()
}
