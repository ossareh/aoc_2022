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



## Summary of things I'd like to readdress:

 -[] d01 conversion of bytes to int; bytes are a str