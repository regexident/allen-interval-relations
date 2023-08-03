use criterion::{black_box, criterion_group, criterion_main, Criterion};

use allen_interval_relations::{
    FromIntervals, Interval, IntervalFrom, IntervalFull, IntervalTo, NonEmpty, Relation,
};

#[inline]
fn with_intervals<F>(f: F)
where
    F: Fn(
        &NonEmpty<IntervalFull>,
        &NonEmpty<IntervalFrom<i32>>,
        &NonEmpty<IntervalTo<i32>>,
        &NonEmpty<Interval<i32>>,
    ),
{
    for interval_full in &[IntervalFull] {
        for interval_from in &[
            IntervalFrom { start: 0 },
            IntervalFrom { start: 1 },
            IntervalFrom { start: 2 },
            IntervalFrom { start: 3 },
            IntervalFrom { start: 4 },
            IntervalFrom { start: 5 },
        ] {
            for interval_to in &[
                IntervalTo { end: 1 },
                IntervalTo { end: 2 },
                IntervalTo { end: 3 },
                IntervalTo { end: 4 },
                IntervalTo { end: 5 },
                IntervalTo { end: 6 },
            ] {
                for interval in &[
                    Interval { start: 0, end: 1 },
                    Interval { start: 1, end: 2 },
                    Interval { start: 2, end: 3 },
                    Interval { start: 3, end: 4 },
                    Interval { start: 4, end: 5 },
                ] {
                    black_box(f(
                        black_box(unsafe { &NonEmpty::new_unchecked(interval_full.clone()) }),
                        black_box(unsafe { &NonEmpty::new_unchecked(interval_from.clone()) }),
                        black_box(unsafe { &NonEmpty::new_unchecked(interval_to.clone()) }),
                        black_box(unsafe { &NonEmpty::new_unchecked(interval.clone()) }),
                    ));
                }
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Relation", |b| {
        b.iter(|| {
            with_intervals(|interval_from, interval_full, interval, interval_to| {
                Relation::from_intervals(interval_from, interval_from);
                Relation::from_intervals(interval_from, interval_full);
                Relation::from_intervals(interval_from, interval);
                Relation::from_intervals(interval_from, interval_to);

                Relation::from_intervals(interval_full, interval_from);
                Relation::from_intervals(interval_full, interval_full);
                Relation::from_intervals(interval_full, interval);
                Relation::from_intervals(interval_full, interval_to);

                Relation::from_intervals(interval, interval_from);
                Relation::from_intervals(interval, interval_full);
                Relation::from_intervals(interval, interval);
                Relation::from_intervals(interval, interval_to);

                Relation::from_intervals(interval_to, interval_from);
                Relation::from_intervals(interval_to, interval_full);
                Relation::from_intervals(interval_to, interval);
                Relation::from_intervals(interval_to, interval_to);
            });
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
