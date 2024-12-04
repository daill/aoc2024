static WORD: &[char] = &['X', 'M', 'A', 'S'];

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::path::absolute;
use std::path::Component::ParentDir;

fn read_from_file() -> (usize, Vec<char>) {
    let mut file = File::open("inputs");
    let result: (usize, Vec<char>)  = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut inputs: Vec<char> = Vec::new();
            let mut count = 0;
            for line in lines {
                if let Ok(line) = line {
                    count = line.len();
                    inputs.append(&mut line.chars().collect());

                }
            }
            (count, inputs)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn dir_calc_task1(count: i32, index: i32, direction: usize, len: usize) -> i32 {
    let mut result = 0;
    //let linepos:i32 = (index % count);
    // 1 2 3
    // 4 0 5
    // 6 7 8

    result = match direction {
        1 => index-count-1,
        2 => index-count,
        3 => index-count+1,
        4 => index-1,
        5 => index+1,
        6 => index+count-1,
        7 => index+count,
        8 => index+count+1,
        _ => 0
    };

    if index%count == 0 && [1,4,6].contains(&direction){
        result = -1;
    }
    if index < count && [1,2,3].contains(&direction){
        result = -1;
    }
    if (index+1)%count == 0 && [3,5,8].contains(&direction){
        result = -1;
    }

    if result < 0 || result >= len as i32 {
        result = -1;
    }
    result
}

fn dir_calc_task2(count: i32, index: i32, direction: usize, len: usize) -> i32 {
    let mut result = 0;
    //let linepos:i32 = (index % count);
    // 1 0 2
    // 0 0 0
    // 3 0 4

    result = match direction {
        1 => index-count-1,
        2 => index-count+1,
        3 => index+count-1,
        4 => index+count+1,
        _ => 0
    };

    result
}


fn check_surrounding_area_task1(count: i32, start: i32, input: &Vec<char>, direction: usize, next_char: usize) -> (i32, bool) {
    let mut res = (0, false);

    if next_char == WORD.len() {
        return (1, true);
    }


    if direction == 0 {
        // check 6 possible ways
        let mut sum = 0;
        for i in 1..=8 {
            let c = dir_calc_task1(count, start, i, input.len());
            if c < 0 {
                continue;
            }
            if let Some(index) = input.get(c as usize) {
                if index.eq(&WORD[next_char]) {
                    res = check_surrounding_area_task1(count, c, input, i, next_char + 1);
                    if res.1 {
                        sum += 1;
                        println!("w/o dir {:?} {:?} {:?} {:?} {:?}", c/count, c%count, &WORD[next_char], i,c);
                    }
                }
            }
        }
        (sum, res.1)
    } else {
        // follow direction
        let c = dir_calc_task1(count, start, direction, input.len());
        if c < 0 {
            return (0, false);
        }
        if let Some(index) = input.get(c as usize) {
            if index.eq(&WORD[next_char]) {
                res = check_surrounding_area_task1(count, c, input, direction, next_char + 1);
                if res.1 {
                    println!("with dir {:?} {:?} {:?} {:?} {:?}", c/count, c%count, &WORD[next_char], direction, c);
                }
            }
        }
        res
    }
}

fn check_surrounding_area_task2(count: i32, start: i32, input: &Vec<char>) -> (i32,bool) {
    let mut res = (0, false);


    // check 6 possible ways
    let mut sum = 0;
    // MM SM MS SS
    // SS SM MS SS
    let top_l = dir_calc_task2(count, start, 1, input.len()) as usize;
    let top_r = dir_calc_task2(count, start, 2, input.len()) as usize;
    let bot_l = dir_calc_task2(count, start, 3, input.len()) as usize;
    let bot_r = dir_calc_task2(count, start, 4, input.len()) as usize;

    if (input[top_l] == input[top_r] && input[top_l] == 'M') && (input[bot_l] == input[bot_r] && 'S' == input[bot_l]) || (input[top_l] == input[top_r] && input[top_l] == 'S') && (input[bot_l] == input[bot_r] && 'M' == input[bot_l]) {
        println!("top  {:?} {:?} {:?} {:?} {:?} {:?}", top_l as i32 % count, top_l as i32 / count, top_r as i32 % count, top_r as i32 / count, input[top_l], input[top_r]);
        println!("center  {:?} {:?} ", start as i32 % count, start as i32 / count);
        println!("bot  {:?} {:?} {:?} {:?} {:?} {:?}", bot_l as i32 % count, bot_l as i32 / count, bot_r as i32 % count, bot_r as i32 / count, input[bot_l], input[bot_r]);
        return (1, true);
    }
    if (input[top_l] == input[bot_l] && input[top_l] == 'M') && (input[top_r] == input[bot_r] && 'S' == input[bot_r]) || (input[top_l] == input[bot_l] && input[top_l] == 'S') && (input[top_r] == input[bot_r] && 'M' == input[top_r]) {
        println!("top  {:?} {:?} {:?} {:?} {:?} {:?}", top_l as i32 % count, top_l as i32 / count, top_r as i32 % count, top_r as i32 / count, input[top_l], input[top_r]);
        println!("center  {:?} {:?} ", start as i32 % count, start as i32 / count);
        println!("bot  {:?} {:?} {:?} {:?} {:?} {:?}", bot_l as i32 % count, bot_l as i32 / count, bot_r as i32 % count, bot_r as i32 / count, input[bot_l], input[bot_r]);
        return (1, true);
    }
    res
}

fn do_task_one(count: i32, input: &Vec<char>) {
    // "idea" search for the x'ses and then check the surrounding area
    // first finding of second char determines the direction
    let mut sum = 0;
    for i in 0..input.len() {
        if input[i] == 'X'{
            println!("start  {:?} {:?} {:?} {:?} {:?} {:?}", i as i32/count, i as i32%count, &WORD[0], -1, -1, i);
            let res = check_surrounding_area_task1(count, i as i32, &input, 0, 1);
                sum += res.0;

        }
    }
    println!("result {:?}", sum);

}

fn do_task_two(count: i32, input: &Vec<char>) {
    // "idea" search for the x'ses and then check the surrounding area
    // first finding of second char determines the direction
    let mut sum = 0;
    for i in (count as usize)..input.len() {
        let y = i as i32;
        if (y%count == 0 || (y+1)%count == 0 || (input.len() as i32)-count < y) {
            continue
        }
        if input[i] == 'A'{
            println!("start  {:?} {:?} {:?} {:?}", i as i32/count, i as i32%count, 'A', i);
            let res = check_surrounding_area_task2(count, i as i32, &input);
            sum += res.0;

        }
    }
    println!("result {:?}", sum);
}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs.0);
    //do_task_one(inputs.0 as i32, &inputs.1);
    do_task_two(inputs.0 as i32, &inputs.1);
}
