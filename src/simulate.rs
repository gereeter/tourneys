use rand;
use rand::Rng;
use rand::distributions::{Normal, Uniform, Distribution};

use Bracket;
use GameResult::*;

struct StatVar {
    mean: f64,
    // 2nd moment
    total_variation: f64,
    // 3rd moment
    third_central_moment: f64,
    count: usize,
    reservoir: Vec<f64>
}

impl StatVar {
    fn new(reservoir_size: usize) -> StatVar {
        StatVar {
            mean: 0.0,
            total_variation: 0.0,
            third_central_moment: 0.0,
            count: 0,
            reservoir: vec![0.0; reservoir_size]
        }
    }

    fn insert(&mut self, value: f64) {
        // Reservoir sampling
        if self.count < self.reservoir.len() {
            self.reservoir[self.count] = value;
        } else {
            let selection = rand::thread_rng().gen_range(0, self.count);
            if selection < self.reservoir.len() {
                self.reservoir[selection] = value;
            }
        }

        // Formulas from Wikipedia "Algorithms for Computing Variance"
        let delta = value - self.mean();
        let total_count = self.count + 1;
        self.third_central_moment +=
            ((delta / total_count as f64) * (delta * self.count as f64)) * ((delta * (self.count as f64 - 1.0)) / total_count as f64)
            - 3.0 * delta * self.total_variation / total_count as f64;
        self.total_variation +=
            (delta / total_count as f64) * (delta * self.count as f64);
        self.mean += delta / total_count as f64;
        self.count = total_count;
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn std_dev(&self) -> f64 {
        (self.total_variation / (self.count - 1) as f64).sqrt()
    }

    fn skewness(&self) -> f64 {
        self.third_central_moment * (self.count as f64 / self.total_variation).sqrt() / self.total_variation
    }

    fn graph(&self) {
        let center = self.mean();
        let bar_width = self.std_dev() / 2.5;

        let mut buckets = vec![0];
        for sampled in self.reservoir.iter() {
            let bar_index = ((sampled - center) / bar_width).round() as isize;
            if bar_index.abs() as usize > (buckets.len() - 1) / 2 {
                let extra_buckets = bar_index.abs() as usize - (buckets.len() - 1) / 2;
                for _ in 0..extra_buckets {
                    buckets.insert(0, 0);
                }
                for _ in 0..extra_buckets {
                    buckets.push(0);
                }
            }
            let bucket_index = bar_index + ((buckets.len() - 1) / 2) as isize;
            buckets[bucket_index as usize] += 1;
        }

        let scale_down = self.reservoir.len() / 256;
        let excess_scale = (scale_down + 3) / 4;
        for (i, &count) in buckets.iter().enumerate() {
            print!("{:7.3} | ", center + bar_width * (i as isize - ((buckets.len() - 1) / 2) as isize) as f64);
            for _ in 0..count / scale_down {
                print!("#");
            }
            let excess_symbols = [' ', '.', ':', '%'];
            print!("{}", excess_symbols[(count % scale_down) / excess_scale]);
            println!();
        }
    }
}

pub fn analyze(bracket: &Bracket, runs: usize, luck_factor: f64, seed: bool, graph: bool) {
    let mut rng = rand::thread_rng();
    let skill_dist = Normal::new(0.0, 1.0);
    let luck_dist = Normal::new(0.0, luck_factor);

    let mut tables = vec![vec![0; bracket.starting_games.len()]; bracket.starting_games.len()];
    let mut l2_bias = StatVar::new(1024);
    let mut max_bias = StatVar::new(8192);
    let mut margin_sq = StatVar::new(1024);
    for iter in 0..runs {
        let mut skills: Vec<_> = bracket.starting_games.iter().map(|_| skill_dist.sample(&mut rng)).collect();
        if seed {
            skills.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        let mut ranks = vec![0; skills.len()];
        for (i, &skill) in skills.iter().enumerate() {
            for (other_i, &other_skill) in skills.iter().enumerate() {
                if i != other_i && (skill > other_skill || (skill == other_skill && i > other_i)) {
                    ranks[i] += 1;
                }
            }
        }

        let mut tourney_bias = 0.0;
        let mut tourney_max_bias: f64 = 0.0;
        let mut tourney_margin = 0.0;

        let mut participants = vec![vec![]; bracket.game_results.len()];
        for (starter, &game) in bracket.starting_games.iter().enumerate() {
            participants[game].push(starter);
        }
        for game in 0..bracket.game_results.len() {
            assert_eq!(participants[game].len(), 2);
            let roll = skills[participants[game][0]] - skills[participants[game][1]] + luck_dist.sample(&mut rng);
            tourney_margin += roll.powi(2);
            let (winner, loser) = if roll < 0.0 {
                (0, 1)
            } else {
                (1, 0)
            };
            let win_player = participants[game][winner];
            let lose_player = participants[game][loser];
            match bracket.game_results[game][0] {
                Continue(next_game) => {
                    participants[next_game].push(win_player);
                },
                Award(award_min, award_max) => {
                    let award = Uniform::new_inclusive(award_min, award_max).sample(&mut rng);
                    tables[ranks[win_player]][award - 1] += 1;
                    let game_bias = (((ranks[win_player] + 1) as f64).log(2.0) - (award as f64).log(2.0)).powi(2);
                    tourney_bias += game_bias;
                    tourney_max_bias = tourney_max_bias.max(game_bias);
                },
            }
            match bracket.game_results[game][1] {
                Continue(next_game) => {
                    participants[next_game].push(lose_player);
                },
                Award(award_min, award_max) => {
                    let award = Uniform::new_inclusive(award_min, award_max).sample(&mut rng);
                    tables[ranks[lose_player]][award - 1] += 1;
                    let game_bias = (((ranks[lose_player] + 1) as f64).log(2.0) - (award as f64).log(2.0)).powi(2);
                    tourney_bias += game_bias;
                    tourney_max_bias = tourney_max_bias.max(game_bias);
                },
            }
        }

        l2_bias.insert(tourney_bias.sqrt());
        max_bias.insert(tourney_max_bias.sqrt());
        margin_sq.insert(tourney_margin / bracket.game_results.len() as f64);

        if (iter + 1) % 500000 == 0 {
            eprintln!("{}...", iter + 1);
        }
    }

    println!("--------------------- Stats after {} runs --------------------", runs);
    println!("L2 bias: {:.5} +/- {:.5} (skewness {:.5})", l2_bias.mean(), l2_bias.std_dev(), l2_bias.skewness());
    if graph {
        l2_bias.graph();
    }
    println!("L∞ (max) bias: {:.5} +/- {:.5} (skewness {:.5})", max_bias.mean(), max_bias.std_dev(), max_bias.skewness());
    if graph {
        max_bias.graph();
    }
    println!("Square margin: {:.5} +/- {:.5} (skewness {:.5})", margin_sq.mean(), margin_sq.std_dev(), margin_sq.skewness());
    if graph {
        margin_sq.graph();
    }
    for rank in 0..bracket.starting_games.len() {
        for tested_rank in 0..bracket.starting_games.len() {
            print!("{:6.2}% ", tables[rank][tested_rank] as f64 / runs as f64 * 100.0);
        }
        println!("");
    }
    println!("");
}
