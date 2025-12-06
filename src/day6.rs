use std::fs;

pub fn p1(){
    let input = fs::read_to_string("./inputs/day6.txt").unwrap();
    let grid: Vec<Vec<&str>> = input.lines().map(|line| line.split_whitespace().collect()).collect();
    let mut result = 0;
    println!("{grid:?}");

    for i in 0..grid[0].len() {
        let operand = grid[grid.len()-1][i];
        let mut res: isize = 0;


        for j in 0..grid.len()-1 {
            match operand {
                "*" => {
                    res = res.max(1);
                    res = res * grid[j][i].parse::<isize>().unwrap();
                },
                "+" => {
                    res+= grid[j][i].parse::<isize>().unwrap();
                }
                _ => {
                    print!("ERROR");
                }
            }
        }
        result += res;
    }
    println!("{result}");

}

pub fn p2(){
    let input = fs::read_to_string("./inputs/day6.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;
    println!("{grid:?}");
    let mut pointers: Vec<usize> = Vec::new();
    pointers = grid[grid.len()-1].iter().enumerate().filter(|f| f.1 != &' ').map(|f| f.0).collect();
    println!("{pointers:?}");

    for i in pointers.iter().enumerate() {
        let operand = grid[grid.len()-1][*i.1];
        let mut inner_results = Vec::new();
        let mut res = 0;
        let next = {
            if pointers.len() > i.0+1 {
                pointers[i.0+1]-1
            }
            else {
                grid[0].len()
            }
        };

        for j in 0..grid.len()-1 {
            for c in (*i.1..next).enumerate() {
                if inner_results.len() < c.0 +1 {
                    inner_results.push(String::from(""));
                }
                inner_results[c.0].push(grid[j][c.1]);
            }
            println!("inner_results:\n{inner_results:?}");
        }
        println!("outer_inner_results:\n{inner_results:?}");
    for num in inner_results.iter() {        
        match operand {
                '*' => {
                    res = res.max(1);
                    res = res * num.trim().parse::<isize>().unwrap();
                },
                '+' => {
                    res+= num.trim().parse::<isize>().unwrap();
                }
                _ => {
                    print!("ERROR");
                }
            }
        }
        result+=res;
    }
    println!("{result}");
/*
    for i in 0..grid[0].len() {
        let operand = grid[grid.len()-1][i];
        let mut inner_results = Vec::new();
        let mut res = 0;
        for j in 0..grid.len()-1 {
            let cs = grid[j][i].chars();
            for c in cs.enumerate() {
                if inner_results.len() < c.0 +1 {
                    inner_results.push(String::from(""));
                }
                inner_results[c.0].push(c.1);

            }

        }
        println!("inner_results:\n{inner_results:?}");
        for num in inner_results.iter() {        
        match operand {
                "*" => {
                    res = res.max(1);
                    res = res * num.parse::<isize>().unwrap();
                },
                "+" => {
                    res+= num.parse::<isize>().unwrap();
                }
                _ => {
                    print!("ERROR");
                }
            }

}
    result+=res;
    }
    println!("{result}");
*/
}