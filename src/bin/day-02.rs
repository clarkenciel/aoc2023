use std::collections::HashMap;
use std::error;
use std::fmt::{Debug, Display};
use std::io;

fn main() -> io::Result<()> {
    let mut draw_one = Draw::new();
    draw_one.add("red", 12);
    draw_one.add("green", 13);
    draw_one.add("blue", 14);

    println!(
        "solution: {}",
        solve(INPUT_ONE, &draw_one).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    );

    Ok(())
}

fn solve(input: &str, draw: &Draw<'_>) -> Result<usize, SolutionError> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .try_fold(0, |acc, game_str| {
            let game: Game<'_> = parse_game(game_str).map_err(SolutionError::InvalidGame)?;
            Ok(if game.could_support(draw) {
                dbg!(acc) + dbg!(game.id)
            } else {
                acc
            })
        })
}

#[derive(Debug)]
enum SolutionError {
    InvalidGame(GameParseError),
}

impl Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "solution failed: {}",
            match self {
                Self::InvalidGame(e) => format!("invalid game: {}", e),
            }
        )
    }
}

impl error::Error for SolutionError {}

#[derive(Debug)]
struct Game<'a> {
    id: usize,
    draws: Vec<Draw<'a>>,
}

impl<'a> Game<'a> {
    fn could_support<'b: 'a>(&self, draw: &Draw<'b>) -> bool {
        self.draws.iter().all(|d| dbg!(d.could_support(draw)))
    }
}

fn parse_game<'a, 'b: 'a>(s: &'b str) -> Result<Game<'a>, GameParseError> {
    let mut parts = s.split(": ");
    if let Some((game_header, draws)) = parts.next().zip(parts.next()) {
        let mut parts = game_header.split(" ");
        if let Some((_, id)) = parts.next().zip(parts.next()) {
            let draws = draws
                .split("; ")
                .map(parse_draw)
                .collect::<Result<_, _>>()
                .map_err(GameParseError::InvalidDraws)?;

            let id = str::parse(id).map_err(|_| GameParseError::InvalidId)?;

            Ok(Game { id, draws })
        } else {
            Err(GameParseError::InvalidGameHeader)
        }
    } else {
        Err(GameParseError::InvalidGameString)
    }
}

#[derive(Debug)]
enum GameParseError {
    InvalidGameString,
    InvalidGameHeader,
    InvalidId,
    InvalidDraws(DrawParseError),
}

impl error::Error for GameParseError {}

impl Display for GameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse game: {}",
            match self {
                Self::InvalidGameString => "malformed game string".to_owned(),
                Self::InvalidGameHeader => "malformed game header".to_owned(),
                Self::InvalidId => "invalid game ID string".to_owned(),
                Self::InvalidDraws(e) => format!("invalid draws: {}", e),
            }
        )
    }
}

/// Mapping of colors to counts
#[derive(Debug)]
struct Draw<'a>(HashMap<&'a str, usize>);

impl<'a> Draw<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add<'b: 'a>(&mut self, color: &'b str, count: usize) {
        self.0
            .entry(color)
            .and_modify(|old_count| *old_count = count)
            .or_insert(count);
    }

    fn could_support<'b: 'a>(&self, other: &Self) -> bool {
        for key in self.0.keys().chain(other.0.keys()) {
            match (self.0.get(key.into()), other.0.get(key.into())) {
                (Some(ours), Some(theirs)) => {
                    if ours > theirs {
                        return false;
                    }
                }
                (Some(_), None) => return false, // if they don't have it we shouldn't have been able to
                (None, Some(_)) => (), // if we don't have it it's possible that we simply didn't see it
                (None, None) => return false, // dunno what's even happening here
            }
        }

        true
    }
}

fn parse_draw<'a, 'b: 'a>(s: &'b str) -> Result<Draw<'a>, DrawParseError> {
    s.split(r", ").try_fold(Draw::new(), |mut draw, draw_str| {
        let parts: Vec<&str> = draw_str.split(" ").collect();

        if let Some((count_str, color)) = parts.get(0).zip(parts.get(1)) {
            let count = str::parse(count_str).map_err(|_| DrawParseError::InvalidCount)?;
            draw.add(color, count);
            Ok(draw)
        } else {
            Err(DrawParseError::InvalidColorCountPair)
        }
    })
}

#[derive(Debug)]
enum DrawParseError {
    InvalidColorCountPair,
    InvalidCount,
}

impl error::Error for DrawParseError {}

impl Display for DrawParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Draw parsing failed: {}",
            match self {
                Self::InvalidColorCountPair => "invalid color count pair",
                Self::InvalidCount => "invalid count",
            }
        )
    }
}

#[test]
fn test_example_one() {
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    let mut draw_one = Draw::new();
    draw_one.add("red", 12);
    draw_one.add("green", 13);
    draw_one.add("blue", 14);

    assert_eq!(8, solve(input, &draw_one).expect("SOLUTION FAILED"));
}

const INPUT_ONE: &'static str = r#"
Game 1: 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue
Game 2: 2 green, 3 blue; 11 red; 2 green, 5 red, 1 blue
Game 3: 19 green, 4 blue, 13 red; 1 green, 1 blue, 1 red; 17 red, 18 green
Game 4: 4 green, 8 blue, 20 red; 19 red, 3 green, 14 blue; 15 red, 4 green, 1 blue; 18 blue, 14 red; 19 red, 10 blue; 3 green, 11 blue, 15 red
Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
Game 6: 2 green, 10 red; 4 blue, 1 red, 2 green; 2 red, 2 blue, 1 green; 5 red, 3 green, 1 blue
Game 7: 4 green, 2 blue, 10 red; 1 green, 12 red; 5 green, 12 red, 2 blue; 10 red, 1 blue, 5 green; 1 green, 1 blue, 11 red
Game 8: 8 blue, 3 red, 1 green; 9 blue, 14 green, 6 red; 3 red, 15 blue, 16 green; 9 red, 4 green, 6 blue
Game 9: 9 blue, 9 red, 5 green; 6 red, 1 green, 12 blue; 7 blue, 3 green; 4 red, 12 blue, 1 green; 5 red, 4 green, 1 blue
Game 10: 1 blue, 2 red, 19 green; 7 green, 5 blue, 7 red; 2 blue, 1 red, 3 green; 2 blue, 9 red, 10 green
Game 11: 2 red, 17 blue, 12 green; 5 green, 3 blue; 14 green, 2 red, 15 blue
Game 12: 4 blue, 13 green, 1 red; 5 blue, 3 green, 4 red; 8 blue, 15 green; 12 blue, 5 red, 6 green; 2 green, 5 blue, 4 red; 11 blue, 18 green, 4 red
Game 13: 8 blue, 11 red, 2 green; 18 red, 7 blue, 7 green; 6 green, 9 red; 7 green, 3 blue, 12 red; 1 green, 4 red, 4 blue
Game 14: 3 green, 11 blue, 1 red; 3 green, 1 red, 13 blue; 5 green, 6 blue, 1 red; 1 red, 5 blue, 5 green; 10 blue, 2 green
Game 15: 3 red, 8 green, 1 blue; 8 green, 10 red, 3 blue; 1 blue, 4 green, 2 red; 10 red, 10 green; 3 blue, 4 green, 3 red; 12 green, 7 red
Game 16: 13 red, 9 blue; 2 green, 7 red, 7 blue; 9 blue, 7 red, 7 green; 13 blue, 10 red
Game 17: 12 red, 19 green, 4 blue; 2 blue, 5 red, 11 green; 4 red, 7 green, 8 blue; 6 red, 10 green; 3 green, 7 red, 10 blue
Game 18: 2 blue, 6 red; 5 red, 3 green; 12 red, 1 blue, 3 green; 1 green, 19 red, 5 blue; 3 green, 2 blue, 16 red
Game 19: 10 red, 5 green; 10 red; 9 red, 7 blue; 1 blue, 8 red
Game 20: 11 green, 5 red, 7 blue; 7 green, 12 red, 11 blue; 13 green, 3 blue, 5 red; 3 red, 3 blue, 1 green
Game 21: 10 blue, 10 green, 2 red; 16 blue, 9 green, 1 red; 3 green, 1 blue, 2 red; 17 green, 5 blue, 2 red; 6 blue, 15 green, 2 red
Game 22: 2 red, 1 blue, 5 green; 4 blue, 3 red, 6 green; 3 red, 4 blue; 4 blue, 1 green, 1 red; 3 blue, 3 red, 5 green
Game 23: 3 red, 7 green; 17 green, 7 red, 5 blue; 4 blue, 4 red; 19 green, 3 red, 9 blue; 3 green, 3 blue, 6 red; 9 red, 7 green, 6 blue
Game 24: 2 red, 14 green, 8 blue; 6 red, 12 blue, 15 green; 1 green, 10 red; 3 red, 7 blue, 15 green; 11 red, 13 blue; 1 green, 9 blue, 10 red
Game 25: 12 blue, 7 red, 7 green; 2 red, 10 green, 9 blue; 11 blue, 6 green, 6 red; 6 blue, 6 green; 4 red, 8 blue, 2 green; 6 green, 4 blue, 3 red
Game 26: 2 blue, 5 red; 9 red, 1 green; 16 red, 2 blue; 4 blue, 1 green, 5 red
Game 27: 12 green, 12 red; 15 green, 11 red; 7 red, 19 green; 1 blue, 2 green, 3 red
Game 28: 4 blue, 4 green, 7 red; 6 green, 9 red, 10 blue; 5 red, 4 blue, 9 green; 9 red, 6 blue, 2 green
Game 29: 1 blue, 3 green, 9 red; 5 green; 3 green, 2 red
Game 30: 1 blue, 5 green, 3 red; 2 green, 1 blue, 3 red; 12 green, 4 red; 5 green, 2 red; 8 green, 4 red, 1 blue; 6 green
Game 31: 9 blue, 5 green; 3 red, 11 blue, 2 green; 1 green, 4 blue, 2 red; 1 green, 3 blue, 1 red; 11 blue, 5 green
Game 32: 3 red, 1 blue, 16 green; 11 green, 4 blue; 2 blue, 4 green, 2 red
Game 33: 4 blue, 15 green; 6 green, 11 blue; 5 blue, 1 red, 13 green; 14 green, 1 red, 5 blue; 1 red, 4 blue, 14 green
Game 34: 2 green, 10 red, 2 blue; 8 red, 10 green; 3 green, 1 blue, 1 red
Game 35: 3 blue, 7 green, 10 red; 4 red, 9 blue; 8 blue, 7 green, 4 red; 2 green, 7 red, 3 blue
Game 36: 2 green, 4 red, 4 blue; 1 blue, 6 red; 7 green, 10 red; 10 red, 3 blue
Game 37: 1 green, 1 red; 1 blue, 1 red, 9 green; 6 blue, 11 green, 10 red; 17 blue, 3 green; 2 green, 8 red, 6 blue
Game 38: 6 blue, 7 green, 2 red; 12 green, 15 blue, 3 red; 7 blue, 3 red, 7 green; 6 blue, 10 green
Game 39: 8 green, 4 red, 14 blue; 19 blue, 11 red, 5 green; 15 green, 12 blue, 13 red; 18 green, 5 red, 11 blue; 10 green, 8 blue, 10 red; 8 green, 2 red
Game 40: 2 green, 5 red, 4 blue; 1 red, 2 green; 10 blue, 1 green; 8 blue, 8 red, 2 green; 2 red, 6 blue
Game 41: 18 red, 2 blue; 17 red, 4 green, 3 blue; 5 blue, 7 red; 3 blue, 3 green, 8 red; 8 red, 2 blue
Game 42: 2 blue, 6 green, 7 red; 2 red, 4 blue; 4 green, 4 blue, 10 red; 6 green, 5 red, 8 blue; 3 red, 3 blue
Game 43: 1 green, 17 red, 1 blue; 16 red, 8 green; 7 blue, 15 red, 10 green; 1 green, 1 blue, 6 red; 13 green, 13 red; 11 green, 8 blue, 1 red
Game 44: 9 blue, 9 green, 19 red; 7 red, 1 blue; 6 blue, 8 green, 9 red; 8 green, 2 blue, 13 red; 1 blue, 5 green, 19 red
Game 45: 5 red, 3 green, 10 blue; 4 green, 17 red, 3 blue; 13 blue, 2 red, 10 green; 11 blue, 15 red, 13 green
Game 46: 14 green, 1 blue, 6 red; 12 green, 18 red; 10 red, 1 blue, 3 green; 5 red, 8 green
Game 47: 8 red, 5 blue, 2 green; 4 red, 4 blue; 3 blue, 9 red, 2 green; 2 red, 2 green, 4 blue; 14 red, 1 green, 2 blue
Game 48: 11 red, 2 blue, 1 green; 2 green, 11 blue, 7 red; 2 red, 1 green, 12 blue; 1 green, 7 red, 2 blue
Game 49: 7 blue, 8 red, 2 green; 10 red, 5 blue, 2 green; 10 red, 2 blue, 3 green
Game 50: 18 red, 3 green; 8 red, 10 blue, 3 green; 11 red, 1 green; 8 red, 9 blue, 1 green; 7 blue, 3 red, 3 green
Game 51: 6 green, 2 blue, 3 red; 1 green, 4 red; 1 red, 1 blue; 1 red, 5 green; 6 green, 2 red
Game 52: 8 green, 6 blue; 2 blue, 1 red, 17 green; 8 red, 8 green; 2 green, 4 red, 2 blue
Game 53: 6 red, 4 blue; 15 red, 4 green; 16 red, 3 green, 7 blue; 1 green, 18 red, 2 blue; 14 red, 8 blue, 1 green
Game 54: 9 red, 11 green, 4 blue; 9 blue, 9 green, 14 red; 3 blue, 1 red, 9 green; 7 green, 16 red, 10 blue; 11 green, 3 blue, 11 red; 3 blue
Game 55: 9 red; 1 blue, 10 red, 1 green; 15 red, 5 green; 2 blue, 1 green, 14 red; 1 blue, 2 green, 8 red; 6 red, 3 green, 2 blue
Game 56: 16 blue, 13 green, 1 red; 7 green, 4 blue; 1 red, 11 blue, 16 green; 10 green, 2 red, 9 blue; 20 green, 1 blue, 1 red; 14 green, 2 blue
Game 57: 13 red, 7 blue, 4 green; 19 red, 3 blue, 8 green; 9 red, 2 blue, 13 green
Game 58: 5 green, 10 blue; 11 green, 9 blue; 6 green, 11 blue; 8 green, 2 blue; 1 red, 5 blue, 2 green; 6 green, 5 blue
Game 59: 7 blue, 4 red; 1 green, 15 red, 7 blue; 6 blue, 15 red; 2 green, 13 red, 7 blue; 6 blue, 15 red
Game 60: 6 green, 2 blue, 1 red; 6 green, 8 blue; 11 green, 2 red, 15 blue; 1 red, 4 blue, 9 green
Game 61: 2 green, 4 blue; 8 red, 4 blue, 3 green; 4 green, 8 blue, 5 red
Game 62: 1 blue, 11 green; 7 green, 3 blue; 7 green, 1 blue, 1 red
Game 63: 14 red, 7 green, 1 blue; 2 red, 15 green; 3 green, 6 red; 20 red, 1 blue, 9 green; 11 red, 1 blue, 15 green
Game 64: 9 red, 2 green; 9 green, 8 red, 2 blue; 2 red, 2 blue; 1 green; 7 red, 1 blue, 9 green; 12 green, 4 red
Game 65: 4 blue, 1 red, 3 green; 7 green, 8 blue; 11 red, 3 green, 1 blue
Game 66: 5 red, 2 blue; 2 green, 1 red; 2 green, 13 red, 4 blue; 15 red, 2 blue
Game 67: 2 green, 1 blue, 15 red; 8 blue, 10 red, 1 green; 2 green, 6 blue, 18 red
Game 68: 6 red, 2 green, 3 blue; 1 blue, 13 red, 5 green; 5 green, 2 blue, 7 red; 2 blue, 8 green, 2 red; 2 red, 1 blue; 8 green, 8 red
Game 69: 2 blue, 3 red; 3 green, 1 red, 2 blue; 2 red, 1 green, 5 blue; 3 red, 3 green, 4 blue; 1 blue, 4 green, 7 red; 2 green, 4 blue
Game 70: 2 red, 17 green; 8 red, 14 green; 1 blue, 18 green; 5 red, 4 green, 1 blue
Game 71: 6 red; 3 green, 9 blue, 18 red; 19 blue, 14 red; 11 blue, 18 red, 5 green
Game 72: 13 blue, 10 red, 4 green; 5 green, 13 blue, 13 red; 7 green, 7 red, 5 blue; 9 blue, 6 red; 4 blue, 6 green; 7 red, 2 blue, 4 green
Game 73: 3 red, 6 blue; 8 red, 2 blue; 6 blue, 1 green, 4 red; 1 green, 5 red
Game 74: 3 red, 3 green, 10 blue; 1 green, 1 red; 7 red, 3 green, 7 blue; 3 blue, 4 red
Game 75: 1 green, 10 red, 8 blue; 13 red, 7 green, 9 blue; 18 red, 9 blue; 2 green, 1 blue, 5 red
Game 76: 4 green, 4 red; 8 green, 3 red, 3 blue; 1 red, 2 green; 6 blue, 3 red, 3 green; 1 red, 1 green, 1 blue
Game 77: 7 red, 8 blue, 7 green; 13 green, 7 blue, 8 red; 2 red, 10 green, 5 blue; 2 red, 5 blue, 3 green
Game 78: 2 red, 7 blue; 5 blue, 6 green, 3 red; 16 green, 3 blue, 10 red; 13 green, 2 blue, 2 red
Game 79: 1 blue, 8 red; 4 blue, 12 red, 5 green; 14 red, 10 blue; 2 green, 7 red, 2 blue; 14 red, 2 green
Game 80: 7 blue, 5 green, 7 red; 2 green, 2 blue, 7 red; 4 red, 1 blue, 18 green; 2 green, 11 red, 3 blue; 4 blue, 9 red, 15 green
Game 81: 3 green, 4 red, 14 blue; 11 blue, 4 green, 14 red; 6 blue, 8 red, 5 green
Game 82: 2 green, 7 blue, 2 red; 15 blue, 2 green, 1 red; 3 blue, 2 green; 1 red; 2 red, 15 blue, 2 green
Game 83: 5 blue, 1 green, 5 red; 12 red, 10 blue; 1 blue, 11 red
Game 84: 4 red, 8 green, 14 blue; 1 green, 8 blue, 2 red; 1 red, 6 blue, 9 green; 8 green, 15 blue, 4 red; 4 blue, 4 red, 6 green
Game 85: 8 green, 16 red, 5 blue; 10 red, 10 green; 18 green, 10 blue, 1 red; 3 red, 9 blue, 13 green
Game 86: 2 blue, 10 green, 6 red; 8 blue, 6 green; 8 blue, 3 red; 8 green, 4 red, 3 blue
Game 87: 8 blue, 4 red; 16 red, 20 blue, 4 green; 18 red, 1 green, 1 blue; 6 red, 1 green, 16 blue; 18 blue, 6 red
Game 88: 13 green, 8 blue, 5 red; 13 red, 5 green, 9 blue; 10 red, 18 blue, 7 green; 14 green, 9 red, 13 blue
Game 89: 14 red, 3 green, 5 blue; 10 blue, 5 red; 9 blue, 12 red, 3 green
Game 90: 2 red, 2 blue; 1 blue, 1 green, 3 red; 2 green, 1 blue, 8 red; 4 red, 2 green, 2 blue; 2 blue, 9 red, 6 green
Game 91: 1 green, 1 blue, 6 red; 3 blue, 2 red; 2 red, 10 green; 7 green, 2 blue, 4 red; 1 blue, 12 green, 8 red
Game 92: 5 red, 6 blue, 14 green; 9 blue, 1 red, 10 green; 2 red, 7 green, 6 blue; 2 red, 10 blue, 4 green
Game 93: 11 red, 5 blue, 2 green; 7 green, 8 red, 6 blue; 3 green, 5 blue, 10 red; 16 red, 8 blue, 6 green; 2 green; 1 green, 11 blue, 16 red
Game 94: 4 blue, 2 red, 4 green; 5 blue, 1 red, 2 green; 12 blue, 3 red, 3 green
Game 95: 1 red, 12 green, 4 blue; 1 blue, 9 green, 3 red; 1 blue, 13 green, 1 red; 3 red, 2 green, 1 blue; 4 blue, 3 red, 15 green
Game 96: 1 red, 7 blue, 2 green; 5 green; 3 red, 5 green, 11 blue
Game 97: 8 green, 6 red; 1 blue, 6 red, 10 green; 1 blue, 6 red
Game 98: 2 green, 8 red, 1 blue; 9 green, 2 blue, 7 red; 1 blue, 2 red, 11 green; 8 red, 10 green, 2 blue
Game 99: 3 blue, 2 red; 1 blue, 3 green, 3 red; 1 red, 3 green; 2 green, 2 red, 2 blue
Game 100: 7 blue, 6 red, 5 green; 3 blue, 13 green, 11 red; 6 red, 13 green, 14 blue; 8 red, 10 blue, 15 green
"#;
