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

## Day 3

After a walk with my father and the dog,
let's do this.
One type of item per compartment, each rucksacks has 2 compartments.
Need to check the current configuration.
Each line (rucksack) can be split in half (2 compartments).
It's searching and eliminating items that are in both compartments.
Priority of item is its position in alphabet (+ 26 if uppercase).
Answer is the sum of priority value of duplicated values.
Parsing and getting priority is okay. 
HashSet are easier to use in Python,
I don't get why `.intersection()` cannot return an instance of `HashSet`.
Right answer on first try.
There are now groups of 3 rucksacks, 
we need to find the elements that comes up in every rucksack.
Grouping by 3 lines should be ok,
adapting my function to take 3 entries instead of 2 a bit bothersome.
It went alright, I used an intermediate set 
instead of juggling with the intersect object.
First guess was correct.

