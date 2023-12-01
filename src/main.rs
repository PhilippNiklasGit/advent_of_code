use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    let file = File::open("input").expect("no such file");
    let buf = BufReader::new(file);
    let p = buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect::<Vec<String>>();

    println!("{}",to_value(p.clone()));
    println!("{}",to_value_new(p));
}

fn to_value(lines: Vec<String>) -> usize {
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

fn to_value_new(lines:Vec<String>) -> usize {
    let mut sum = 0;
    for line in lines {
        let line_chars = line.chars().collect::<Vec<char>>();
        let mut num_list:Vec<char> = [].to_vec();  

        for i in 0..line.len() {
            if line.len()-i>2{
                match &line[i..i+3] {
                    "one" => num_list.push('1'),
                    "two" => num_list.push('2'),
                    "six" => num_list.push('6'),
                    _ => {}
                }
                if line.len()-i>3 {
                    match &line[i..i+4] {
                        "four" => num_list.push('4'),
                        "five" => num_list.push('5'),
                        "nine" => num_list.push('9'),
                        _ => {}
                    }
                    if line.len()-i>4{
                        match &line[i..i+5] {
                            "seven" => num_list.push('7'),
                            "eight" => num_list.push('8'),
                            "three" => num_list.push('3'),
                            _ => {}
                        }
                    }
                }
            }
            if line_chars[i].is_numeric() {
                num_list.push(line_chars[i]);
            }

        }
        let first_num = num_list.first().unwrap_or(&'0');
        let last_num = num_list.last().unwrap_or(&'0');
        let line_chars = [first_num,last_num].into_iter().collect::<String>();
        let line_chars = line_chars.parse::<usize>().ok().unwrap();
        sum += line_chars;
    }
    sum
}
