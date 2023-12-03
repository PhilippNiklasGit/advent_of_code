use core::cmp;
fn main() {
    let input = include_str!("input");
    println!("{}", part1(input));
}

//                y      len 
fn calc_dist(num:(usize, Vec<usize>), spec_char: (usize, usize)) -> usize {
    let (num_y, num_x_arr) = num;
    let mut dist = usize::MAX;
    for num_x in num_x_arr {
        let y = num_y.abs_diff(spec_char.0);
        let x = num_x.abs_diff(spec_char.1);
        let cur_dist = ((y.pow(2) + x.pow(2)) as f64).sqrt();
        let cur_dist = cur_dist.floor();
        dist = cmp::min(cur_dist as usize, dist);
    }
    dist

}

fn calc_square(y:isize,x:isize) -> Vec<(usize, usize)> {
    let mut square_vec:Vec<(usize,usize)> = [].to_vec();
    for i in -1..=1 {
        for j in -1..=1 {
            if y+i>=0 && j+x>=0 {
                square_vec.push(((y+i) as usize, (j+x) as usize));
            }
        }
    }

    square_vec
}

fn part1(input: &str) -> usize {
    let shematics = input.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let spec_char_pos = input.lines()
        .into_iter()
        .map(|line| line.chars()
             .enumerate()
             .filter(|c| !c.1.is_numeric()&&!(c.1=='.'))
             .map(|c| c.0)
             .collect::<Vec<usize>>()
        )
        .enumerate()
        .filter(|pos_arr| !pos_arr.1.is_empty())
        .collect::<Vec<(usize,Vec<usize>)>>();
    let mut spec_char_fields:Vec<(usize,usize)> = [].to_vec();
    
    for y in spec_char_pos.clone() {
        for x in y.1 {
            for pos in calc_square(y.0 as isize, x as isize) {
                spec_char_fields.push(pos);
            }
        }
    }
    


    let mut num_arr:Vec<(usize,Vec<usize>,String)> = [].to_vec();
    for y in 0..shematics.len() { // iter through lines
        let mut last_num_pos = 0;
        let mut cur_num: (usize, Vec<usize>, String) = (y, [].to_vec(),String::new());
        for x in 0..shematics[y].len() { // iter through specific chars in line 
            if shematics[y][x].is_numeric() {
                if last_num_pos==x {
                    cur_num.1.push(x);
                    cur_num.2.push(shematics[y][x]);
                    
                    last_num_pos+=1;
                } else {
                    if cur_num.2!=String::from("") {
                        num_arr.push(cur_num.clone());
                    }
                    //cur_num = String::from(shematics[y][x]);
                    cur_num = (y, [x].to_vec(),String::from(shematics[y][x]));
                    last_num_pos=x+1;
                }
            }
        }
        if cur_num.2!=String::from("") {
            num_arr.push(cur_num);
        }
    }


    for pos in spec_char_pos.clone() {
        let pos_y = pos.0;
        for pos_x in pos.1.clone() {
            for num in num_arr.clone() {
                if calc_dist((num.0,num.1.clone()), (pos_y, pos_x))<2{
                    println!("pos: {:?} | num: {:?} ", (pos_y, pos_x), (num.0,num.1.len()));
                    println!("num: {}", num.2);
                }
            } 
        }
    }

    let mut final_num_arr = [].to_vec();
    for num in num_arr {
        for x in num.1 {
            if spec_char_fields.contains(&(num.0, x)) {
                final_num_arr.push(num.2);
                break;
            }
        }
    }
    println!("{:?}", final_num_arr.clone());


    final_num_arr.into_iter()
                .fold(0,|acc, x| {
                    let new_x = x.parse::<usize>().unwrap();
                    acc+new_x
                })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            );
        assert_eq!(4361, result);
    }
}
