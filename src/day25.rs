use crate::day::Day;

pub struct Day25 {}

impl Day for Day25 {
    fn part1(&self, input: &str) -> String {
        // parse public keys
        let card_public_key = input.lines().next().unwrap().parse().unwrap();
        let door_public_key = input.lines().skip(1).next().unwrap().parse().unwrap();

        // reverse engineer loop size
        let mut card_loop_size = None;
        let mut door_loop_size = None;

        let mut value = 1i64;
        let mut loops = 0;
        while card_loop_size == None || door_loop_size == None {
            value *= 7;
            value %= 20201227;
            loops += 1;

            if value == card_public_key {
                card_loop_size = Some(loops);
            }
            if value == door_public_key {
                door_loop_size = Some(loops);
            }
        }

        // determine most efficient loop size
        let loop_size;
        let subject_number;
        if card_loop_size.unwrap() < door_loop_size.unwrap() {
            loop_size = card_loop_size.unwrap();
            subject_number = door_public_key;
        } else {
            loop_size = door_loop_size.unwrap();
            subject_number = card_public_key;
        }

        // calculate encryption key
        value = 1;
        for _ in 0..loop_size {
            value *= subject_number;
            value %= 20201227;
        }

        value.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day25{}.part1("5764801
17807724"), "14897079");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day25{}.part2(""), "");
    }
}
