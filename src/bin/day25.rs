const SUBJECT_NUMBER: usize = 7;
const MODULO: usize = 20201227;

fn transform(number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * number) % MODULO;
    }
    value
}

fn calculate_encryption_key(public_keys: [usize; 2]) -> usize {
    let mut value = 1;
    for loop_size in 1.. {
        value = (value * SUBJECT_NUMBER) % MODULO;
        if value == public_keys[0] {
            return transform(public_keys[1], loop_size);
        } else if value == public_keys[1] {
            return transform(public_keys[0], loop_size);
        }
    }
    unreachable!()
}

fn main() {
    println!("Part 1: {}", calculate_encryption_key([12232269, 19452773]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(calculate_encryption_key([5764801, 17807724]), 14897079);
    }
}
