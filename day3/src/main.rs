use std::{fs, collections::HashSet};

use itertools::Itertools;

const LOWER_CASE_A_CODE: u32 = 'a' as u32;
const UPPER_CASE_A_CODE: u32 = 'A' as u32;

fn main(){
    let content = fs::read_to_string("./input.txt").unwrap();
    first(&content);
    second(&content);
    println!("{},{}", LOWER_CASE_A_CODE, UPPER_CASE_A_CODE)
}

fn first(input: &str) {
    let summary = input.lines().map(|line| {
        let comp1: HashSet<char> = HashSet::from_iter(line[0..(line.len() / 2)].chars());
        let comp2 = &line[(line.len() / 2)..line.len()];

        comp2.chars()
             .filter(|item| comp1.contains(item))
             .map(|item| match item {
                    
                    'a'..='z' => item as u32 - LOWER_CASE_A_CODE + 1,
                    'A'..='Z' => item as u32 - UPPER_CASE_A_CODE + 27,
                    _ => unreachable!(),

        })
        .next()
        .unwrap()
    }).sum::<u32>();

    println!("{}", summary);
}

fn second(input: &str) {
    let summary = input.lines()
                        .chunks(3)
                        .into_iter()
                        .map(|mut chunk| {
                        let comp1: HashSet<char> = HashSet::from_iter(chunk.next().unwrap().chars());
                        let comp2: HashSet<char> = HashSet::from_iter(chunk.next().unwrap().chars());

                            chunk
                            .next()
                            .unwrap()
                            .chars()
                            .filter(move |item| { comp1.contains(item) && comp2.contains(item) })
                            .map(|item| match item {
                                'a'..='z' => item as u32 - LOWER_CASE_A_CODE + 1,
                                'A'..='Z' => item as u32 - UPPER_CASE_A_CODE + 27,
                                _ => unreachable!(),
                            })
                            .next()
                            .unwrap()
                        }).sum::<u32>();
    println!("{}", summary)
}