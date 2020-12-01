// https://adventofcode.com/2020/day/1

pub fn find(list: &[i32], amount: usize, predicate: fn(&Vec<i32>) -> bool) -> Option<Vec<i32>> {
    use itertools::Itertools;

    list.iter()
        .cloned()
        .combinations_with_replacement(amount)
        .find(predicate)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path)?;

    let expense_list = input_data
        .split('\n')
        .into_iter()
        .map(|f| f.parse().unwrap())
        .collect::<Vec<i32>>();

    let pair = find(&expense_list, 2, |v| v.iter().sum::<i32>() == 2020).unwrap();
    let triple = find(&expense_list, 3, |v| v.iter().sum::<i32>() == 2020).unwrap();

    println!("[part 1] {:?} => {:?}", pair, pair.iter().product::<i32>());
    println!(
        "[part 2] {:?} => {:?}",
        triple,
        triple.iter().product::<i32>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_pair_expense() {
        // given
        let list = vec![1721, 979, 366, 299, 675, 1456];

        // when
        let expenses = find(&list, 2, |v| v.iter().sum::<i32>() == 2020).unwrap();
        let product: i32 = expenses.iter().product();

        // then
        assert_eq!(product, 514579);
    }

    #[test]
    fn gets_the_triple_expense() {
        // given
        let list = vec![1721, 979, 366, 299, 675, 1456];

        // when
        let expenses = find(&list, 3, |v| v.iter().sum::<i32>() == 2020).unwrap();
        let product: i32 = expenses.iter().product();

        // then
        assert_eq!(product, 241861950);
    }
}
