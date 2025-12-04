use std::fs;

pub fn p1() {
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    let lines = input.lines();
    let values: Vec<Vec<u32>> = lines.map(|line| line.chars().map(|c| char::to_digit(c, 10).unwrap()).collect()).collect();
    let mut total = 0;
    for v in values {
        let left = {
            let mut m = 0;
            let mut res = 0;
            for i in 0..(v.len()-1) {
                if v[i] > m {
                    m = v[i];
                    res = i;
                }
            }
            res
        };
        let right = {
            let mut m = 0;
            let mut res = 0;
            for i in left+1..(v.len()) {
                if v[i] > m {
                    m = v[i];
                    res = i;
                }
            }
            res
        };
        let res = (v[left]*10) + v[right];
        //println!("{res}");
        total += res;

    }

    println!("{total}")


}
    
pub fn p2() {
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    let lines = input.lines();
    let values: Vec<Vec<u32>> = lines.map(|line| line.chars().map(|c| char::to_digit(c, 10).unwrap()).collect()).collect();
    let mut total = 0;
    for v in values {
        let mut indices = vec![0;12];
        
        for j in 0..12 {
            let vv: usize = {
                let mut m = 0;
                let mut res = 0;
                let prev = {
                    if j == 0 {
                        0
                    }
                    else {
                        indices[j-1]+1
                    }
                };
                for i in prev..(v.len()-(11-j)) {
                    if v[i] > m {
                        //println!("{} was larger than {m} so {res} was set to {i}", v[i]);
                        m = v[i];
                        res = i;
                    }
                }
                res
            };
            indices[j] = vv;
        }
        let res = {
            let mut r = 0;
            for vv in indices.iter().enumerate() {
                //println!("{vv:?}, {}", v[*vv.1]);
                let pos = vv.0;
                let index = *vv.1;
                let arr_val = v[index] as usize;
                //println!("Pos is {pos}, Index is {index}, arr_val is {arr_val}, adds up to: {}", arr_val*(10_usize.pow(12-pos as u32)));


                r+=arr_val*(10_usize.pow(12-pos as u32));
                //println!("r was set to: {r}");
            }
            r
        };
        //println!("{res}");
        total += res;

    }

    println!("{}", total / 10)


}