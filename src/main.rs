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

fn to_value(v: Vec<String>) -> usize {
    let mut sum = 0;
    for nv in v {
        let n = nv.chars()
                .collect::<Vec<char>>();
        let n = n.into_iter()
                .filter(|x| x.is_numeric())
                .collect::<Vec<char>>();
        //illegal dark magic
        let e = [n.first().unwrap_or(&'0'),n.last().unwrap_or(&'0')].into_iter().collect::<String>();
        let e = e.parse::<usize>().ok().unwrap();
        sum += e;
    }
    sum
}

fn to_value_new(v:Vec<String>) -> usize {
    let mut sum = 0;
    for s in v {
        let ch = s.chars().collect::<Vec<char>>();
        let mut nums:Vec<char> = [].to_vec();  

        for i in 0..s.len() {
            if s.len()-i>2{
                match &s[i..i+3] {
                    "one" => nums.push('1'),
                    "two" => nums.push('2'),
                    "six" => nums.push('6'),
                    _ => {}
                }
                if s.len()-i>3 {
                    match &s[i..i+4] {
                        "four" => nums.push('4'),
                        "five" => nums.push('5'),
                        "nine" => nums.push('9'),
                        _ => {}
                    }
                    if s.len()-i>4{
                        match &s[i..i+5] {
                            "seven" => nums.push('7'),
                            "eight" => nums.push('8'),
                            "three" => nums.push('3'),
                            _ => {}
                        }
                    }
                }
            }
            if ch[i].is_numeric() {
                nums.push(ch[i]);
            }

        }
        // get first and last to string ('1' + '5' -> "15")
        let ch = [nums.first().unwrap_or(&'0'), nums.last().unwrap_or(&'0')].into_iter().collect::<String>();
        // parse string to num
        let ch = ch.parse::<usize>().ok().unwrap();
        sum += ch;
    }
    sum
}

//NOTE: fucked something up, doesnt work
#[allow(unused)]
fn to_value_alt(v: Vec<String>) -> u32 {
    let mut sum = 0;
    v[0].find("");
    for nv in v {
        let n = nv.chars().collect::<Vec<char>>();
        let mut e = 0;
        for i in 0..nv.len() {
            if n[i].is_numeric() {
                e+= n[i].to_digit(10).unwrap_or(0) * 10;
            }
        }
        for i in (0..nv.len()).rev() {
            if n[i].is_numeric() {
                e+= n[i].to_digit(10).unwrap_or(0);
            }
        }
        
        sum += e;
    }
    sum
}

