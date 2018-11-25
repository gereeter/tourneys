extern crate rand;
extern crate clap;
#[macro_use] extern crate text_io;

use clap::{Arg, App, SubCommand};

mod parse;
mod render;
mod simulate;

#[derive(Copy, Clone)]
enum GameResult {
    Continue(usize),
    Award(usize, usize)
}

pub struct Bracket {
    starting_games: Vec<usize>,
    game_results: Vec<[GameResult; 2]>
}

fn main() {
    let matches = App::new("Tournament simulator")
                      .version("0.1")
                      .author("Jonathan S <gereeter+code@gmail.com>")
                      .about("View and analyze the fairness and other properties of tournament schedules")
                      .subcommand(SubCommand::with_name("simulate")
                                             .about("Gather statistics about the results of a bracket")
                                             .arg(Arg::with_name("runs")
                                                      .help("How many games to simulate")
                                                      .long("runs")
                                                      .value_name("COUNT")
                                                      .takes_value(true)
                                                      .default_value("2000000"))
                                             .arg(Arg::with_name("seed")
                                                      .help("Whether the inputs to the tournament should be seeded by skill")
                                                      .long("seed"))
                                             .arg(Arg::with_name("graph")
                                                      .help("Display graphs of the various statistics")
                                                      .long("graph"))
                                             .arg(Arg::with_name("luck")
                                                      .help("How much the game is influenced by luck")
                                                      .long("luck")
                                                      .value_name("AMOUNT")
                                                      .takes_value(true)
                                                      .default_value("0.0")))
                      .subcommand(SubCommand::with_name("view")
                                             .about("Print a graphical representation of the bracket"))
                      .arg(Arg::with_name("BRACKET")
                               .help("The name of the bracket to analyze")
                               .required(true))
                      .get_matches();

    let bracket_name = matches.value_of("BRACKET").unwrap();
    let bracket_file = std::fs::File::open(bracket_name).unwrap();
    let bracket = parse::parse(bracket_file);

    if let Some(submatches) = matches.subcommand_matches("simulate") {
        let runs = submatches.value_of("runs").unwrap().parse().expect("Expected an integer number of runs");
        let luck = submatches.value_of("luck").unwrap().parse().expect("Expected a floating point amount of luck");
        simulate::analyze(&bracket, runs, luck, submatches.is_present("seed"), submatches.is_present("graph"));
    } else if let Some(_) = matches.subcommand_matches("view") {
        render::draw_bracket(&bracket);
    } else {
        eprintln!("Unknown command!");
    }
}
