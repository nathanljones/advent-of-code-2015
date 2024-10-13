use std::collections::HashSet;

const NORTH: char = '^';
const SOUTH: char = 'v';
const EAST: char = '>';
const WEST: char = '<';

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
fn main() {
    let inputs = include_str!("input.txt");
    let no_houses = how_many_houses_get_present(&inputs);
    println!("{no_houses}");
}

fn how_many_houses_get_present(directions: &str) -> u32 {
    let mut positions_visited: HashSet<Position> = HashSet::new();

    let mut santa_position = Position { x: 0, y: 0 };
    let mut robo_santa_position = Position { x: 0, y: 0 };
    positions_visited.insert(santa_position);
    //positions_visited.
    for (loop_count, direction) in directions.chars().enumerate() {
        let mut current_position = Position { x: 0, y: 0 };

        if (loop_count + 1) % 2 == 0 {
            current_position = robo_santa_position;
        } else {
            current_position = santa_position;
        }
        match direction {
            NORTH => current_position.x += 1,
            SOUTH => current_position.x -= 1,
            EAST => current_position.y += 1,
            WEST => current_position.y -= 1,
            _ => println!("invalid direction"),
        }
        positions_visited.insert(current_position);
        if (loop_count + 1) % 2 == 0 {
            robo_santa_position = current_position;
        } else {
            santa_position = current_position;
        }
    }
    positions_visited.len() as u32
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn deliver_to_three() {
        let no_houses = how_many_houses_get_present("^v");
        assert_eq!(no_houses, 3);

        let no_houses = how_many_houses_get_present("^>v<");
        assert_eq!(no_houses, 3);
    }

    #[test]
    fn deliver_to_eleven() {
        let no_houses = how_many_houses_get_present("^v^v^v^v^v");
        assert_eq!(no_houses, 11);
    }
}
