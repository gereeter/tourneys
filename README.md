# Tournament Simulator

A (rough around the edges) program to display tournament brackets and evaluate them on various quality measures. Example tournaments are provided in the `tourneys` folder, some designed by me.

## Measures of quality

Roughly speaking, we want tournaments to be fair, entertaining, and efficient.

"Fairness" can be broken into a few different aspects:
- Accuracy. A tournament should rank players according to their actual skill levels. Note that, due to the way tournament results are usually interpreted, getting the top rankings correct is much more important than getting the bottom rankings.
- Equal Opportunity. A player should not be disadvantaged or advantaged simply because of where they are placed in the bracket. The easiest way to achieve this is to have a symmetrical bracket, where every path to a win is identical to every other path. Note that seeding goes directly counter to this idea.

"Entertainment" is obviously subjective, but for our purposes consists of:
- Winnable Matches. Blowouts where everybody knows who will win are often both boring and demoralizing. This could also be considered an aspect of fairness - not being thrown up agains vastly superior opponents.
- Hope. Players who are still competing should always have the ability to win the tournament if they win every upcoming game. Very few brackets satisfy this exactly, and perfect adherence is not necessarily that important. In fact, many brackets (such as round robin or the Swiss System) explicitly reject this as a goal, in complete favor of participation.
- Participation. Players should get the opportunity to play many games, and should not have long series of byes in a row. This is often in direct opposition to hope and efficiency.

Efficiency is the property of producing a ranking in a small number of both matches and rounds.

## Usage

Run
```
cargo run /path/to/bracket.tourney view
```
to get a visual representation of a bracket. This can be used to evaluate efficiency, hope, and participation.

Run
```
cargo run /path/to/bracket.tourney simulate
```
to get measures of fairness and entertainment. The "bias" returned is the log absolute difference between a player's actual skill ranking and where they placed in the tournament, and for a given tournament we combine the biases with either the L2 norm (root mean square) or the L∞ norm (maximum). Low values of bias imply high accuracy. Additionally, the standard deviation of bias across many tournaments gives a measurement of equal opportunity - a high variance implies that certain arrangements of players are significantly more unfair than others. The square margin is how close games were, squared. It gives a measure of how winnable matches are, with low square margins implying close games that could go either way. Finally, for more detail, there is a grid showing the probabilities that a person of a given skill rank gets a certain rank in the tournament. High numbers on the diagonal show high accuracy, and the off-diagonal elements show what sort of biases the bracket is likely to have.

## Simulation Model

Players are modeled as having a single, normally distributed skill value. When they compete, the difference of the players' skills is added to a normally distributed luck value, and the sign of the result determines the winner.

## Links

[tourneygeek.com](https://tourneygeek.com/) has some interesting articles and has a similar definition of fairness, though they evaluate it differently - in particular, I aim to make sure that the rankings of all players matter (though in particular the top 3). [tournamentdesign.org](https://www.tournamentdesign.org/) has a collection of fairly well-designed brackets.
