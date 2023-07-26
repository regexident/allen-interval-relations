use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use allen_interval_relations::{AtomicRelations, FromRanges, Relation};

type DiscreteValue = i32;
type NonDiscreteValue = i32;

#[inline]
fn with_ranges_discrete<F>(f: F)
where
    F: Fn(&RangeFull, &RangeFrom<DiscreteValue>, &RangeTo<DiscreteValue>, &Range<DiscreteValue>),
{
    for range_full in &[..] {
        for range_from in &[0.., 1.., 2.., 3.., 4.., 5..] {
            for range_to in &[..1, ..2, ..3, ..4, ..5, ..6] {
                for range in &[0..1, 1..2, 2..3, 3..4, 4..5] {
                    black_box(f(
                        black_box(range_full),
                        black_box(range_from),
                        black_box(range_to),
                        black_box(range),
                    ));
                }
            }
        }
    }
}

#[inline]
fn with_ranges_non_discrete<F>(f: F)
where
    F: Fn(
        &RangeFull,
        &RangeFrom<NonDiscreteValue>,
        &RangeToInclusive<NonDiscreteValue>,
        &RangeInclusive<NonDiscreteValue>,
    ),
{
    for range_full in &[..] {
        for range_from in &[0.., 1.., 2.., 3.., 4.., 5..] {
            for range_to in &[..=1, ..=2, ..=3, ..=4, ..=5, ..=6] {
                for range in &[0..=1, 1..=2, 2..=3, 3..=4, 4..=5] {
                    black_box(f(
                        black_box(range_full),
                        black_box(range_from),
                        black_box(range_to),
                        black_box(range),
                    ));
                }
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Discrete atomic relations", |b| {
        fn function<S, T>(s: S, t: T) -> Option<AtomicRelations>
        where
            AtomicRelations: FromRanges<S, T>,
        {
            AtomicRelations::from_ranges(s, t)
        }

        b.iter(|| {
            with_ranges_discrete(|range_from, range_full, range, range_to| {
                function(range_from.clone(), range_from.clone());
                function(range_from.clone(), range_full.clone());
                function(range_from.clone(), range.clone());
                function(range_from.clone(), range_to.clone());

                function(range_full.clone(), range_from.clone());
                function(range_full.clone(), range_full.clone());
                function(range_full.clone(), range.clone());
                function(range_full.clone(), range_to.clone());

                function(range.clone(), range_from.clone());
                function(range.clone(), range_full.clone());
                function(range.clone(), range.clone());
                function(range.clone(), range_to.clone());

                function(range_to.clone(), range_from.clone());
                function(range_to.clone(), range_full.clone());
                function(range_to.clone(), range.clone());
                function(range_to.clone(), range_to.clone());
            });
        })
    });

    c.bench_function("Non-discrete atomic relations", |b| {
        fn function<S, T>(s: S, t: T) -> Option<AtomicRelations>
        where
            AtomicRelations: FromRanges<S, T>,
        {
            AtomicRelations::from_ranges(s, t)
        }

        b.iter(|| {
            with_ranges_non_discrete(|range_from, range_full, range, range_to| {
                function(range_from.clone(), range_from.clone());
                function(range_from.clone(), range_full.clone());
                function(range_from.clone(), range.clone());
                function(range_from.clone(), range_to.clone());

                function(range_full.clone(), range_from.clone());
                function(range_full.clone(), range_full.clone());
                function(range_full.clone(), range.clone());
                function(range_full.clone(), range_to.clone());

                function(range.clone(), range_from.clone());
                function(range.clone(), range_full.clone());
                function(range.clone(), range.clone());
                function(range.clone(), range_to.clone());

                function(range_to.clone(), range_from.clone());
                function(range_to.clone(), range_full.clone());
                function(range_to.clone(), range.clone());
                function(range_to.clone(), range_to.clone());
            });
        })
    });

    c.bench_function("Discrete relation", |b| {
        fn function<S, T>(s: S, t: T) -> Option<Relation>
        where
            Relation: FromRanges<S, T>,
        {
            Relation::from_ranges(s, t)
        }

        b.iter(|| {
            with_ranges_discrete(|range_from, range_full, range, range_to| {
                function(range_from.clone(), range_from.clone());
                function(range_from.clone(), range_full.clone());
                function(range_from.clone(), range.clone());
                function(range_from.clone(), range_to.clone());

                function(range_full.clone(), range_from.clone());
                function(range_full.clone(), range_full.clone());
                function(range_full.clone(), range.clone());
                function(range_full.clone(), range_to.clone());

                function(range.clone(), range_from.clone());
                function(range.clone(), range_full.clone());
                function(range.clone(), range.clone());
                function(range.clone(), range_to.clone());

                function(range_to.clone(), range_from.clone());
                function(range_to.clone(), range_full.clone());
                function(range_to.clone(), range.clone());
                function(range_to.clone(), range_to.clone());
            });
        })
    });

    c.bench_function("Non-discrete relation", |b| {
        fn function<S, T>(s: S, t: T) -> Option<Relation>
        where
            Relation: FromRanges<S, T>,
        {
            Relation::from_ranges(s, t)
        }

        b.iter(|| {
            with_ranges_non_discrete(|range_from, range_full, range, range_to| {
                function(range_from.clone(), range_from.clone());
                function(range_from.clone(), range_full.clone());
                function(range_from.clone(), range.clone());
                function(range_from.clone(), range_to.clone());

                function(range_full.clone(), range_from.clone());
                function(range_full.clone(), range_full.clone());
                function(range_full.clone(), range.clone());
                function(range_full.clone(), range_to.clone());

                function(range.clone(), range_from.clone());
                function(range.clone(), range_full.clone());
                function(range.clone(), range.clone());
                function(range.clone(), range_to.clone());

                function(range_to.clone(), range_from.clone());
                function(range_to.clone(), range_full.clone());
                function(range_to.clone(), range.clone());
                function(range_to.clone(), range_to.clone());
            });
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
