#![feature(iter_collect_into)]

//! Optional direct verification of the nightly API, separate from the stable
//! clear-plus-extend implementation tested by default.

fn main() {
    let batches = [vec![1, -1, 2], vec![], vec![-3, 0], vec![4, 5, 6]];
    let mut buffer = Vec::with_capacity(16);
    let capacity = buffer.capacity();
    for batch in batches {
        let expected: Vec<_> = batch.iter().copied().filter(|value| *value > 0).collect();
        buffer.clear();
        batch
            .into_iter()
            .filter(|value| *value > 0)
            .collect_into(&mut buffer);
        assert_eq!(buffer, expected);
        assert_eq!(buffer.capacity(), capacity);
    }
    println!("perf-collect-into: nightly collect_into behavior and reuse verified");
}
