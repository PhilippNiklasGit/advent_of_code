fn main() {
    let p = include_str!("./input");
    let p = p.split('\n').collect::<Vec<&str>>();
    println!("{}",part1(p));
}

fn part1(lines: Vec<&str>) -> usize {
    let mut sum = 0;
    for line in lines {
        let line_chars = line.chars()
                .collect::<Vec<char>>();

        let num_list = line_chars.into_iter()
                .filter(|x| x.is_numeric())
                .collect::<Vec<char>>();

        let first_num = num_list.first().unwrap_or(&'0');
        let last_num = num_list.last().unwrap_or(&'0');
        let line_sum = [first_num,last_num].into_iter().collect::<String>();
        let line_sum = line_sum.parse::<usize>().ok().unwrap();
        sum += line_sum;
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet".lines().collect()
            );
        assert_eq!(142, result);
    }
}
