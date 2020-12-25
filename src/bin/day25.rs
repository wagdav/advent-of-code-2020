use std::error::Error;

fn transform(pk: usize, loop_size: usize) -> usize {
    let mut res = 1;
    for _ in 0..loop_size {
        res = (res * pk) % 20201227;
    }

    res
}

fn find_loop_size(pk: usize) -> usize {
    let base = 7;
    let mut res = 1;
    for i in 1.. {
        res = (res * base) % 20201227;
        if res == pk {
            return i;
        }
    }

    0
}

fn solve_part1(door_pk: usize, card_pk: usize) -> usize {
    let door_loop_size = find_loop_size(door_pk);

    transform(card_pk, door_loop_size)
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let card_pk = 13233401;
    let door_pk = 6552760;

    println!("Part 1 {:?}", solve_part1(door_pk, card_pk));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(transform(7, 8), 5764801);
        assert_eq!(transform(7, 11), 17807724);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(17807724, 5764801), 14897079);
    }
}
