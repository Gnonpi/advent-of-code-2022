# advent-of-code-2022
Let's try and make it!

[Advent of code](https://adventofcode.com/)

## Day 1

Getting everything setup, 
selecting `reqwest`and `dotenvy` to automate communication.
Not caring about `unwrap`.
First solution is ok, the reqwest part was the messiest.
Submitting by hand.
Second part is getting the sum of the top three.

## Day 2

After a day without reliable internet,
let's see.
Copying the Cargo+functions from previous day.
I should probably make a template.
`[A, B, C]` are my plays, `[X, Y, Z]` the ones of the opponents.
Score is `A*1 + B*2 + C*3 + 3*draw + 6*win`.
First question is computing the score following that rule.
Making it slow:
coding the outcome func, then a compute score func.
It's the opposite, ABC is the opponent and I'm XYZ.
Making things the over way, it works, and the result is higher.
Then we change the meaning of XYZ.
It would have been cleaner if I used enum instead of int,
I'm having to deal with offsets.
The sign form a cycle: you can go forward/backward in it.
I'll need to be smarter tomorrow.
