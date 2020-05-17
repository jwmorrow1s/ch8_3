use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct CentralTendency {
    pub mean: f64,
    pub median: f64,
    pub mode: HashSet<i32>,
}

#[allow(dead_code)]
fn mean(v: &Vec<i32>) -> f64 {
    if v.len() == 0 {
        0.0
    } else {
        v.iter().sum::<i32>() as f64 / v.len() as f64
    }
}

#[allow(dead_code)]
fn middle_or_average_of_middle_two(v: &Vec<i32>) -> f64 {
    let list_size_divisible_by_two = v.len() % 2 == 0;

    if v.len() == 1 {
        v[0] as f64
    } else if v.len() == 2 {
        (v[0] as f64 + v[1] as f64) / 2.0
    } else if list_size_divisible_by_two {
        (v[(v.len() / 2) - 1] as f64 + v[((v.len() / 2) - 1) + 1] as f64) / 2.0
    } else {
        v[(v.len() / 2)] as f64
    }
}

#[allow(dead_code)]
fn median(v: &Vec<i32>) -> f64 {
    if v.len() == 0 {
        return 0.0;
    }

    let mut list_copy = v.clone();

    list_copy.sort();

    middle_or_average_of_middle_two(&list_copy)
}

#[allow(dead_code)]
fn mode(v: &Vec<i32>) -> HashSet<i32> {
    if v.len() == 0 {
        return HashSet::new();
    }

    let mut counts: HashMap<i32, usize> = HashMap::new();

    for iter in v.iter() {
        let count = counts.entry(*iter).or_insert(0);
        *count += 1;
    }

    let modes = counts
        .iter()
        .filter(|(_, v)| v.clone() == counts.values().max().unwrap())
        .map(|(k, _)| k.clone())
        .collect::<HashSet<i32>>();

    if modes.len() == v.len() {
        //if all the elements have the maximum count, then there is no mode
        HashSet::new()
    } else {
        modes
    }
}

impl CentralTendency {
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<i32>) -> CentralTendency {
        CentralTendency {
            mean: mean(v),
            median: median(v),
            mode: mode(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn mean_it_works() {
        assert_eq!(mean(&vec!()), 0.0);
        assert_eq!(mean(&vec!(1)), 1.0);
        assert_eq!(mean(&vec!(1, 2, 3, 4, 5)), 3.0);
    }

    #[test]
    fn middle_or_average_of_middle_two_it_works() {
        assert_eq!(middle_or_average_of_middle_two(&vec!(1)), 1.0);
        assert_eq!(middle_or_average_of_middle_two(&vec!(1, 2)), 1.5);
        assert_eq!(middle_or_average_of_middle_two(&vec!(1, 2, 3, 4)), 2.5);
        assert_eq!(middle_or_average_of_middle_two(&vec!(1, 2, 3, 4, 5)), 3.0);
        assert_eq!(
            middle_or_average_of_middle_two(&vec!(1, 2, 3, 4, 5, 6, 7)),
            4.0
        );
    }

    #[test]
    fn median_it_works() {
        assert_eq!(median(&vec!()), 0.0);
        assert_eq!(median(&vec!(1, 2, 3, 4, 5)), 3.0);
        assert_eq!(median(&vec!(4, 3, 1, 2, 5)), 3.0);
        assert_eq!(median(&vec!(1, 2, 4, 5)), 3.0);
        assert_eq!(median(&vec!(5, 1, 4, 2)), 3.0);
    }

    #[test]
    fn mode_it_works() {
        assert_eq!(mode(&vec!()), HashSet::new());
        assert_eq!(mode(&vec!(1, 2, 3)), HashSet::new());
        assert_eq!(
            mode(&vec!(1, 2, 2, 3)),
            HashSet::from_iter(vec!(2).iter().cloned())
        );
        assert_eq!(
            mode(&vec!(1, 2, 2, 3, 3, 4)),
            HashSet::from_iter(vec!(2, 3).iter().cloned())
        );
    }

    #[test]
    fn central_tendency_from_vec_it_works() {
        assert_eq!(
            CentralTendency::from_vec(&vec!()),
            CentralTendency {
                mean: 0.0,
                median: 0.0,
                mode: HashSet::new()
            }
        );
        assert_eq!(
            CentralTendency::from_vec(&vec!(4, 3, 2, 3, 2, 1)),
            CentralTendency {
                mean: 2.5,
                median: 2.5,
                mode: HashSet::from_iter(vec!(2, 3).iter().cloned())
            }
        );
    }
}
