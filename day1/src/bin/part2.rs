fn main() {
    let p = include_str!("./input");
    let p = p.split('\n').collect::<Vec<&str>>();
    println!("{}",to_value(p));
}


fn to_value(lines:Vec<&str>) -> usize {
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
