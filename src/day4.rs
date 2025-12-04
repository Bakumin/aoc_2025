use std::fs;
use adventofcode_utils::find_structures;

pub fn p1() {
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    //println!("{map:?}");
    let mut res = 0;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] != '@' {
                continue;
            }
            let mut counter = 0;
            if x > 0 {
                if y > 0 && map[x-1][y-1] == '@'{
                    counter+=1;
                }
                if map[x-1][y] == '@' {
                    counter+=1;
                }
                if y < map[x].len()-1 && map[x-1][y+1] == '@' {
                    counter+=1;
                }
            }
            if x < map.len()-1 {
                if y > 0 && map[x+1][y-1] == '@' {
                    counter+=1;
                }
                if map[x+1][y] == '@' {
                    counter+=1;
                }
                if y < map[x].len()-1 && map[x+1][y+1] == '@' {
                    counter+=1;
                }
            }

            if y> 0 && map[x][y-1]=='@' {
                counter+=1;
            }
            if y < map[x].len()-1 && map[x][y+1] == '@' {
                counter+=1;
            }

            if counter <4 {
                res+=1;
            }

        }
    }

    println!("D4P1: {res}");




}

pub fn p2() {
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    //println!("{map:?}");
    let mut res = 0;

    loop {
        let old_res = res;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] != '@' {
                continue;
            }
            let mut counter = 0;
            if x > 0 {
                if y > 0 && map[x-1][y-1] == '@'{
                    counter+=1;
                }
                if map[x-1][y] == '@' {
                    counter+=1;
                }
                if y < map[x].len()-1 && map[x-1][y+1] == '@' {
                    counter+=1;
                }
            }
            if x < map.len()-1 {
                if y > 0 && map[x+1][y-1] == '@' {
                    counter+=1;
                }
                if map[x+1][y] == '@' {
                    counter+=1;
                }
                if y < map[x].len()-1 && map[x+1][y+1] == '@' {
                    counter+=1;
                }
            }

            if y> 0 && map[x][y-1]=='@' {
                counter+=1;
            }
            if y < map[x].len()-1 && map[x][y+1] == '@' {
                counter+=1;
            }

            if counter <4 {
                res+=1;
                map[x][y] = '.';
            }

        }
    }

    println!("D4P2: {res}");
    if res == old_res {
        break;
    }
}

}