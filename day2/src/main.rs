use std::fs;
use std::io;
use std::str::FromStr;

fn travel(instructions: &Vec<(&str, isize)>) -> (isize, isize) {
    let mut pos: isize = 0;
    let mut depth: isize = 0;

    for (action, num) in instructions {
        match *action {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => panic!("Unknown action found {}", action), 
        }
    }
    (pos, depth)
}

fn travel_with_aim(instructions: &Vec<(&str, isize)>) -> (isize, isize) {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (action, num) in instructions {
        match *action {
            "forward" => {
                pos += num;
                depth += aim * num;
            },
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!("Unknown action found {}", action), 
        }
    }
    (pos, depth)
}


fn main() -> io::Result<()> {
    let input = fs::read_to_string("input")?;
    let input = input
        .lines()
        .map(|s| {
            let (action, num) = s.split_once(" ").unwrap();
            (action, isize::from_str(num).unwrap())
        })
        .collect();

    let (pos, depth) = travel(&input);

    println!("End Position: {}", pos);
    println!("End Depth: {}", depth);
    println!("Mul: {}", pos * depth);

    let (pos, depth) = travel_with_aim(&input);

    println!("End Position: {}", pos);
    println!("End Depth: {}", depth);
    println!("Mul: {}", pos * depth);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel() {
        let input = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];

        let expected = (15, 10);
        let actual = travel(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_travel_with_aim() {
        let input = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];

        let expected = (15, 60);
        let actual = travel_with_aim(&input);
        assert_eq!(actual, expected);
    }
}
