# Advent of Code 2022 Solutions

Based on: https://adventofcode.com/

I've spent a bunch of time in the second half of this year in Rust; implementing relatively low
level byte related request responses. Like many folks when jumping into something new I've only
really picked up a working knowledge of the byte abstractions that Rust has. I'm going to use AoC'22
to focus in on flexing various parts of Rust.

I'll keep notes on the solutions below.

## Day 1 (d01)

This was my first AoC (ever!) I just decided to power through it. After slowing down to write d02,
d01 seems painful! Given another crack at it I'd definitely break out the inner loops into functions
and refine my byte handling.

It got me wondering how one converts a string rep for a number into an int. In the solution as 
written I use Rusts ability to safely and correctly parse a sequence of bytes into a str, and then
parse the str as an int. In the future I may come back and provide an alternate solution to this.

## Day 2 (d02)

Knowing better the format I had a better idea of how long I should spend on each challenge. In this
case I decided to spend the time trying to model things as enums particular with a focus on using
`std::convert::From`. Refactoring to implement strategy_2 was quite simple. It resulted in some
duplication, but I'm fine with that. I've not really messed with the Iterator abstraction in Rust, 
whenever I get near it I really like what I see.

Working through this got me interested in how one would write this using `no_std`. Particularly
interesting w.r.t. reading the data file. I'd like to tackle this without losing too much of the
seperation of concerns - I feel like iterators play a part in this perhaps? 

## Summary of things I'd like to address:

 - [ ] d01 conversion of bytes to int; bytes are a str
 - [ ] d02 write a no_std version