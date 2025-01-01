fn main() {
    let inputs = include_str!("input.txt");

    let total: u32 = inputs.lines().map(no_square_feet).sum();
    println!("{total}");
}

fn no_square_feet(dimensions: &str) -> u32 {
    let mut sq_feet: Vec<u32> = dimensions
        .split("x")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    //the sq_feet vector is as follows
    //[0] = Length
    //[1] = width
    //[2] = height
    // calc is 2*l*w + 2*w*h + 2*h*l
    let wrapping_paper_needed = (2 * sq_feet[0] * sq_feet[1])
        + (2 * sq_feet[1] * sq_feet[2])
        + (2 * sq_feet[2] * sq_feet[0]);

    sq_feet.sort();

    let area_smallest_side = sq_feet[0] * sq_feet[1];

    wrapping_paper_needed + area_smallest_side
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn square_feet_58() {
        let square_feet = no_square_feet("2x3x4");
        assert_eq!(square_feet, 58);
    }

    #[test]
    fn square_feet_43() {
        let square_feet = no_square_feet("1x1x10");
        assert_eq!(square_feet, 43);
    }
}
