use crate::sort::Sorter;
use std::cmp::Ordering;

/// InsertionSort implements the [Sorter](crate::sort::Sorter) trait with the Insertion Sort
/// algorithm. A brief explanation of the sort:
///
/// The algorithm divides the slice into two parts: an already sorted section, at the beginning, and
/// a potentially unsorted section at the end.
///
/// ```
/// // v already sorted array, containing only the number 5
/// [  5, 1, 3, 2, 7];
/// //    ^ Start of unsorted array, containing numbers 1, 3, 2, and 7. Also the next item that will
/// //      be considered for adding to the sorted section, read on below
/// ```
///
/// Assuming our goal is an ascending sort (lowest item at the start of the array, highest item at
/// the end of the array), each step of the algorithm does the following:
///
/// The first item of the unsorted section is considered for the sorted section. It is then compared
/// with the last item of the sorted section.
///
/// If the new item is higher than the previous last in the sorted section, we're done; as the
/// sorted section is already, well, sorted, we can be sure that adding the new item keeps the
/// ordering of the already sorted section.
///
/// If the new item is lower than the previous last in the sorted section, they are swapped. The
/// algorithm then continues and compares the item with the next one in the unsorted section. This
/// is repeated until an item is encountered that is smaller than the one we're trying to place.
///
/// To visualize, continuing on with the previous example:
/// ```
/// // v already sorted
/// [  5, 1, 3, 2, 7];
/// //    ^ next item
/// //      will be swapped with 5
///
/// // v--v already sorted.
/// [  1, 5, 3, 2, 7];
/// //       ^ next item
/// //         will be swapped with 5
///
/// // v-----v already sorted
/// [  1, 3, 5, 2, 7];
/// //          ^ next item.
/// //            will be swapped with 5, then with 3
///
/// // v--------v already sorted
/// [  1, 2, 3, 5, 7];
/// //             ^ next item
/// //               no swap will take place.
///
/// // v-----------v already sorted
/// [  1, 2, 3, 5, 7];
/// // no next item.
///
/// ```
///
///
pub struct InsertionSort {
    ord: Ordering,
}

impl InsertionSort {
    /// Creates an [InsertionSort] struct which will sort the items in ascending order.
    pub fn asc() -> Self {
        Self {
            ord: Ordering::Greater,
        }
    }

    /// Creates an [InsertionSort] struct which will sort the items in descending order.
    pub fn desc() -> Self {
        Self {
            ord: Ordering::Less,
        }
    }
}

impl<T> Sorter<T> for InsertionSort
where
    T: Ord,
{
    fn sort(&self, items: &mut [T]) {
        for i in 1..items.len() {
            for j in (1..=i).rev() {
                if items[j].cmp(&items[j - 1]) == self.ord {
                    break;
                }

                items.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_numbers_asc() {
        let mut input = [2, 1, 5, 3, 4];

        InsertionSort::asc().sort(&mut input);

        assert_eq!(&input, &[1, 2, 3, 4, 5])
    }

    #[test]
    fn sorts_numbers_desc() {
        let mut input = [2, 1, 5, 3, 4];

        InsertionSort::desc().sort(&mut input);

        assert_eq!(&input, &[5, 4, 3, 2, 1])
    }

    #[test]
    fn sorts_empty_slice() {
        let mut input: [i32; 0] = [];

        InsertionSort::desc().sort(&mut input);

        assert_eq!(&input, &[])
    }

    #[test]
    fn sorts_one_element_slice() {
        let mut input = [1];

        InsertionSort::desc().sort(&mut input);

        assert_eq!(&input, &[1])
    }
}
