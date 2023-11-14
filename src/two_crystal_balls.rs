use std::error::Error;

#[derive(PartialEq, Debug)]
enum CrystalBallError {
    AllValuesFalse,
}

fn two_crystal_balls(haystack: &[bool; 10_000]) -> Result<usize, CrystalBallError> {
    // Doing this with binary search + walking the rest of the array
    // Would still be O(n)
    // Instead of binary search, we do "sqrt" search
    // Meaning at worst we do O(sqrt(n)) + sqrt(n)) which is O(sqrt(n))

    let jump_amount = f32::floor(f32::sqrt(haystack.len() as f32));
    let mut i = jump_amount as usize;

    while (i + jump_amount as usize) < haystack.len() {
        if haystack[i] {
            break;
        } else {
            i += jump_amount as usize
        }
    }

    i -= jump_amount as usize;
    for _ in 0..(jump_amount as usize) {
        if haystack[i] {
            return Ok(i);
        } else {
            i += 1;
        }
    }
    Err(CrystalBallError::AllValuesFalse)
}

mod test {
    use crate::two_crystal_balls::{two_crystal_balls, CrystalBallError};

    fn create_haystack2(breaks: usize) -> [bool; 10] {
        let mut arr = [false; 10];
        for (index, _) in arr.into_iter().enumerate() {
            if index > breaks {
                arr[index] = true;
            }
        }

        arr
    }
    fn create_haystack(breaks: usize) -> [bool; 10_000] {
        let mut arr = [false; 10_000];
        for (index, _) in arr.into_iter().enumerate() {
            if index >= breaks {
                arr[index] = true;
            }
        }

        arr
    }

    #[test]
    fn it_works() {
        assert_eq!(two_crystal_balls(&create_haystack(341)).unwrap(), 341);
    }

    #[test]
    fn it_throws_err() {
        assert_eq!(
            two_crystal_balls(&[false; 10_000]).unwrap_err(),
            CrystalBallError::AllValuesFalse,
        );
    }
}
