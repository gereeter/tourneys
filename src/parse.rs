use std::io::Read;
use Bracket;
use GameResult::*;

pub fn parse<R: Read>(reader: R) -> Bracket {
    let mut bracket_iter = reader.bytes().map(|b| b.unwrap());

    let num_players: usize;
    let num_matches: usize;
    scan!(bracket_iter => "{} {}\n", num_players, num_matches);

    let mut bracket = Bracket {
        starting_games: Vec::with_capacity(num_players),
        game_results: Vec::with_capacity(num_matches)
    };

    for _ in 0..num_players {
        bracket.starting_games.push(read!("{}", bracket_iter));
    }
    for _ in 0..num_matches {
        let win_result: String = read!("{}", bracket_iter);
        let lose_result: String = read!("{}", bracket_iter);
        let win_parsed = if win_result.starts_with('c') {
            Continue(read!("c{}", win_result.as_bytes().iter().cloned()))
        } else if win_result.starts_with('a') {
            let award_min;
            let award_max;
            scan!(win_result.as_bytes().iter().cloned() => "a{}-{}", award_min, award_max);
            Award(award_min, award_max)
        } else {
            panic!("Unknown game result {}", win_result);
        };
        let lose_parsed = if lose_result.starts_with('c') {
            Continue(read!("c{}", lose_result.as_bytes().iter().cloned()))
        } else if lose_result.starts_with('a') {
            let award_min;
            let award_max;
            scan!(lose_result.as_bytes().iter().cloned() => "a{}-{}", award_min, award_max);
            Award(award_min, award_max)
        } else {
            panic!("Unknown game result {}", lose_result);
        };

        bracket.game_results.push([win_parsed, lose_parsed]);
    }

    bracket
}
