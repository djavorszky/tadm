use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

use tadm::sort::insertion_sort::InsertionSort;
use tadm::sort::Sorter;

const RANDOM_LARGE: [usize; 100] = [
    487, 346, 409, 430, 282, 128, 208, 235, 345, 5, 90, 485, 140, 332, 6, 31, 490, 406, 312, 415,
    33, 427, 458, 137, 62, 373, 375, 72, 143, 324, 385, 50, 294, 93, 79, 360, 314, 33, 412, 274,
    443, 373, 81, 238, 494, 206, 404, 242, 216, 5, 277, 251, 93, 299, 459, 361, 446, 103, 64, 43,
    63, 401, 380, 477, 123, 181, 200, 180, 411, 381, 443, 214, 327, 24, 136, 464, 248, 167, 487,
    426, 111, 386, 247, 235, 393, 39, 139, 120, 489, 59, 60, 166, 96, 415, 9, 138, 441, 100, 486,
    97,
];

const REVERSE_LARGE: [usize; 100] = [
    100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78,
    77, 76, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63, 62, 61, 60, 59, 58, 57, 56, 55, 54,
    53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30,
    29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5,
    4, 3, 2, 1,
];

const ASCENDING_LARGE: [usize; 100] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
    99, 100,
];

const RANDOM_SMALL: [usize; 10] = [324, 385, 50, 294, 93, 79, 360, 314, 33, 412];

const REVERSE_SMALL: [usize; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

const ASCENDING_SMALL: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

fn insertion_sort_benchmark(c: &mut Criterion) {
    let sorter = InsertionSort::asc();

    let mut group = c.benchmark_group("insertion_sort");
    //  group.throughput(Throughput::Bytes(*size as u64));

    for (desc, input) in [
        ("large_random", RANDOM_LARGE),
        ("large_asc", ASCENDING_LARGE),
        ("large_desc", REVERSE_LARGE),
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(desc),
            &input,
            move |b, input| {
                b.iter_batched(
                    // Setup lambda
                    || input.clone(),
                    // Measured lambda
                    |mut input| {
                        sorter.sort(&mut input);

                        input
                    },
                    BatchSize::PerIteration,
                );
            },
        );
    }

    for (desc, input) in [
        ("small_random", RANDOM_SMALL),
        ("small_asc", ASCENDING_SMALL),
        ("small_desc", REVERSE_SMALL),
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(desc),
            &input,
            move |b, input| {
                b.iter_batched(
                    // Setup lambda
                    || input.clone(),
                    // Measured lambda
                    |mut input| {
                        sorter.sort(&mut input);

                        input
                    },
                    BatchSize::PerIteration,
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, insertion_sort_benchmark);
criterion_main!(benches);

// TODO(Dan) - check benchmark result and also what else can be done. (optimize the thing.)
