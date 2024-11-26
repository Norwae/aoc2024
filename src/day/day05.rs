use crate::*;

parsed_day!(|input|{
    Ok::<Vec<i32>, !>(input.lines().filter_map(|l|{
        l.parse::<i32>().ok()
    }).collect::<Vec<_>>())
}, part1, part2);

fn part2(mut data: Vec<i32>) -> usize {
    let mut cursor = 0i32;
    let mut n = 0;
    while cursor >= 0 && cursor < data.len() as i32 {
        let index = cursor as usize;
        let delta =  data[index];
        data[index] += if delta < 3 { 1 } else { -1 };
        cursor += delta;
        n += 1
    }
    n
}
fn part1(input: &mut Vec<i32>) -> usize {
    let mut data = input.clone();
    let mut cursor = 0i32;
    let mut n = 0;
    while cursor >= 0 && cursor < data.len() as i32 {
        let index = cursor as usize;
        let delta =  data[index];
        data[index] += 1;
        cursor += delta;
        n += 1
    }
    n
}