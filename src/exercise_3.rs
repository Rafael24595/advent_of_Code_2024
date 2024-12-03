use crate::utils;

const HEAD: &str = "mul(";
const TAIL: &str  = ")";

const ENABLE: &str  = "do()";
const DISABLE: &str  = "don't()";

pub(crate) fn exercise_3_1() {

    utils::print_title("EXERCISE 3.1");

    let content = utils::read_to_string("EXERCISE_III_I.txt");

    let total = calculate_mults(&content);

    utils::print_result(total);
}

pub(crate) fn exercise_3_2() {

    utils::print_title("EXERCISE 3.2");

    let content = utils::read_to_string("EXERCISE_III_I.txt");

    let total = calculate_mults_with_status(&content);

    utils::print_result(total);

}

fn calculate_mults(content: &str) -> i64 {
    let mut buffer = content;
    
    let mut result = 0;
    let mut print_buffer = Vec::new();
    while buffer.len() > 0 {
        let (tuple, content) = find_mult(buffer);
        if let Some(tuple) = tuple {
            result += tuple.0 * tuple.1;
        }
        
        print_buffer.push(style_statement(tuple, result));
        
        buffer = content;
    }

    print_statement(print_buffer);

    return result;
}

fn find_mult(content: &str) -> (Option<(i64, i64)>, &str) {
    let head = content.find(HEAD);
    if head.is_none() {
        return (None, "");
    }

    let head = head.unwrap();

    let sentence_start = head + HEAD.len();
    let sentence_unclosed = &content[sentence_start..];
    
    let tail = sentence_unclosed.find(TAIL);
    if tail.is_none() {
        return (None, "");
    }

    let tail = tail.unwrap();

    let sentence_end = tail + TAIL.len();
    let rest = &sentence_unclosed[sentence_end..];

    let sentence_content = &sentence_unclosed[..tail];

    match make_tuple(sentence_content) {
        Ok(numbers) => (Some((numbers[0], numbers[1])), rest),
        Err(_) => (None, sentence_unclosed),
    }
}

fn style_statement(tuple: Option<(i64, i64)>, result: i64) -> (String, String) {
    let tuple_styled = match tuple {
        Some(tuple) => utils::ConsoleColors::CONSOLE_SUCESS.wrap(format!("{:?}", tuple)),
        None => utils::ConsoleColors::CONSOLE_FAIL.wrap(format!("{:?}", tuple))
    };

    let result_styled = utils::ConsoleColors::CONSOLE_RESULT.wrap(result);

    return (tuple_styled, result_styled);
}

fn print_statement(print_buffer: Vec<(String, String)>) {
    let (longest, _) = print_buffer.iter()
        .max_by_key(|(t, _)| t.len())
        .expect("No data to print");
    let arrow = utils::ConsoleColors::CONSOLE_BOLD.wrap("->");
    for statement in print_buffer.iter() {
        let tuple = &statement.0;
        let result = &statement.1;
        let spaces_tuple = " ".repeat(longest.len() - tuple.len());
        println!("{tuple}{spaces_tuple} {arrow} [{result}]");
    }
}


/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/


fn calculate_mults_with_status(content: &str) -> i64 {
    let mut buffer = content;
    
    let mut result = 0;
    let mut status = true;
    let mut print_buffer = Vec::new();
    while buffer.len() > 0 {
        let (tuple, part_status, content) = find_mult_with_status(buffer);
        status = part_status.unwrap_or(status);
        if let (true, Some(tuple)) = (status, tuple) {
            result += tuple.0 * tuple.1;
        }

        print_buffer.push(style_status_statement(tuple, status, result));

        buffer = content;
    }

    print_status_statement(print_buffer);

    return result;
}

fn find_mult_with_status(content: &str) -> (Option<(i64, i64)>, Option<bool>, &str) {
    let head = content.find(HEAD);
    if head.is_none() {
        return (None, None, "");
    }

    let head = head.unwrap();

    let sentence_start = head + HEAD.len();
    let sentence_unclosed = &content[sentence_start..];

    let (status, _) = check_sentence_status(content, sentence_start);
    if status.is_some() && !status.unwrap() {
        return (None, status, sentence_unclosed);
    }
    
    let tail = sentence_unclosed.find(TAIL);
    if tail.is_none() {
        return (None, status, "");
    }

    let tail = tail.unwrap();

    let sentence_end = tail + TAIL.len();
    let rest = &sentence_unclosed[sentence_end..];

    let sentence_content = &sentence_unclosed[..tail];

    match make_tuple(sentence_content) {
        Ok(numbers) => (Some((numbers[0], numbers[1])), status, rest),
        Err(_) => (None, status, sentence_unclosed),
    }
}

fn check_sentence_status(content: &str, sentence_start: usize) -> (Option<bool>, &str) {
    let sentence_prefix = &content[..sentence_start];
    let enable = sentence_prefix.find(ENABLE);
    let disable = sentence_prefix.find(DISABLE);

    if enable.is_none() && disable.is_none() {
        return (None, "");
    }

    if enable.is_some() && disable.is_none() {
        return (Some(true), "");
    }

    if enable.is_none() && disable.is_some() {
        return (Some(false), "");
    }

    let enable = enable.unwrap();
    let disable = disable.unwrap();

    let mut index = disable;
    if enable > disable {
        index = enable;
    }

    let rest = &sentence_prefix[index..];

    return check_sentence_status(rest, sentence_start);
}

fn style_status_statement(tuple: Option<(i64, i64)>, status: bool, result: i64) -> (String, String, String) {
    let (tuple_styled, result_styled) = style_statement(tuple, result);

    let status_styled = match status {
        true => utils::ConsoleColors::CONSOLE_SUCESS.wrap(status),
        false => utils::ConsoleColors::CONSOLE_FAIL.wrap(status),
    };

    return (tuple_styled, status_styled, result_styled);
}

fn print_status_statement(print_buffer: Vec<(String, String, String)>) {
    let (longest, _, _) = print_buffer.iter()
        .max_by_key(|(t, _, _)| t.len())
        .expect("No data to print");
    let arrow = utils::ConsoleColors::CONSOLE_BOLD.wrap("->");
    for statement in print_buffer.iter() {
        let tuple = &statement.0;
        let status = &statement.1;
        let result = &statement.2;
        let spaces_tuple = " ".repeat(longest.len() - tuple.len());
        let spaces_status = " ".repeat(utils::ConsoleColors::CONSOLE_FAIL.wrap("false").len() - status.len());
        println!("{tuple}{spaces_tuple} {arrow} {status}{spaces_status} [{result}]");
    }
}


/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/


fn make_tuple(content: &str) -> Result<Vec<i64>, ()> {
    let mut numbers = Vec::new();
    for number_str in content.split(",") {
       match number_str.trim().parse::<i64>() {
           Ok(number) => numbers.push(number),
           _ => return Err(())
       }
    } 
    if numbers.len() != 2 {
        return Err(())
    }
    return Ok(numbers);
}