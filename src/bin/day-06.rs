use std::io;

fn main() -> io::Result<()> {
    println!("part 1: {}", part_one(INPUT_ONE));

    Ok(())
}

fn part_one(input: &[Race]) -> usize {
    input
        .iter()
        .map(|race| dbg!(count_successes(race)))
        .product()
}

fn count_successes(
    Race {
        duration,
        distance_record,
    }: &Race,
) -> usize {
    // The shape of possible max distances traces out a parabola: y = wait * (duration - wait)
    // we can use the quadratic function to find the places where this parabola intersects
    // the line of the distance record and those two roots are the minimum wait to win
    // and the maximum wait.
    let target = *distance_record as f64 + 1.0;
    let sqrt_component = f64::sqrt(f64::powi(*duration as f64, 2) - (4.0 * target));
    let min = f64::ceil((*duration as f64 - sqrt_component) / 2.0);
    let max = f64::floor((*duration as f64 + sqrt_component) / 2.0);
    (max - min) as usize + 1 // + 1 because range is inclusive
}

// My first pass iterated over possible wait times, plugging the wait time in for x
// in the quadratic equation above. It gave me the right answer and allowed me to
// confirm that my "find the root" approach was working correctly.
// fn successes<'r>(race: &'r Race) -> impl Iterator<Item = usize> + 'r {
//     trials(race).filter(|dist| dist > &race.distance_record)
// }

// fn trials<'r>(
//     Race {
//         duration,
//         distance_record: _,
//     }: &'r Race,
// ) -> impl Iterator<Item = usize> + 'r {
//     (1..*duration).map(|wait_duration| wait_duration * (*duration - wait_duration))
// }

#[derive(Debug)]
struct Race {
    duration: usize,
    distance_record: usize,
}

#[test]
fn example_one() {
    let input = [
        Race {
            duration: 7,
            distance_record: 9,
        },
        Race {
            duration: 15,
            distance_record: 40,
        },
        Race {
            duration: 30,
            distance_record: 200,
        },
    ];

    assert_eq!(288, part_one(&input));
}

const INPUT_ONE: &'static [Race] = &[
    Race {
        duration: 46,
        distance_record: 358,
    },
    Race {
        duration: 68,
        distance_record: 1054,
    },
    Race {
        duration: 98,
        distance_record: 1807,
    },
    Race {
        duration: 66,
        distance_record: 1080,
    },
];
