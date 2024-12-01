use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Welcome to Day 1!");

    let input = read_input();
    let (list1, list2) = split_input_into_2_sorted_lists(input);

    let part_1_answer = part_1(&list1, &list2);
    println!("Part 1 answer - {}", part_1_answer);

    let part_2_answer = part_2(&list1, &list2);
    println!("Part 2 answer - {}", part_2_answer);
}

fn split_input_into_2_sorted_lists(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input {
        let pair = line.split("   ").collect::<Vec<_>>();
        list1.push(pair[0].parse::<i32>().unwrap());
        list2.push(pair[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();
    (list1, list2)
}

fn part_1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    sum
}

fn part_2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let list2_grouped = group_by(&list2);
    let mut sum = 0;
    for value in list1 {
        sum += value * list2_grouped.get(&value).unwrap_or(&0);
    }
    sum
}

fn group_by(list: &Vec<i32>) -> HashMap<i32, i32> {
    let hashmap = list.iter().fold(HashMap::new(), |mut acc, value| {
        *acc.entry(*value).or_insert(0) += 1;
        acc
    });
    hashmap
}

fn read_input() -> Vec<String> {
    let buffer = BufReader::new(File::open("input.txt").unwrap());
    buffer
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}
