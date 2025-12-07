use std::{collections::HashMap, fs};

pub fn p1() {
    let input: String = fs::read_to_string("./inputs/day7.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}",grid[0]);
    let start = grid[0].iter().position(|f| f == &'S').unwrap();
    let mut new_grid = Vec::new();
    let mut beam_columns = vec![start];
    let mut split_counter = 0;
    let mut iter = grid.into_iter();
    iter.next().unwrap();
    for line in iter {
        let mut new_line = line.clone();
        let mut new_columns =  Vec::new();
        for incoming in beam_columns {
            match line[incoming] {
                '.' => {
                    if !new_columns.contains(&incoming) {
                        new_columns.push(incoming);
                    }
                    new_line[incoming] = '|';
                },
                '^' => {
                    if !new_columns.contains(&(incoming-1)) {
                        new_columns.push(incoming-1);
                    }
                    if !new_columns.contains(&(incoming+1)) {
                        new_columns.push(incoming+1);
                    }
                    new_line[incoming-1] = '|';
                    new_line[incoming+1] = '|';
                    split_counter+=1;
                }
                _=> {println!("ERROR"); panic!("Got false input")}
            }
        }
        new_grid.push(new_line);
        beam_columns = new_columns;
    }
    fs::write("outputs/day7.txt", adventofcode_utils::grid_to_string(new_grid)).unwrap();
    
    println!("Result: {split_counter}");

}

pub fn p2() {
        let input: String = fs::read_to_string("./inputs/day7.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}",grid[0]);
    let start = grid[0].iter().position(|f| f == &'S').unwrap();
    let mut new_grid = Vec::new();
    let mut beam_columns = HashMap::new();
    beam_columns.insert(start, 1);
    let mut split_counter: usize = 1;
    let mut iter = grid.into_iter();
    iter.next().unwrap();
    for line in iter {
        let mut new_line = line.clone();
        let mut new_columns =  HashMap::new();
        for incoming in beam_columns {
            match line[incoming.0] {
                '.' => {
                    new_columns.entry(incoming.0).and_modify(|f| *f+=incoming.1).or_insert(incoming.1);
                    new_line[incoming.0] = '|';
                },
                '^' => {
                    new_columns.entry(incoming.0+1).and_modify(|f| *f+=incoming.1).or_insert(incoming.1);
                    new_columns.entry(incoming.0-1).and_modify(|f| *f+=incoming.1).or_insert(incoming.1);
                    new_line[incoming.0-1] = '|';
                    new_line[incoming.0+1] = '|';
                    split_counter+=incoming.1;
                }
                _=> {println!("ERROR"); panic!("Got false input")}
            }
        }

        println!("{:?}", new_line.clone());
        println!("{:?}", new_columns.clone());
        println!("{split_counter}");
        new_grid.push(new_line);
        beam_columns = new_columns;
    }

    fs::write("outputs/day7p2.txt", adventofcode_utils::grid_to_string(new_grid)).unwrap();
    
    println!("Result: {split_counter}");


}