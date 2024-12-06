use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::ptr::copy;

fn read_from_file() -> () {
    let mut file = File::open("inputs");
    let result: () = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line) = line {

                }
            }

            ()
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one() {

}

fn do_task_two() {

}

fn main() {
    let inputs = read_from_file();

    println!("{:?}", inputs);
    do_task_one();
    //do_task_two();
}
