fn main() {
    let input = include_str!("input");
    println!("{}", part1(input));
}
#[derive(Debug, Clone)]
#[allow(unused)]
struct Card {
    id: usize,
    your_nums: Vec<usize>,
    win_nums: Vec<usize>
}
impl Card {
    pub fn from_str(input: &str) -> Self {
        let input = input.split(':').collect::<Vec<&str>>();
        let id = &input[0][5..].trim();
        let id = id.parse::<usize>().unwrap();
        let cards = input[1].split('|').collect::<Vec<&str>>();
        let your_nums = cards[0].split(' ')
            .map(|c| c.parse::<usize>().unwrap_or(0))
            .filter(|c| c!=&0)
            .collect::<Vec<usize>>();
        let win_nums = cards[1].split(' ')
            .map(|c| c.parse::<usize>().unwrap_or(0))
            .filter(|c| c!=&0)
            .collect::<Vec<usize>>();
        Self {
            id,
            your_nums,
            win_nums
        }

    }
}
fn calc_recursive(cards: &[Card]) -> usize {
    let win_amount = cards[0].your_nums.clone().into_iter()
        .filter(|your_num| cards[0].win_nums.contains(&your_num))
        .count();
    if win_amount == 0 { return 1; } 
    cards[..win_amount].into_iter().enumerate().map(|(index,_)| {
        calc_recursive(&cards[index+1..])
    })
    .sum::<usize>() + 1
}

fn part1(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| Card::from_str(line))
        .collect::<Vec<Card>>();

    cards.clone().into_iter()
        .enumerate()
        .map(|(index, _)| {
            calc_recursive(&cards[index..])
    }).sum::<usize>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            );
        assert_eq!(30, result);
    }
}
