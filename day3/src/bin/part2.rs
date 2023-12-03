use core::cmp;
fn main() {
    let input = include_str!("input");
    println!("{}", part2(input));
}

fn is_adjacent(num: (usize,usize), size: usize, spec_char: (usize, usize)) -> bool {
    let (num_y, num_x) = num;
    let (spec_char_y, spec_char_x) = spec_char;


    let mut dist = usize::MAX;
    for num_x in num_x..num_x+size {
        let y = num_y.abs_diff(spec_char_y);
        let x = num_x.abs_diff(spec_char_x);

        let cur_dist = ((y.pow(2) + x.pow(2)) as f64).sqrt();
        let cur_dist = cur_dist.floor();

        dist = cmp::min(cur_dist as usize, dist);
    }
    dist<2
}


fn part2(input: &str) -> usize {
    let new_chars = input.lines().enumerate().map(|(y, line)|{
        line.chars()
            .enumerate()
            .filter(|(_,character)| character!=&'.')
            .map(move |(x,character)| {
            ((y,x),character)
        })
    })
    .flatten()
    .collect::<Vec<((usize,usize),char)>>();

    let spec_chars = new_chars.clone().into_iter()
                            .filter(|character| !character.1.is_numeric())
                            .collect::<Vec<((usize,usize),char)>>();

    let num_arr = new_chars.into_iter()
                            .filter(|character| character.1.is_numeric())
                            .collect::<Vec<((usize,usize),char)>>();

    let mut num_iter = num_arr.into_iter().peekable();
    let mut num_arr: Vec<((usize,usize),String)> = [].to_vec();
    let mut last_num:isize = -2;

    while let Some(character) = num_iter.next(){
        let (_,character_x) = character.0;
        if character_x as isize==last_num+1 {
            let mut last = num_arr.pop().unwrap();
            last.1.push(character.1);
            num_arr.push(last);
            last_num+=1;
        } else {
            num_arr.push((character.0, character.1.to_string()));
            last_num=character_x as isize;
        }
    };

    spec_chars.into_iter()
        .flat_map(|spec_char| {
            let nums = num_arr.clone().into_iter()
                .filter(|num| is_adjacent(num.0,num.1.len(), spec_char.0))
                .collect::<Vec<((usize,usize),String)>>();

            if nums.len()==2 {
                let num1 = nums[0].1.parse::<usize>().unwrap();
                let num2 = nums[1].1.parse::<usize>().unwrap();
                Some(num1*num2)
            } else {
                None
            }
        })
        .sum::<usize>()
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
        assert_eq!(467835, result);
    }
}

