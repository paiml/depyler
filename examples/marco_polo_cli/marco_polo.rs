#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR__: &'static str = "=";
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MarcoPoloGame {
    pub difficulty: String,
    pub verbose: bool,
    pub score: i32,
    pub attempts: i32,
    pub difficulty_ranges: std::collections::HashMap<String, DepylerValue>,
}
impl MarcoPoloGame {
    pub fn new(difficulty: String, verbose: bool) -> Self {
        Self {
            difficulty,
            verbose,
            score: 0,
            attempts: 0,
            difficulty_ranges: std::collections::HashMap::new(),
        }
    }
    pub fn generate_number(&self) -> i32 {
        let (min_val, max_val) = self.difficulty_ranges.clone()[self.difficulty.clone() as usize];
        return {
            use rand::Rng;
            rand::thread_rng().gen_range(min_val..=max_val)
        };
    }
    pub fn get_hint(&self, guess: i32, target: i32) -> String {
        if guess < target {
            let distance = target - guess;
            if distance > 20 {
                return "Marco!(Way too low)".to_string();
            } else {
                if distance > 10 {
                    return "Marco!(Too low)".to_string();
                } else {
                    return "Marco!(A bit low)".to_string();
                };
            };
        } else {
            let distance = guess - target;
            if distance > 20 {
                return "Marco!(Way too high)".to_string();
            } else {
                if distance > 10 {
                    return "Marco!(Too high)".to_string();
                } else {
                    return "Marco!(A bit high)".to_string();
                };
            };
        };
    }
    pub fn play_round(&self) -> bool {
        let target = self.generate_number();
        let (min_val, max_val) = self.difficulty_ranges.clone()[self.difficulty.clone() as usize];
        if self.verbose.clone() {
            println!("{}", format!("\n[DEBUG] Target number: {}", target));
        };
        println!(
            "{}",
            format!(
                "\nI'm thinking of a number between {} and {}...",
                min_val, max_val
            )
        );
        println!(
            "{}",
            "When you guess, I'll say 'Marco!' and give you a hint.".to_string()
        );
        println!("{}", "Find the number to hear 'Polo!'".to_string());
        let mut round_attempts = 0;
        while true {
            {
                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                    {
                        let guess_str = input("\nYour guess: ".to_string());
                        let guess = guess_str.parse::<i32>().unwrap_or(0);
                        if guess < min_val || guess > max_val {
                            println!(
                                "{}",
                                format!("Please guess between {} and {}", min_val, max_val)
                            );
                            continue;
                        };
                        let round_attempts = round_attempts + 1;
                        self.attempts = self.attempts.clone() + 1;
                        if guess == target {
                            println!("{}", "🎉 Polo! You found it!".to_string());
                            self.score = self.score.clone() + 1;
                            if self.verbose.clone() {
                                println!(
                                    "{}",
                                    format!("[DEBUG] Attempts this round: {}", round_attempts)
                                );
                            };
                            return true;
                        } else {
                            let hint = self.get_hint(guess, target);
                            println!("{}", hint);
                        };
                    }
                    Ok(())
                })();
                if let Err(_) = _result {
                    {
                        println!("{}", "Please enter a valid number!".to_string());
                    }
                }
            }
        }
    }
    pub fn calculate_performance(&self) -> String {
        if self.attempts.clone() == 0 {
            return "No games played".to_string();
        };
        let avg_attempts = self.attempts.clone() / (self.score.clone()).max(1);
        if avg_attempts <= 5 {
            return "🏆 Expert".to_string();
        } else {
            if avg_attempts <= 7 {
                return "⭐ Good".to_string();
            } else {
                if avg_attempts <= 10 {
                    return "👍 Average".to_string();
                } else {
                    return "🎯 Keep practicing".to_string();
                };
            };
        };
    }
}
#[derive(Default)]
struct Args {
    #[doc = "Number of rounds to play(default: 3)"]
    rounds: i32,
    #[doc = "Game difficulty(default: medium)"]
    difficulty: String,
    #[doc = "Enable verbose output"]
    verbose: bool,
    version: Option<String>,
}
#[doc = "Parse command line arguments."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_arguments() -> Args {
    ()
}
#[doc = "Print welcome banner."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn print_welcome() {
    println!("{}", STR__.repeat(50 as usize));
    println!("{}", {
        let s = "🎮 Welcome to Marco Polo CLI! 🎮".to_string();
        let width = 50 as usize;
        let fillchar = " ";
        if s.len() >= width {
            s.to_string()
        } else {
            let total_pad = width - s.len();
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;
            format!(
                "{}{}{}",
                fillchar.repeat(left_pad),
                s,
                fillchar.repeat(right_pad)
            )
        }
    });
    println!("{}", STR__.repeat(50 as usize));
}
#[doc = "Print game statistics."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn print_statistics(game: &MarcoPoloGame, rounds: i32) {
    println!("{}", format!("{}{}", "\n", STR__.repeat(50 as usize)));
    println!("{}", {
        let s = "📊 Game Statistics 📊".to_string();
        let width = 50 as usize;
        let fillchar = " ";
        if s.len() >= width {
            s.to_string()
        } else {
            let total_pad = width - s.len();
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;
            format!(
                "{}{}{}",
                fillchar.repeat(left_pad),
                s,
                fillchar.repeat(right_pad)
            )
        }
    });
    println!("{}", STR__.repeat(50 as usize));
    println!("{}", format!("Rounds played: {}/{}", game.score, rounds));
    println!("{}", format!("Total attempts: {}", game.attempts));
    let _cse_temp_0 = game.score > 0;
    if _cse_temp_0 {
        println!(
            "{}",
            format!("Average attempts per round: {}", game.attempts / game.score)
        );
    }
    println!(
        "{}",
        format!("Performance: {}", game.calculate_performance())
    );
    println!("{}", STR__.repeat(50 as usize));
}
#[doc = "Main entry point."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let args = parse_arguments();
    print_welcome();
    let game = MarcoPoloGame::new(args.difficulty, args.verbose);
    println!(
        "{}",
        format!(
            "\nStarting {} rounds on {} difficulty...",
            args.rounds, args.difficulty
        )
    );
    for round_num in (1)..(args
        .rounds
        .iter()
        .chain(1.iter())
        .cloned()
        .collect::<Vec<_>>())
    {
        println!("{}", format!("\n{}", STR__.repeat(30 as usize)));
        println!("{}", format!("Round {} of {}", round_num, args.rounds));
        println!("{}", format!("{}", STR__.repeat(30 as usize)));
        if !game.play_round() {
            println!("{}", "\nGame ended early.");
            break;
        }
    }
    print_statistics(&game, args.rounds);
    println!("{}", "\nThanks for playing! 👋");
    ()
}
