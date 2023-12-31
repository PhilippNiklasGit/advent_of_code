use core::cmp;

fn main() {
    let input = include_str!("input");
    println!("{}", part2(input));
}

fn is_adjacent(num: ShemeString, spec_char: ShemeCharacter) -> bool {
    let (num_y, num_x) = (num.pos.y,num.pos.x);
    let (spec_char_y, spec_char_x) = (spec_char.pos.y,spec_char.pos.x);

    for num_x in num_x..num_x+num.string.len(){
        let y = num_y.abs_diff(spec_char_y);
        let x = num_x.abs_diff(spec_char_x);
        if y<=1&&x<=1 {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
struct Position {
    y: usize,
    x: usize
}
impl Position {
    pub fn new(y: usize, x: usize) -> Self {
        Self {
            y,
            x
        }
    }
}

#[derive(Debug, Clone)]
struct ShemeCharacter {
    pos: Position,
    pub character: char 
}

impl ShemeCharacter {
    pub fn new(y: usize,x: usize, character: char) -> Self {
        Self {
            pos: Position::new(y,x),
            character
        }
    }
}

#[derive(Debug, Clone)]
struct ShemeString {
    pos: Position,
    pub string: String  
}

impl ShemeString {
    pub fn from_character(ch: ShemeCharacter) -> Self {
        Self {
            pos: ch.pos,
            string: String::from(ch.character)
        }
    }
}

fn part2(input: &str) -> usize {
    let (num_arr,spec_chars):(Vec<ShemeCharacter>,Vec<ShemeCharacter>) = input.lines().enumerate()
        .map(|(y, line)|{
            line.chars()
                .enumerate()
                .filter(|(_,character)| character!=&'.')
                .map(move |(x,character)| {
                    ShemeCharacter::new(y,x,character)
                })
        })
    .flatten()
    .partition(|c| c.character.is_numeric());
    
    let mut num_iter = num_arr.into_iter();
    let mut num_arr: Vec<ShemeString> = [].to_vec();
    let mut last_num:isize = -2;

    while let Some(character) = &num_iter.next(){
        if character.pos.x as isize==last_num+1 {
            let mut last = num_arr.pop().unwrap();
            last.string.push(character.character);
            num_arr.push(last);
            last_num+=1;
        } else {
            num_arr.push(ShemeString::from_character(character.clone()));
            last_num=character.pos.x as isize;
        }
    };

    spec_chars.into_iter()
        .flat_map(|spec_char| {
            let nums = num_arr.clone().into_iter()
                .filter(|num| is_adjacent(num.clone(),spec_char.clone()))
                .collect::<Vec<ShemeString>>();

            if nums.len()==2 {
                let num1 = nums[0].string.parse::<usize>().unwrap();
                let num2 = nums[1].string.parse::<usize>().unwrap();
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

