use std::cmp;

fn main() {
    let game_input = include_str!("input");
    let base_game = Game::new(0, 12, 13, 14);
    println!("{}", part1(base_game, game_input))
}

fn part1(base_game: Game, game_input: &str) -> usize {
    let game_list = game_input.split('\n').collect::<Vec<&str>>();
    let mut sum = 0; 
    for game_str in game_list {
        if &game_str == &"" {continue;}
        let mut game = Game::from_id(&game_str);
        let reach_list = game_str.split(':').collect::<Vec<&str>>();
        let reach_list = reach_list.last().unwrap();
        for reach in reach_list.split(';') {
            game.register_reach(&reach);
        }
        if base_game.combination_is_possible(&game) {
            sum+= game.id;
        }

    }
    sum
}
#[derive(Debug)]
#[allow(unused)]
struct Game {
    pub id: usize,
    pub red: usize,
    pub blue: usize,
    pub green: usize
}
#[allow(unused)]
impl Game {
    pub fn new(id: usize, red: usize, green:usize, blue:usize) -> Self {
        Self {
            id,
            red,
            blue,
            green
        }
    }
    pub fn from_id(game_str: &str) -> Self {
        let game_fields = game_str.split(' ').collect::<Vec<&str>>();
        let id = &game_fields[1]; // get id field
        let id = &id[..id.len()-1]; // remove :
        let id = id.parse::<usize>().unwrap();
        Self {
            id,
            red:0,
            blue:0,
            green:0
        }
    }

    pub fn register_reach(&mut self, reach_str: &str) {
        for colour in reach_str.split(',') {
            let colour_fields = colour.split(' ').collect::<Vec<&str>>();
            self.register_colour(colour_fields[2], colour_fields[1]);
        }
    }
    fn register_colour(&mut self, colour: &str, val: &str) {
        let val = val.parse::<usize>().unwrap_or(0);
        match &colour[0..1] {
            "g" => self.green = cmp::max(val, self.green),
            "r" => self.red = cmp::max(val, self.red),
            "b" => self.blue = cmp::max(val, self.blue),
            _ => {}
        } 
    }
    pub fn combination_is_possible(&self,game: &Game) -> bool {
        self.red>=game.red && self.green>=game.green&& self.blue>=game.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            Game::new(0, 12, 13, 14),
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            );
        assert_eq!(8,result);
    }
}
