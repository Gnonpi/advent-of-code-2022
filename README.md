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

## Day 4

Had a long car trip,
had a bit of time to think about the problem.
It's guessing the inclusivity of intervals.
Using size of range to reduce checks,
just compare lower and upper bounds.
With all the time to think, first star was easy.
Next we want to check overlap between the two parts of the section.
It's checking the left-right parts of the bounds of the two sections.
I made a first attempt, 
but I forgot to order the pair with the leftmost item first.
The tests didn't pick it up.
Second attempt was correct.

## Day 5

Let's move things into a template folder.
It's about re-arranging crates, 
following a sequence of instructions to move elements,
and read the top of each pile.
And just that day the problem is complex enough to be splitted in modules.
All those structures to parse is a bit of a pain.
I should add logging in the template.
Argh it was my test case that was wrong, I lost 20mn on this.
Once the parsing and concat done,
solving the problem is easy.
Got the answer on first attempt.
Next part is just changing the way the crates are moved.
A bit of move issue with the `split_at` method.
And done in one try.
Strings with Rust is not easy.

## Day 6

It was a quiet day, let's start.
The template is copied.
It's about detecting the position of the 
first 4 chars that are different.
More strings!
`Iterator.next_chunk` would have been perfect, 
but it's in nightly only.
First start was quite easy,
working with string->char and a hashset to identify distinct sequence.
The second part is making a scan with 14 chars instead of 4.
It shouldn't be a problem to adapt my functions.
It took me like 3 minutes to get the second star.
It seemed super easy. 
Maybe it was my code that hadn't the "4" value hardcoded?

## Day 7

Haut les coeurs!
I saw some bash commands in the problem statement,
it's going to be fun parsing that in Rust :harold:.
So it's going to be parsing simplified `cd` and `ls`,
get a listing of the directories,
get the ones that are big enough.
Should I keep the tree information or is flat ok?
In the example, `a/` contains `e/` that contains the file,
so the structure is important.
Abusing `From<String>` to parse the input.
I didn't want to implement a recursive tree in Rust
(maybe another time),
so I went with a more iterative approach.
It took a loooong time (it's 1am),
but I managed to get the first star.
I'm going to bed.
Second part should be easy enough for tomorrow.
In the morning, yes, second part was easier 
now that the abstractions are built correctly.

## Day 8

Let's do this.
So we have a map of trees, with their height,
and we need to find a sort of ranking from each direction.
First attempt is way too high.
But I'm not sure where I made a mistake?
I might let it go for today: 
maybe I'll go back to it this weekend.

## Day 9

After yesterday hardship,
let's hope for a better day today.
Ok, so it's a Head followed by a Tail,
I think we can ditch the grid and focus on coordinates.

I'm taking back the challenge on the 11th,
didn't finish the challenge on Friday
and took yesterday off.

After nothing that the head and tail can overlap,
plus that the tail only move when it's not adjacent,
everything is working fine.
First star collected.
I got one position weird in my test (4-4 becoming a 0-0),
but skipping it.

The second part is about extending from 2 points to n points.
That's interesting.
I'm starting by splitting into modules,
I'll probably add a new struct that takes the solution of part1
and apply it as a cord.
There are 10 nodes (H, 1 ... 8, T).
Creating that new struct when super fine.
Got the next star on first attempt.

## Day 10

I finished day 9 quite ok,
let's move to yesterday's challenge.
So it's a problem so increasing/decreasing a register
with some cycle skips. 
And then reading some values at defined cycles.
We need to keep the value at every cycle,
if it's a few hundreds/thousands, it should be ok.
First iterative approach is not okay, too complicated.
Maybe I can "paint" a vector.
I went with painting, it's then a bit of map+reduce and we're done!
Second part, we need to iterate over our registers,
and if the register value is around the counter, draw one.
Got the solution, but I'm perplex as the solution
needed a human eye or something to read the letters.

## Day 11


