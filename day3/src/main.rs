use std::fs;
use std::io;

fn bool_to_signed(bools: &Vec<bool>) -> Vec<isize> {
    bools
        .iter()
        .map(|x| match x {
            false => -1,
            true => 1,
        })
        .collect()
}

fn add_arrays(x: Vec<isize>, y: Vec<isize>) -> Vec<isize> {
    x.iter().zip(y).map(|(a, b)| a + b).collect()
}

fn count_bits(bit_vec: &Vec<Vec<bool>>) -> Vec<isize> {
    bit_vec
        .iter()
        .map(|x| bool_to_signed(x))
        .reduce(|acc, x| add_arrays(acc, x))
        .unwrap()
}

fn greq_zero(ints: &Vec<isize>) -> Vec<bool> {
    ints.iter().map(|x| x >= &0).collect()
}

fn leq_zero(ints: &Vec<isize>) -> Vec<bool> {
    ints.iter().map(|x| x < &0).collect()
}

fn to_base_10(bools: &Vec<bool>) -> usize {
    let mut acc = 0;

    for (i, x) in bools.iter().rev().enumerate() {
        match x {
            true => acc += 2_usize.pow(i as u32),
            false => continue,
        }
    }

    acc
}

fn rating(input: &Vec<Vec<bool>>, greq: bool, n_bits: usize) -> Vec<bool> {
    let mut input = input.to_owned();

    for i in 0..n_bits {
        let acc_bits = count_bits(&input);

        let subset_bool = if greq {
            greq_zero(&acc_bits).to_owned()
        } else {
            leq_zero(&acc_bits).to_owned()
        };

        let iter_input = input.to_owned();

        let filtered: Vec<&Vec<bool>> = iter_input
            .iter()
            .filter(|x| x[i] == subset_bool[i])
            .collect();

        input = Vec::new();

        for item in filtered {
            input.push(item.to_owned());
        }

        if input.len() == 1 {
            return input.first().unwrap().to_owned();
        }
    }
    panic!("Filtered out all the possible items!");
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input")?;

    let input: Vec<Vec<bool>> = input
        .lines()
        .map(|x| -> Vec<bool> { x.chars().map(|c| c.to_digit(10).unwrap() != 0).collect() })
        .collect();

    let acc_bits = count_bits(&input);

    let gamma_bool = greq_zero(&acc_bits);
    let gamma_rate = to_base_10(&gamma_bool);

    let epsilon_bool = leq_zero(&acc_bits);
    let epsilon_rate = to_base_10(&epsilon_bool);

    let total_consumption = gamma_rate * epsilon_rate;

    let n_bits = acc_bits.len();

    let oxygen_bool = rating(&input, true, n_bits);
    let oxygen_rating = to_base_10(&oxygen_bool);

    let co2_bool = rating(&input, false, n_bits);
    let co2_rating = to_base_10(&co2_bool);

    let life_support = oxygen_rating * co2_rating;

    println!("Gamma rate: {}", gamma_rate);
    println!("Epsion rate: {}", epsilon_rate);
    println!("Total Consumption: {}", total_consumption);

    println!("Oxygen rating: {}", oxygen_rating);
    println!("CO2 rating: {}", co2_rating);
    println!("Life Support rating: {}", life_support);

    Ok(())
}
