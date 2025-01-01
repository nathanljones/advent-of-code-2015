fn main() {
    let inputs = include_str!("input.txt");
    let final_floor = which_floor(&inputs);
    println!("{final_floor}");
}

fn which_floor(directions: &str) -> i32 {
    let floors_up: i32 = directions
        .chars()
        .filter(|direction| *direction == '(')
        .count()
        .try_into()
        .unwrap();
    let floors_down: i32 = directions
        .chars()
        .filter(|direction| *direction == ')')
        .count()
        .try_into()
        .unwrap();
    floors_up - floors_down
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn floor_0() {
        let directions = String::from("(())");
        let floor = which_floor(&directions);
        assert_eq!(floor, 0);

        let directions = String::from("()()");
        let floor = which_floor(&directions);
        assert_eq!(floor, 0);
    }
    #[test]
    fn floor_3() {
        let directions = String::from("(((");
        let floor = which_floor(&directions);
        assert_eq!(floor, 3);

        let directions = String::from("(()(()(");
        let floor = which_floor(&directions);
        assert_eq!(floor, 3);
    }
    #[test]
    fn floor_also_3() {
        let directions = String::from("))(((((");
        let floor = which_floor(&directions);
        assert_eq!(floor, 3);
    }
    #[test]
    fn floor_minus_1() {
        let directions = String::from("())");
        let floor = which_floor(&directions);
        assert_eq!(floor, -1);

        let directions = String::from("))(");
        let floor = which_floor(&directions);
        assert_eq!(floor, -1);
    }
    #[test]
    fn floor_minus_3() {
        let directions = String::from(")))");
        let floor = which_floor(&directions);
        assert_eq!(floor, -3);

        let directions = String::from(")())())");
        let floor = which_floor(&directions);
        assert_eq!(floor, -3);
    }
}
