use crate::submarine::{Movement, Submarine};
use std::{fs, path::Path};

use nom::{
    character::complete::{alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn ident(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((alpha1, space1, digit1))(input)
}

fn block(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    separated_list1(newline, ident)(input)
}

pub fn parse_submarine<P: AsRef<Path>>(path: P) -> Option<Submarine> {
    let movements_data = fs::read_to_string(path).expect("unable to read from file");

    if let Ok(parsed) = block(&movements_data) {
        let mut submarine = Submarine::default();

        let mut movements = parsed.1.iter();

        while let Some(movement_tuple) = movements.next() {
            let movement: Movement = movement_tuple.into();
            submarine.add_movement(movement);
        }

        return Some(submarine);
    }
    return None;
}
