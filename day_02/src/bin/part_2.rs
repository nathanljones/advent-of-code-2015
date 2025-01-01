fn main() {
    let inputs = include_str!("input.txt");

    let total: u32 = inputs.lines().map(length_needed).sum();
    println!("{total}");
}

fn length_needed(dimensions: &str) -> u32 {
    let mut sq_feet: Vec<u32> = dimensions
        .split("x")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    sq_feet.sort();

    let smallest_perimeter = sq_feet[0] + sq_feet[0] + sq_feet[1] + sq_feet[1];
    let volume = sq_feet[0] * sq_feet[1] * sq_feet[2];

    smallest_perimeter + volume
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn ribbon34() {
        let square_feet = length_needed("2x3x4");
        assert_eq!(square_feet, 34);
    }

    #[test]
    fn square_feet_43() {
        let square_feet = length_needed("1x1x10");
        assert_eq!(square_feet, 14);
    }
}
