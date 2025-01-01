fn main() {
    let inputs = include_str!("input.txt");
    let final_floor = find_basement(&inputs);
    println!("{final_floor}");
}
fn find_basement(directions: &str) -> i32 {
    let mut current_floor: i32 = 0;
    let mut position: i32 = 0;
    for (character_position, direction) in directions.chars().enumerate() {
        match direction {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => current_floor = current_floor,
        }
        position = character_position.try_into().unwrap();
        if current_floor == -1 {
            break;
        }
    }
    position + 1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn postion_1() {
        let directions = String::from(")");
        let no_directions = find_basement(&directions);
        assert_eq!(no_directions, 1);
    }
    #[test]
    fn postion_5() {
        let directions = String::from("()())");
        let no_directions = find_basement(&directions);
        assert_eq!(no_directions, 5);
    }
}
