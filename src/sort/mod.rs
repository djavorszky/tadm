pub mod insertion_sort;

/// Sorter is a handy trait to allow multiple distinct implementations of the same sorting
/// mechanism.
pub trait Sorter<T>
where
    T: Ord,
{
    fn sort(&self, items: &mut [T]);
}
