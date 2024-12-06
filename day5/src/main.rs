static WORD: &[char] = &['X', 'M', 'A', 'S'];

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::ops::Index;
use std::path::absolute;
use std::path::Component::ParentDir;

fn read_from_file() -> (
    HashMap<i32, Vec<i32>>,
    HashMap<i32, Vec<i32>>,
    Vec<Vec<i32>>,
) {
    let mut file = File::open("inputs");
    let result: (
        HashMap<i32, Vec<i32>>,
        HashMap<i32, Vec<i32>>,
        Vec<Vec<i32>>,
    ) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut pages: Vec<Vec<i32>> = Vec::new();
            let mut is_pages = false;
            for line in lines {
                if let Ok(line) = line {
                    if line.is_empty() {
                        is_pages = true;
                        continue;
                    }
                    if is_pages {
                        pages.push(line.split(",").map(|x| x.parse().unwrap()).collect());
                    } else {
                        let rule: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
                        let mut before_vec = before.entry(rule[1]).or_insert_with(|| Vec::new());
                        before_vec.push(rule[0]);
                        let mut after_vec = after.entry(rule[0]).or_insert_with(|| Vec::new());
                        after_vec.push(rule[1]);
                    }
                }
            }
            println!("{:?} {:?}", before, after);

            (before, after, pages)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(
    before: &HashMap<i32, Vec<i32>>,
    after: &HashMap<i32, Vec<i32>>,
    pages: &Vec<Vec<i32>>,
) {
    let mut sum = 0;
    for page_id in 0..(pages.len() as i32) {
        println!("{:?}", page_id);
        let page = &pages[page_id as usize];
        let mut valid = true;
        for j in 0..page.len() as i32 {
            println!("page {:?}", page);

            let b = &page[..(j as usize)];
            let a = &page[(j as usize) + 1..];
            println!(
                "before {:?}",
                before.get(&page[j as usize]).unwrap_or(&Vec::new())
            );
            for bb in before.get(&page[j as usize]).unwrap_or(&Vec::new()) {
                if a.contains(bb) {
                    valid = false;
                    println!("error {:?} in {:?}", bb, a);
                    break;
                }
            }
            println!(
                "after {:?}",
                after.get(&page[j as usize]).unwrap_or(&Vec::new())
            );
            for aa in after.get(&page[j as usize]).unwrap_or(&Vec::new()) {
                if b.contains(aa) {
                    valid = false;
                    println!("error {:?} in {:?}", aa, b);
                    break;
                }
            }
        }
        if valid {
            println!(" valid {:?}", page);
            let mid = page.len() / 2;
            println!("{:?} adding {:?}", mid, page[mid as usize]);
            sum += page[mid as usize];
        }
    }
    println!("{}", sum);
}

fn order_rule(
    page: Vec<i32>,
    after: &HashMap<i32, Vec<i32>>,
    before: &HashMap<i32, Vec<i32>>,
) -> (bool, Vec<i32>) {
    let mut page_clone: Vec<i32> = Vec::new();
    for j in 0..page.len() as i32 {
        println!("page {:?}", page);

        let mut b = page[..(j as usize)].to_vec();
        let mut a = page[(j as usize) + 1..].to_vec();
        println!(
            "before {:?}",
            before.get(&page[j as usize]).unwrap_or(&Vec::new())
        );
        for bb in before.get(&page[j as usize]).unwrap_or(&Vec::new()) {
            if a.contains(bb) {
                // order
                println!("error {:?} in {:?}", bb, a);
                let pos = a.iter().position(|i| i == bb).unwrap() as i32;
                b.push(a.remove(pos as usize));
                page_clone.append(&mut b);
                page_clone.push(page[j as usize]);
                page_clone.append(&mut a);
                println!("swapped {} with {} in {:?}", j, pos + j, page_clone);
                return (false, page_clone);
            }
        }
        println!(
            "after {:?}",
            after.get(&page[j as usize]).unwrap_or(&Vec::new())
        );
        for aa in after.get(&page[j as usize]).unwrap_or(&Vec::new()) {
            if b.contains(aa) {
                // order
                println!("error {:?} in {:?}", aa, b);
                let pos = b.iter().position(|i| i == aa).unwrap() as i32;
                a.push(b.remove(pos as usize));
                page_clone.append(&mut b);
                page_clone.push(page[j as usize]);
                page_clone.append(&mut a);
                println!("swapped {} with {} in {:?}", j, pos + j, page_clone);
                return (false, page_clone);
            }
        }
    }
    println!("done {:?} {:?}", page_clone, page);
    (true, page)
}

fn do_task_two(
    before: &HashMap<i32, Vec<i32>>,
    after: &HashMap<i32, Vec<i32>>,
    pages: &Vec<Vec<i32>>,
) {
    let mut sum = 0;
    for page_id in 0..(pages.len() as i32) {
        println!("{:?}", page_id);
        let page = &pages[page_id as usize];
        let mut page_clone = page.clone();
        let mut valid = false;
        let mut cnt = 0;
        while !valid {
            let res = order_rule(page_clone, after, before);
            valid = res.0;
            page_clone = res.1;
            cnt += 1;
        }
        if cnt > 1 && !page_clone.is_empty() {
            println!("invalid / counted {:?} cnt {}", page, cnt);
            let mid = page_clone.len() / 2;
            println!("{:?} adding {:?}", mid, page_clone[mid]);
            sum += page_clone[mid];

        }
    }
    println!("{}", sum);
}

fn main() {
    let inputs = read_from_file();

    println!("{:?} {:?} {:?}", inputs.0, inputs.1, inputs.2);
    //do_task_one(&inputs.0, &inputs.1, &inputs.2);
    do_task_two(&inputs.0, &inputs.1, &inputs.2);
}
