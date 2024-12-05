use std::collections::HashMap;

use crate::utils;

pub(crate) fn exercise_5_1() {

    utils::print_title("EXERCISE 5.1");

    let content = utils::read_to_string("EXERCISE_V_I.txt");

    let (rules, pages) = parse_data(&content);
    
    let total = evalue_pages(&rules, pages);

    utils::print_result(total);
}

pub(crate) fn exercise_5_2() {

    utils::print_title("EXERCISE 5.2");

    let content = utils::read_to_string("EXERCISE_V_I.txt");

    let (rules, pages) = parse_data(&content);
    
    let total = evalue_pages_with_correction(&rules, pages);

    utils::print_result(total);

}

fn evalue_pages(rules: &HashMap<i64, (Vec<i64>, Vec<i64>)>, pages: Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for row in pages {
        let mut status = true;

        let mut row_styled = row.iter()
            .map(|p| utils::ConsoleColors::CONSOLE_POWER.wrap(p))
            .collect::<Vec<String>>();

        for (i, page) in row.iter().enumerate() {
            let left = &row[0..i].to_vec();
            let right = &row[i..row.len()].to_vec();
            if !evalue_page(rules, page, left, right) {
                status = false;
                row_styled[i] = utils::ConsoleColors::CONSOLE_FAIL.wrap(page);
                break;
            }
            row_styled[i] = utils::ConsoleColors::CONSOLE_SUCESS.wrap(page);
        }

        if status {
            let middle_index = row.len() / 2;
            result += row[middle_index];
        }
    
        println!("{}: [{}] => {}\n",
            utils::ConsoleColors::CONSOLE_BOLD.wrap("Row"),
            row_styled.join(","),
            utils::ConsoleColors::CONSOLE_RESULT.wrap(result)
        );
    }

    result
}

fn evalue_page(rules: &HashMap<i64, (Vec<i64>, Vec<i64>)>, page: &i64, left: &Vec<i64>, right: &Vec<i64>) -> bool {
    let rules = rules.get(&page).expect("Not registered");

    for n in left {
        if rules.1.contains(&n) {
            return false;
        }
    }

    for n in right {
        if rules.0.contains(&n) {
            return false;
        }
    }

    return true;
}


/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/


fn evalue_pages_with_correction(rules: &HashMap<i64, (Vec<i64>, Vec<i64>)>, pages: Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for row in pages {
        let (eval_result, count, buffer) = evalue_row(rules, &row, Vec::new(), 0);

        result += eval_result;

        let mut styled_status = utils::ConsoleColors::CONSOLE_SUCESS.wrap(true);
        if count != 0 {
            styled_status = utils::ConsoleColors::CONSOLE_FAIL.wrap(false);
        }

        println!("{}: {:?} => {styled_status} [{}] {}\n", 
            utils::ConsoleColors::CONSOLE_BOLD.wrap("Row"),
            row,
            utils::ConsoleColors::CONSOLE_RESULT.wrap(result),
            buffer.iter().map(|v| format!("\n{v}")).collect::<Vec<String>>().join("")
        );

    }

    result
}

fn evalue_row(rules: &HashMap<i64, (Vec<i64>, Vec<i64>)>, row: &Vec<i64>, mut buffer: Vec<String>, count: usize) -> (i64, usize, Vec<String>) {
    for (i, page) in row.iter().enumerate() {
        let left = &row[0..i].to_vec();
        let right = &row[i+1..row.len()].to_vec();

        let (status, fix, updated_buffer) = evalue_page_with_correction(rules, page, left, right, buffer);
        buffer = updated_buffer;
        if !status {
            return evalue_row(rules, &fix, buffer, count + 1);
        }
    }

    if count == 0 {
        return (0, count, buffer);
    }    

    (row[row.len() / 2], count, buffer)
}

fn evalue_page_with_correction(rules: &HashMap<i64, (Vec<i64>, Vec<i64>)>, page: &i64, left: &Vec<i64>, right: &Vec<i64>, mut buffer: Vec<String>) -> (bool, Vec<i64>, Vec<String>) {
    let rules = rules.get(&page).expect("Not registered");

    let mut last_index = None;
    let mut fix = Vec::new();
    let mut status = true;

    for (i, n) in right.iter().enumerate() {
        if rules.0.contains(&n) {
            last_index = Some(i);
            status = false;
        }
    }

    if let Some(i) = last_index {
        let left_fragment = &right[0..i+1];
        let right_fragment = &right[i+1..right.len()];
        fix = [left, left_fragment, &[*page], right_fragment].concat();
        buffer.push(format!("{} | {} {} {}",
            utils::ConsoleColors::CONSOLE_SUCESS.wrap(format!("{:?}", left)),
            utils::ConsoleColors::CONSOLE_POWER.wrap(format!("{:?}", left_fragment)),
            utils::ConsoleColors::CONSOLE_RESULT.wrap(format!("-> {page} +", )),
            utils::ConsoleColors::CONSOLE_POWER.wrap(format!("{:?}", right_fragment)),
        ));
    }

    return (status, fix, buffer);
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn parse_data(content: &str) -> (HashMap<i64, (Vec<i64>, Vec<i64>)>, Vec<Vec<i64>>) {
    let normalized_content = content.replace("\r\n", "\n");
    let fragments = normalized_content.split("\n\n").collect::<Vec<&str>>();
    if fragments.len() != 2 {
        panic!("Bad format")
    }

    let rules = parse_rules(fragments[0]);
    let pages = parse_pages(fragments[1]);

    (rules, pages)
}

fn parse_rules(content: &str) -> HashMap<i64, (Vec<i64>, Vec<i64>)> {
    let mut rules: HashMap<i64, (Vec<i64>, Vec<i64>)> = HashMap::new();

    for line in content.split("\n") {
        let fragments = line.trim()
            .split("|")
            .map(|f| f.parse::<i64>().expect("Not a number"))
            .collect::<Vec<i64>>();
        if fragments.len() < 1 && fragments.len() > 2 {
            panic!("Bad format")
        }

        let left = fragments[0];
        let right = fragments[1];


        if !rules.contains_key(&left) {
            rules.insert(left, (Vec::new(), Vec::new()));
        }

        rules.get_mut(&left)
            .expect("Cannot happend").1
            .push(right);


        if !rules.contains_key(&right) {
            rules.insert(right, (Vec::new(), Vec::new()));
        }

        rules.get_mut(&right)
            .expect("Cannot happend").0
            .push(left);
    }

    rules
}

fn parse_pages(content: &str) -> Vec<Vec<i64>> {
    content.split("\n")
        .map(|l| l.trim()
            .split(",")
            .filter(|n| !n.is_empty())
            .map(|s| s.parse::<i64>().expect("Not a number"))
            .collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>()
}