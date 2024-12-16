use std::cmp::max;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;
use std::str::FromStr;
use std::sync::atomic::fence;
use regex::Regex;

fn read_from_file() -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>, (i32, i32), Vec<char>, i32, i32) {
    let mut file = File::open("test");
    let result: (HashSet<(i32, i32)>, HashSet<(i32, i32)>, (i32, i32), Vec<char>, i32, i32) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();

            let mut walls:HashSet<(i32, i32)> = HashSet::new();
            let mut cargo:HashSet<(i32, i32)> = HashSet::new();
            let mut start:(i32, i32) = (0,0);
            let mut height = 0;
            let mut movement = false;
            let mut movs: Vec<char>=Vec::new();
            let mut width = 0;

            for line in lines {
                if let Ok(line) = line {
                    println!("{:?}", line);
                    if line.is_empty() {
                        movement = true;
                        continue;
                    }
                    if !movement {
                        width = line.len() as i32;
                        line.chars().into_iter().enumerate().for_each(|(i, x)| {
                            if x == '#' {
                                walls.insert((i as i32, height));
                            }
                            if x == '@' {
                                start = (i as i32, height);
                            }
                            if x == 'O' {
                                cargo.insert((i as i32, height));
                            }

                        });
                        height += 1;
                    } else {
                        movs.append(&mut line.chars().collect::<Vec<char>>());
                    }

                }

            }
            (walls, cargo, start, movs, width, height)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn calc(cargo: &HashSet<(i32, i32)>) -> i64{
    let mut sum: i64 = 0;

    for c in cargo {
        sum += (c.1 as i64 * 100) + c.0 as i64;
    }

    sum
}

fn check(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>, start: &(i32, i32), mov: (i32, i32), width: i32, height: i32, changelog: &mut Vec<(i32, i32)>) -> bool {
    let mut valid = false;
    let np = (start.0+mov.0, start.1+mov.1);

    if walls.contains(&np) {
        valid = false;
    } else if cargo.contains(&np) {
        valid = check(walls, cargo, &np, mov, width, height, changelog);
        changelog.push(*start);
    } else {
        changelog.push(*start);
        valid = true;
    }


    valid
}

fn do_task_one(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>, start: &(i32, i32), movements: &Vec<char>, width: i32, height: i32) {
    let mut botpos = start.clone();
    let mut cargoc = cargo.clone();


    for m in movements {
        //printmap(walls, &cargoc, &botpos, width, height);
        let mut newcargo = cargoc.clone();
        let mov:(i32, i32) = match m {
            '>' => (1,0),
            '^' => (0,-1),
            '<' => (-1,0),
            'v' => (0,1),
            _ => panic!("mov not found")
        };

        let newpos =  &(botpos.0+mov.0, botpos.1+mov.1);

        if walls.contains(newpos) {
            // do nothing
            continue;
        }

        let mut valid = true;
        if cargoc.contains(&newpos) {
            let mut changelog: Vec<(i32, i32)> = Vec::new();
            valid = check(walls, &cargoc, newpos, mov, width, height, &mut changelog);
            //println!("{:?} {:?}", valid, &changelog);
            let mut mergeset: HashSet<(i32, i32)> = HashSet::new();
            if valid {
                for change in changelog {
                    newcargo.remove(&change);
                    mergeset.insert((change.0+mov.0, change.1+mov.1));
                }
                newcargo.extend(mergeset);
                cargoc = newcargo;
            }
        }

        if valid {
            botpos = *newpos;
        }


        //println!("{:?} {:?}", botpos, cargoc);
    }

    printmap(walls, &cargoc, &botpos, width, height);
    let mut sum: i64 = 0;
    cargoc.iter().for_each(|p| sum += (p.1 as i64 * 100) + p.0 as i64 );
    println!("{:?}", sum);
}



fn printmap(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>, start: &(i32, i32), width: i32, height: i32) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for w in walls {
        map[w.1 as usize][w.0 as usize] = '#';
    }

    for c in cargo {
        map[c.1 as usize][c.0 as usize] = 'O';
    }
    map[start.1 as usize][start.0 as usize] = '@';

    for l in map {
        println!("{:?}", l);
    }

}

fn check_t2(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>, start: &(i32, i32), mov: (i32, i32), width: i32, height: i32, changelog: &mut Vec<(i32, i32)>, crates: &HashMap<(i32, i32), (i32, i32)>) -> bool {
    let mut valid = false;
    let np = (start.0+mov.0, start.1+mov.1);


    if mov != (-1,0) && mov != (1,0) {
        // when up and down check the crate
        if let Some(cp) = crates.get(&start) {
            let mut cpn = (cp.0+mov.0, cp.1+mov.1);

            if walls.contains(&np) || walls.contains(&cpn) {
                valid = false;
            } else if cargo.contains(&np) || cargo.contains(&cpn) {

                if cargo.contains(&np) {
                    // when a point has been found, we need it's neighbour
                    valid = check_t2(walls, cargo, &np, mov, width, height, changelog, &crates);
                    changelog.push(*start);

                }
                if cargo.contains(&cpn) {
                    // when a point has been found, we need it's neighbour
                    valid = check_t2(walls, cargo, &cpn, mov, width, height, changelog, &crates);
                    changelog.push(*cp);

                }
            } else {
                changelog.push(*start);
                changelog.push(*cp);
                valid = true;
            }
        } else {
            valid = false;
        }

    } else {
        if walls.contains(&np){
            valid = false;
        } else if cargo.contains(&np) {
            valid = check_t2(walls, cargo, &np, mov, width, height, changelog, &crates);
            changelog.push(*start);
        } else {
            changelog.push(*start);
            valid = true;
        }
    }

    valid
}

fn do_task_two(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>, start: &(i32, i32), movements: &Vec<char>, width: i32, height: i32) {
    let mut botpos = start.clone();
    botpos = (botpos.0*2, botpos.1);

    let (nw, nc, mut crates) = expand(walls, cargo);
    let mut cargoc = nc.clone();

    printmap(&nw, &cargoc, &botpos, width*2, height);

    for mcounter in 0..movements.len() {
        let m = movements[mcounter];

        let mut newcargo = cargoc.clone();
        let mov:(i32, i32) = match m {
            '>' => (1,0),
            '^' => (0,-1),
            '<' => (-1,0),
            'v' => (0,1),
            _ => panic!("mov not found")
        };

        let newpos =  &(botpos.0+mov.0, botpos.1+mov.1);

        if nw.contains(newpos) {
            // do nothing
            continue;
        }

        let mut valid = true;
        if cargoc.contains(&newpos) {
            let mut changelog: Vec<(i32, i32)> = Vec::new();
            valid = check_t2(&nw, &cargoc, newpos, mov, width, height, &mut changelog, &crates);
            //println!("{:?} {:?}", valid, &changelog);
            let mut mergeset: HashSet<(i32, i32)> = HashSet::new();
            //println!("{:?}\n {:?} {:?} {:?}", changelog, crates, start, valid);
            if valid {
                for change in changelog {
                    newcargo.remove(&change);
                    mergeset.insert((change.0+mov.0, change.1+mov.1));

                    if let Some(c) = crates.remove(&change) {
                        crates.remove(&c);
                        //mergeset.insert((c.0+mov.0, c.1+mov.1));

                        crates.insert((change.0+mov.0, change.1+mov.1), (c.0+mov.0, c.1+mov.1));
                        crates.insert((c.0+mov.0, c.1+mov.1), (change.0+mov.0, change.1+mov.1));
                    }
                }
                //println!("{:?} \n {:?}", newcargo, mergeset);
                newcargo.extend(mergeset);
                cargoc = newcargo;
            }
        }

        if valid {
            botpos = *newpos;
        }


        //println!("{:?} {:?}", botpos, cargoc);
    }

    printmap(&nw, &cargoc, &botpos, width*2, height);
    let mut sum: i64 = 0;
    cargoc.iter().for_each(|p| sum += (p.1 as i64 * 100) + p.0 as i64 );
    println!("{:?}", sum);
}

fn expand(walls: &HashSet<(i32, i32)>, cargo: &HashSet<(i32, i32)>) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>, HashMap<(i32, i32), (i32, i32)>) {
    let mut new_walls = HashSet::new();
    let mut new_cargo = HashSet::new();
    let mut crates: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for w in walls {
        let mut newx = w.0 *2;
        if w.0 == 0 {
            newx = 0;
        }

        new_walls.insert((newx, w.1));
        new_walls.insert((newx+1, w.1));


    }

    for c in cargo {
        let mut newx = c.0 *2;
        if c.0 == 0 {
            newx = 0;
        }


        new_cargo.insert((newx, c.1));
        new_cargo.insert((newx+1, c.1));
        crates.insert((newx, c.1), (newx+1, c.1));
        crates.insert((newx+1, c.1), (newx, c.1));

    }

    (new_walls, new_cargo, crates)
}


fn main() {
    let mut inputs = read_from_file();

    println!("{:?} {:?} {:?} {:?}", &inputs.0, &inputs.1, &inputs.2, &inputs.3);
    //do_task_one(&inputs.0, &inputs.1, &inputs.2, &inputs.3, inputs.4, inputs.5);
    do_task_two(&inputs.0, &inputs.1, &inputs.2, &inputs.3, inputs.4, inputs.5);
}

