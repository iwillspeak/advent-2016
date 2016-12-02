extern crate advent;

use advent::day1::*;
static  MOVES: &'static str = "R3, L5, R1, R2, L5, R2, R3, L2, L5, R5, L4, L3, R5, L1, R3, R4, R1, L3, R3, L2, L5, L2, R4, R5, R5, L4, L3, L3, R4, R4, R5, L5, L3, R2, R2, L3, L4, L5, R1, R3, L3, R2, L3, R5, L194, L2, L5, R2, R1, R1, L1, L5, L4, R4, R2, R2, L4, L1, R2, R53, R3, L5, R72, R2, L5, R3, L4, R187, L4, L5, L2, R1, R3, R5, L4, L4, R2, R5, L5, L4, L3, R5, L2, R1, R1, R4, L1, R2, L3, R5, L4, R2, L3, R1, L4, R4, L1, L2, R3, L1, L1, R4, R3, L4, R2, R5, L2, L3, L3, L1, R3, R5, R2, R3, R1, R2, L1, L4, L5, L2, R4, R5, L2, R4, R4, L3, R2, R1, L4, R3, L3, L4, L3, L1, R3, L2, R2, L4, L4, L5, R3, R5, R3, L2, R5, L2, L1, L5, L1, R2, R4, L5, R2, L4, L5, L4, L5, L2, L5, L4, R5, R3, R2, R2, L3, R3, L2, L5";

pub fn main() {
    let (_, pos, positions) =  MOVES.split(",")
        .map(|m| { m.trim() })
        .map(|m| { Move::from(m) })
        .fold((Direction::North, Position::new(), Vec::new()), |state, m| {
            let (direction, pos, mut positions) = state;
            let new_direction = direction.turn(m.turn());
            let end_pos = pos.move_by(&new_direction, m.dist());
            positions.extend((1..(m.dist() + 1)).map(|d| {
                pos.move_by(&new_direction, d)
            }));
            (new_direction, end_pos, positions)
        });

    let mut dupes = Vec::new();
    for i in 0..positions.len() {
        let ref pos = positions[i];
        if positions[(i + 1)..].contains(&pos) {
            dupes.push(pos);
        }
    }

    println!("first dupe dist: {}", dupes.first().unwrap().travel_dist());
    println!("end {:?} ({})", pos, pos.travel_dist());
}
