use std;

use Bracket;
use GameResult::*;

#[derive(Clone)]
struct RenderedBlock {
    lines: Vec<String>,
    out_line: usize,
    width: usize,
    game_latency: usize
}

pub fn draw_bracket(bracket: &Bracket) {
    let mut final_drawings = vec![];
    let mut match_drawings = vec![vec![]; bracket.game_results.len()];
    for (i, &match_id) in bracket.starting_games.iter().enumerate() {
        match_drawings[match_id].push(RenderedBlock {
            lines: vec![format!("{:3} ────", i + 1)],
            out_line: 0,
            width: 8,
            game_latency: 0
        });
    }

    let mut next_match_name = String::from("A");

    for match_id in 0..bracket.game_results.len() {
        let mut input_drawings = std::mem::replace(&mut match_drawings[match_id], vec![]);
        assert!(input_drawings.len() == 2);
        let mut drawing = RenderedBlock {
            lines: Vec::with_capacity(input_drawings[0].lines.len() + 1 + input_drawings[1].lines.len()),
            out_line: input_drawings[0].lines.len(),
            width: std::cmp::max(input_drawings[0].width, input_drawings[1].width) + 8,
            game_latency: std::cmp::max(input_drawings[0].game_latency, input_drawings[1].game_latency) + 1
        };

        let out_lines = [input_drawings[0].out_line, input_drawings[1].out_line];
        let pads = [drawing.width - 8 - input_drawings[0].width, drawing.width - 8 - input_drawings[1].width];

        for line in &mut input_drawings[0].lines[0..out_lines[0]] {
            line.extend(std::iter::repeat(' ').take(pads[0]));
            line.push_str("        ");
        }
        input_drawings[0].lines[out_lines[0]].extend(std::iter::repeat('─').take(pads[0]));
        input_drawings[0].lines[out_lines[0]].push_str("┐       ");
        for line in &mut input_drawings[0].lines[(out_lines[0]+1)..] {
            line.extend(std::iter::repeat(' ').take(pads[0]));
            line.push_str("│       ");
        }

        for line in &mut input_drawings[1].lines[0..out_lines[1]] {
            line.extend(std::iter::repeat(' ').take(pads[1]));
            line.push_str("│       ");
        }
        input_drawings[1].lines[out_lines[1]].extend(std::iter::repeat('─').take(pads[1]));
        input_drawings[1].lines[out_lines[1]].push_str("┘       ");
        for line in &mut input_drawings[1].lines[(out_lines[1]+1)..] {
            line.extend(std::iter::repeat(' ').take(pads[1]));
            line.push_str("        ");
        }

        let mut middle_line = match bracket.game_results[match_id][1] {
            Continue(loser_game) => {
                let match_name = next_match_name.clone();
                let mut extra_as = 0;
                loop {
                    match next_match_name.pop() {
                        Some(c) => {
                            if c == 'Z' {
                                extra_as += 1;
                                continue;
                            } else {
                                next_match_name.push(((c as u8) + 1) as char);
                                break;
                            }
                        },
                        None => {
                            extra_as += 1;
                            break;
                        }
                    }
                }
                next_match_name.extend(std::iter::repeat('A').take(extra_as));

                let mut loser_drawing_line: String = std::iter::repeat(' ').take(drawing.width - 6 - match_name.len()).collect();
                loser_drawing_line.push_str(&match_name);
                loser_drawing_line.push_str(" ─────");
                match_drawings[loser_game].push(RenderedBlock {
                    lines: vec![loser_drawing_line],
                    out_line: 0,
                    width: drawing.width,
                    game_latency: drawing.game_latency
                });
                let padding = drawing.width - 9 - match_name.len(); // Really should be width, not length, but names are ASCII
                let mut line: String = std::iter::repeat(' ').take(padding).collect();
                line.push_str(&match_name);
                line
            },
            Award(award_min, _award_max) => {
                let award_str = format!("(#{})", award_min);
                let padding = drawing.width - 9 - award_str.len(); // Really should be width, not length, but names are ASCII
                let mut line: String = std::iter::repeat(' ').take(padding).collect();
                line.push_str(&award_str);
                line
            }
        };
        middle_line.push_str(" ├───────");

        drawing.lines.extend(input_drawings[0].lines.drain(..));
        drawing.lines.push(middle_line);
        drawing.lines.extend(input_drawings[1].lines.drain(..));

        match bracket.game_results[match_id][0] {
            Continue(win_match) => {
                match_drawings[win_match].push(drawing);
            },
            Award(award_min, _award_max) => {
                drawing.lines[drawing.out_line].push_str(&format!(" #{}", award_min));
                final_drawings.push((award_min, drawing));
            }
        }
    }

    let mut max_latency = 0;
    final_drawings.sort_by_key(|&(award, _)| award);
    for &(_, ref drawing) in final_drawings.iter() {
        max_latency = std::cmp::max(drawing.game_latency, max_latency);
        for line in &drawing.lines {
            println!("{}", line);
        }
        println!();
    }
    println!("Games: {}, Rounds: {}", bracket.game_results.len(), max_latency);
}
