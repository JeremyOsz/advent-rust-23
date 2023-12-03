use std::fs::File;
use std::io::{self, BufRead};

use alloc::collections;

struct Game {
  game_number: i32,
  rounds: Vec<Round>,
}

struct Round {
  blue: i32,
  red: i32,
  green: i32,
}

// Read lines from input
// For each line, parse the line into a game
fn parse_input(filename: &str) -> io::Result<Vec<Game>> {
  let file = File::open(filename)?;
  let reader = io::BufReader::new(file);
  let mut games: Vec<Game> = Vec::new();
  
  for line in reader.lines() {
    let line = line?;
    let game = parse_game(&line);
    games.push(game);
  }
  
  Ok(games)
}



// parse_game parses a line into a game
// Example line: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_game(line: &str) -> Game {

  // Find the game number 
  let game_number = parse_game_number(line).unwrap();

  // Remove the game number from the line
  let line = line.replace(&format!("Game {}: ", game_number), "");

  // Split rounds by semicolon
  let rounds = line.split(";");

  // For each round, parse the round
  let rounds = rounds.map(|round| parse_round(round)).collect::<Vec<Round>>();

  // Return the game
  Game { 
    game_number, rounds 
  }

}

fn parse_round(round_str: &str) -> Round {
  let colors = ["blue", "red", "green"];
  let mut round = Round { blue: 0, red: 0, green: 0 };

  println!("Parsing round: {}", round_str);

  let round_colors = round_str.split(",");
  
  // map through round_colors
  // add value to round

  for round_color in round_colors {
    let round_color = round_color.trim();
    let color = round_color.split(" ").next().unwrap();
    let number = round_color.split(" ").last().unwrap().parse::<i32>().unwrap();

    match color {
      "blue" => round.blue += number,
      "red" => round.red += number,
      "green" => round.green += number,
      _ => panic!("Invalid color"),
    }
  }
  round
}

fn parse_game_number(line: &str) -> Option<i32> {
  let re = regex::Regex::new(r"Game (\d+):").unwrap();
  let captures = re.captures(line)?;
  let game_number = captures.get(1)?.as_str();
  
  game_number.parse::<i32>().ok()
}

fn game_is_possible(game: &Game, max_blocks: &Round) -> bool {
  // Print "Checking game 1"
  println!("Checking game {}", game.game_number);

  // For each round, check if any of the colors are greater than the max blocks
  for round in &game.rounds {
    // check if blue is greater than max blocks
    // Print "Blue blocks: {n} - Max blocks: {m} - Game 1 is not possible"
    println!("Blue blocks: {} - Max blocks: {}", round.blue, max_blocks.blue);
    if round.blue > max_blocks.blue {
      // Print "Blue blocks: {n} - Max blocks: {m} - Game 1 is not possible"
      print!("NOT POSSIBLE");
      return false;
    }

    // check if red is greater than max blocks
    // Print "Red blocks: {n} - Max blocks: {m} - Game 1 is not possible"
    println!("Red blocks: {} - Max blocks: {}", round.red, max_blocks.red);
    if round.red > max_blocks.red {
      // Print "Red blocks: {n} - Max blocks: {m} - Game 1 is not possible"
      print!("NOT POSSIBLE");
      return false;
    }

    // check if green is greater than max blocks
    // Print "Green blocks: {n} - Max blocks: {m} - Game 1 is not possible"
    println!("Green blocks: {} - Max blocks: {}", round.green, max_blocks.green);
    if round.green > max_blocks.green {
      // Print "Green blocks: {n} - Max blocks: {m} - Game 1 is not possible"
      print!("NOT POSSIBLE");

      return false;
    }
  }
  return true;
}

pub fn main() {

  let input = parse_input("./input.txt").unwrap();

  let max_blocks = Round {
    red: 12,
    green: 13,
    blue: 14,
  };

  // 1 should be possible
  let possible = game_is_possible(&input[0], &max_blocks);


  // Print if the game is possible
  println!("Game 1 is possible: {}", possible);
}




#[cfg(test)]
mod tests {
  use super::*;
  
  // #[test]
  // fn test_process() {
  //   main();
  // }
  
  #[test]
  fn test_part1() {

    let max_blocks = Round {
      red: 12,
      green: 13,
      blue: 14,
    };
    
    let games  = parse_input("./src/test.txt").unwrap();
    // 1 should be possible
    assert!(game_is_possible(&games[0], &max_blocks));
    
    // 2 should be possible
    assert!(game_is_possible(&games[1], &max_blocks));

    // 5 should be possible
    assert!(game_is_possible(&games[4], &max_blocks));

    // 3 should not be possible
    assert!(!game_is_possible(&games[2], &max_blocks));

    // 4 should not be possible
    assert!(!game_is_possible(&games[3], &max_blocks));

  }
}

