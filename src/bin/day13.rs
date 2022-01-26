use num::integer::lcm;

fn find_earliest_time_and_bus(buses: &[usize], time: usize) -> (usize, usize) {
    buses
        .iter()
        .map(|b| {
            (
                if time % *b == 0 {
                    time
                } else {
                    time + *b - (time % *b)
                },
                *b,
            )
        })
        .min()
        .expect("there should be at least one")
}

fn find_sync_timestamp(offsetted_buses: &[(usize, usize)]) -> usize {
    let mut t = 0;
    let mut period = 1;
    for (offset, bus_id) in offsetted_buses {
        while (t + *offset) % *bus_id != 0 {
            t += period;
        }
        period = lcm(period, *bus_id);
    }
    t
}

fn main() {
    let time = 1000677;
    let buses_raw = "29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,661,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,521,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";
    let offsetted_buses: Vec<(usize, usize)> = buses_raw
        .split(',')
        .enumerate()
        .map(|(offset, s)| (offset, s.parse::<usize>()))
        .filter(|(_, b)| Result::is_ok(b))
        .map(|(offset, b)| (offset, b.expect("errors should have been filtered out")))
        .collect::<Vec<_>>();
    let buses: Vec<usize> = offsetted_buses.iter().map(|(_, bus)| *bus).collect();
    let (earliest_time, earliest_bus) = find_earliest_time_and_bus(&buses, time);
    println!("Part 1: {}", (earliest_time - time) * earliest_bus);
    println!("Part 2: {}", find_sync_timestamp(&offsetted_buses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(find_sync_timestamp(&[(0, 17), (2, 13), (3, 19)]), 3417);
    }
}
