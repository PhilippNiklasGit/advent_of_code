fn main() {
    let input = include_str!("input");
    println!("{}", part2(input));
}

fn calc_square(y:isize,x:isize) -> Vec<(usize, usize)> {
    let mut square_vec:Vec<(usize,usize)> = [].to_vec();
    for i in -1..=1 {
        for j in -1..=1 {
            println!("y {} : x {}", y+i, x+j);
            if y+i>=0 && j+x>=0 {
                square_vec.push(((y+i) as usize, (j+x) as usize));
            }
        }
    }

    square_vec
}

fn part2(input: &str) -> usize {
    let shematics = input.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let spec_char_pos = input.lines()
        .into_iter()
        .map(|line| line.chars()
             .enumerate()
             .filter(|c| c.1=='*')
             .map(|c| c.0)
             .collect::<Vec<usize>>()
        )
        .enumerate()
        .filter(|pos_arr| !pos_arr.1.is_empty())
        .collect::<Vec<(usize,Vec<usize>)>>();
    //let mut spec_char_fields: = [].to_vec();
    
    let mut spec_char_fields: Vec<Vec<(usize,usize)>> = [].to_vec();
    for y in spec_char_pos {
        let mut gear_pos_array = [].to_vec();
        for x in y.1 {
            for pos in calc_square(y.0 as isize, x as isize) {
                gear_pos_array.push(pos);
            }
        }
        spec_char_fields.push(gear_pos_array);
        
    }

    let mut num_arr:Vec<(usize,Vec<usize>,String)> = [].to_vec();
    for y in 0..shematics.len() { // iter through lines
        let mut last_num_pos = 0;
        //let mut cur_num = String::new();
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
    /*
    let mut final_num_arr = [].to_vec();
    for num in num_arr {
        for x in num.1 {
            if spec_char_fields.contains(&(num.0, x)) {
                final_num_arr.push(num.2);
                break;
            }
        }
    }
    
    final_num_arr.into_iter()
                .fold(0,|acc, x| {
                    let new_x = x.parse::<usize>().unwrap();
                    acc+new_x
                })
    */
    for gear_radius in spec_char_fields {
        for gear_field in gear_radius {}
    }
    //println!("{spec_char_fields:?}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
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
