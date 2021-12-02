use std::collections::VecDeque;
use std::fs;
use std::io;
use std::str::FromStr;

fn count_increases(measurements: &Vec<isize>) -> isize {
    let mut prev_val: isize = isize::MAX;
    let mut increase_count: isize = 0;

    for val in measurements.iter() {
        let increment = (val > &prev_val) as isize;

        prev_val = *val;
        increase_count += increment;
    }

    increase_count
}

fn window_sum(measurements: &Vec<isize>, window_size: usize) -> Vec<isize> {
    let mut buffer: VecDeque<isize> = VecDeque::with_capacity(window_size);
    let mut result: Vec<isize> = Vec::new();

    for val in measurements.iter() {
        if buffer.len() == buffer.capacity() {
            let _ = buffer.pop_back();
        }
        buffer.push_front(*val);
        if buffer.len() == buffer.capacity() {
            let window_sum = buffer.iter().sum();
            result.push(window_sum);
        }
    }

    result
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input")?;
    let input = input.lines().map(|s| isize::from_str(s).unwrap()).collect();
    let result = count_increases(&input);
    println!("Day 1: {}", result);

    let window_sums = window_sum(&input, 3);
    let result = count_increases(&window_sums);
    println!("Day 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let input = input.iter().map(|&e| e as isize).collect();

        let expected = 7;
        let actual = count_increases(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_window_sum() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let input = input.iter().map(|&e| e as isize).collect();

        let expected = vec![607, 618, 618, 617, 647, 716, 769, 792];
        let expected: Vec<isize> = expected.iter().map(|&e| e as isize).collect();

        let actual = window_sum(&input, 3);
        assert_eq!(actual, expected);

        let expected = 5;
        let actual = count_increases(&actual);
        assert_eq!(actual, expected);
    }
}
