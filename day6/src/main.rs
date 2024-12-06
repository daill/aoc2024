use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::ptr::copy;

fn read_from_file() -> ((i32, i32, i32, i32), Vec<(i32, i32)>) {
    let mut file = File::open("inputs");
    let result: ((i32, i32, i32, i32), Vec<(i32, i32)>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut map: Vec<(i32, i32)> = Vec::new();
            let mut width = 0;
            let mut height = 0;
            let mut px = 0;
            let mut py = 0;
            for line in lines {
                if let Ok(line) = line {
                    height += 1;
                    if width == 0 {
                        width = line.len() as i32;
                    }
                    line.chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .enumerate()
                        .for_each(|(i, c)| {
                            if *c == '#' {
                                map.push((i as i32, height - 1 as i32));
                            }
                            if *c == '^' {
                                px = i as i32;
                                py = height - 1 as i32;
                            }
                        });
                }
            }
            println!("{:?} {:?} {:?} {:?} {:?}", width, height, px, py, map);

            ((width, height, px, py), map)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

// 0 = up
// 1 = right
// 2 = down
// 3 = left
fn move_player(
    position: (i32, i32),
    map: &Vec<(i32, i32)>,
    width: i32,
    height: i32,
    direction: i32,
) -> ((i32, i32), i32) {

    // up or right
    // check next step
    let next = match direction {
        0 => (position.0, position.1 - 1),
        1 => (position.0 + 1, position.1),
        3 => (position.0 - 1, position.1),
        2 => (position.0, position.1 + 1),
        _ => panic!("Unknown direction"),
    };

    if map.contains(&next) {
        // turn right
        if direction == 3 {
            return (position, 0);
        }
        return (position, direction + 1);
    }

    (next, direction)
}

fn do_task_one(player: (i32, i32), map: &Vec<(i32, i32)>, width: i32, height: i32) {
    let mut position = player;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut direction = 0;
    seen.insert(player);
    while position.0 > 0 && position.0 < width && position.1 > 0 && position.1 < height {
        (position, direction) = move_player(position, map, width, height, direction);
        println!(
            "Position: {:?} map: {:?} width: {:?} height: {:?} direction: {:?}",
            position, map, width, height, direction
        );
        seen.insert(position);
    }
    println!("{:?} {:?}", seen, seen.len() - 1);
}

fn do_task_two(player: (i32, i32), map: &Vec<(i32, i32)>, width: i32, height: i32) {
    let mut position = player;
    let mut seen: Vec<(i32, i32)> = Vec::new();
    let mut direction = 0;
    while position.0 >= 0 && position.0 < width && position.1 >= 0 && position.1 < height {
        (position, direction) = move_player(position, map, width, height, direction);
        if !seen.contains(&position) {
            seen.push(position);
        }
    }

    println!("{:?} {:?}", seen, seen.len());


    let mut sum = 0;
    for point in seen {
        let mut position = player;
        let mut mapc = map.clone();
        let mut direction = 0;
        mapc.push(point);
        let mut cycle: Vec<((i32, i32), i32)> = Vec::new();
        while position.0 >= 0 && position.0 < width && position.1 >= 0 && position.1 < height {
            (position, direction) = move_player(position, &mapc, width, height, direction);
            if !cycle.contains(&(position, direction)) {
                cycle.push((position, direction));
            } else {
                //println!("cycle? {:?}", cycle);
                println!("{:?}", point);
                sum += 1;
                break;
            }
        }
    }

    println!("{:?}", sum);

}

fn main() {
    let inputs = read_from_file();

    println!("{:?}", inputs);
    //    do_task_one((inputs.0 .2, inputs.0 .3), &inputs.1, inputs.0 .0,inputs.0 .1,);
    do_task_two(
        (inputs.0 .2, inputs.0 .3),
        &inputs.1,
        inputs.0 .0,
        inputs.0 .1,
    );
}
