#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Welcome to Day 1!");

    let input = read_input();
    let (list1, list2) = split_input_into_2_sorted_lists(input);

    let part_1_answer = part_1_zip(&list1, &list2);
    println!("Part 1 answer - {}", part_1_answer);

    let part_2_answer = part_2(&list1, &list2);
    println!("Part 2 answer - {}", part_2_answer);
}

fn split_input_into_2_sorted_lists(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input {
        let pair = line.split_whitespace().collect::<Vec<_>>();
        let pair = line.split_whitespace().collect::<Vec<_>>();
        list1.push(pair[0].parse::<i32>().unwrap());
        list2.push(pair[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();
    (list1, list2)
}

fn part_1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1
        .iter()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (value - list2[index]).abs())
}

fn part_1_zip(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1.iter().zip(list2).map(|(x, y)| (x - y).abs()).sum()
}

fn part_2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let list2_grouped = group_by(&list2);
    list1.iter().fold(0, |acc, value| {
        acc + value * list2_grouped.get(&value).unwrap_or(&0)
    })
}

fn group_by(list: &Vec<i32>) -> HashMap<i32, i32> {
    list.iter().fold(HashMap::new(), |mut acc, value| {
        *acc.entry(*value).or_insert(0) += 1;
        acc
    })
}

fn read_input() -> Vec<String> {
    let buffer = BufReader::new(File::open("input.txt").unwrap());
    buffer
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}
