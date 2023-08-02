use allen_interval_relations::*;

#[test]
fn converses() {
    // Symmetric relations:

    let symmetric_relations = [Relation::Equals];

    for relation in symmetric_relations {
        let first_converse = relation.as_converse();
        assert_eq!(relation, first_converse);

        let second_converse = first_converse.as_converse();
        assert_eq!(relation, second_converse);
    }

    // Asymmetric relations:

    let asymmetric_relations = [
        Relation::Precedes { is_inverted: false },
        Relation::Meets { is_inverted: false },
        Relation::Overlaps { is_inverted: false },
        Relation::Finishes { is_inverted: false },
        Relation::Contains { is_inverted: false },
        Relation::Starts { is_inverted: false },
    ];

    for relation in asymmetric_relations {
        let first_converse = relation.as_converse();
        assert_ne!(relation, first_converse);

        let second_converse = first_converse.as_converse();
        assert_eq!(relation, second_converse);
    }
}

#[test]
fn validate() {
    let validator = IntervalValidator;

    let min = isize::min_value();
    let mid = 0;
    let max = isize::max_value();

    let err = IntervalError::EmptyInterval;

    // RangeFull
    assert_eq!(validator.validate_interval(&(..)), Ok(()));

    // RangeTo<isize>
    assert_eq!(validator.validate_interval(&(..min)), Err(err));
    assert_eq!(validator.validate_interval(&(..mid)), Ok(()));
    assert_eq!(validator.validate_interval(&(..max)), Ok(()));

    // RangeToInclusive<isize>
    assert_eq!(validator.validate_interval(&(..=min)), Err(err));
    assert_eq!(validator.validate_interval(&(..=mid)), Ok(()));
    assert_eq!(validator.validate_interval(&(..=max)), Ok(()));

    // RangeFrom<isize>
    assert_eq!(validator.validate_interval(&(min..)), Ok(()));
    assert_eq!(validator.validate_interval(&(mid..)), Ok(()));
    assert_eq!(validator.validate_interval(&(max..)), Err(err));

    // Range<isize>
    assert_eq!(validator.validate_interval(&(min..min)), Err(err));
    assert_eq!(validator.validate_interval(&(mid..mid)), Err(err));
    assert_eq!(validator.validate_interval(&(max..max)), Err(err));

    assert_eq!(validator.validate_interval(&(max..mid)), Err(err));
    assert_eq!(validator.validate_interval(&(mid..min)), Err(err));

    assert_eq!(validator.validate_interval(&(min..mid)), Ok(()));
    assert_eq!(validator.validate_interval(&(mid..max)), Ok(()));

    // RangeInclusive<isize>
    assert_eq!(validator.validate_interval(&(min..=min)), Err(err));
    assert_eq!(validator.validate_interval(&(mid..=mid)), Err(err));
    assert_eq!(validator.validate_interval(&(max..=max)), Err(err));

    assert_eq!(validator.validate_interval(&(max..=mid)), Err(err));
    assert_eq!(validator.validate_interval(&(mid..=min)), Err(err));

    assert_eq!(validator.validate_interval(&(min..=mid)), Ok(()));
    assert_eq!(validator.validate_interval(&(mid..=max)), Ok(()));
}

#[test]
fn precedes() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Precedes { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ────────────────┐
    // t:                          └───────────────────── ─ ─
    assert_eq!(Relation::from_ranges(..4, 5..), EXPECTED);
    assert_eq!(Relation::from_ranges(..=4, 5..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ────────────────┐
    // t:                          └──────────────┘
    assert_eq!(Relation::from_ranges(..4, 5..8), EXPECTED);
    assert_eq!(Relation::from_ranges(..=4, 5..=8), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:      ┌──────────────┐
    // t:                          └───────────────────── ─ ─
    assert_eq!(Relation::from_ranges(1..4, 5..), EXPECTED);
    assert_eq!(Relation::from_ranges(1..=4, 5..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:      ┌──────────────┐
    // t:                          └──────────────┘
    assert_eq!(Relation::from_ranges(1..4, 5..8), EXPECTED);
    assert_eq!(Relation::from_ranges(1..=4, 5..=8), EXPECTED);
}

#[test]
fn is_preceded_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Precedes { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌───────────────────── ─ ─
    // t: ─ ─ ────────────────┘
    assert_eq!(Relation::from_ranges(5.., ..4), EXPECTED);
    assert_eq!(Relation::from_ranges(5.., ..=4), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌──────────────┐
    // t: ─ ─ ────────────────┘
    assert_eq!(Relation::from_ranges(5..8, ..4), EXPECTED);
    assert_eq!(Relation::from_ranges(5..=8, ..=4), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌───────────────────── ─ ─
    // t:      └──────────────┘
    assert_eq!(Relation::from_ranges(5.., 1..4), EXPECTED);
    assert_eq!(Relation::from_ranges(5.., 1..=4), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌──────────────┐
    // t:      └──────────────┘
    assert_eq!(Relation::from_ranges(5.., 1..4), EXPECTED);
    assert_eq!(Relation::from_ranges(5.., 1..=4), EXPECTED);
}

#[test]
fn meets() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Meets { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ─────────────────────┐
    // t:                          └───────────────────── ─ ─
    assert_eq!(Relation::from_ranges(..5, 5..), EXPECTED);
    assert_eq!(Relation::from_ranges(..5, 5..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ─────────────────────┐
    // t:                          └──────────────┘
    assert_eq!(Relation::from_ranges(..5, 5..8), EXPECTED);
    assert_eq!(Relation::from_ranges(..5, 5..8), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:           ┌──────────────┐
    // t:                          └───────────────────── ─ ─
    assert_eq!(Relation::from_ranges(2..5, 5..), EXPECTED);
    assert_eq!(Relation::from_ranges(2..5, 5..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:           ┌──────────────┐
    // t:                          └──────────────┘
    assert_eq!(Relation::from_ranges(2..5, 5..8), EXPECTED);
    assert_eq!(Relation::from_ranges(2..5, 5..8), EXPECTED);
}

#[test]
fn is_met_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Meets { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌───────────────────── ─ ─
    // t: ─ ─ ─────────────────────┘
    assert_eq!(Relation::from_ranges(5.., ..5), EXPECTED);
    assert_eq!(Relation::from_ranges(5.., ..=5), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌──────────────┐
    // t: ─ ─ ─────────────────────┘
    assert_eq!(Relation::from_ranges(5..8, ..5), EXPECTED);
    assert_eq!(Relation::from_ranges(5..=8, ..=5), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌───────────────────── ─ ─
    // t:           └──────────────┘
    assert_eq!(Relation::from_ranges(5.., 2..5), EXPECTED);
    assert_eq!(Relation::from_ranges(5.., 2..=5), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌──────────────┐
    // t:           └──────────────┘
    assert_eq!(Relation::from_ranges(5..8, 2..5), EXPECTED);
    assert_eq!(Relation::from_ranges(5..=8, 2..=5), EXPECTED);
}

#[test]
fn overlaps() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Overlaps { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ──────────────────────────┐
    // t:                     └────────────────────────── ─ ─
    assert_eq!(Relation::from_ranges(..6, 4..), EXPECTED);
    assert_eq!(Relation::from_ranges(..=6, 4..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ──────────────────────────┐
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(..6, 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(..=6, 4..=7), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                ┌──────────────┐
    // t:                     └────────────────────────── ─ ─
    assert_eq!(Relation::from_ranges(3..6, 4..), EXPECTED);
    assert_eq!(Relation::from_ranges(3..=6, 4..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                ┌──────────────┐
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(3..6, 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(3..=6, 4..=7), EXPECTED);
}

#[test]
fn is_overlapped_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Overlaps { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌────────────────────────── ─ ─
    // t: ─ ─ ──────────────────────────┘
    assert_eq!(Relation::from_ranges(4.., ..6), EXPECTED);
    assert_eq!(Relation::from_ranges(4.., ..=6), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t: ─ ─ ──────────────────────────┘
    assert_eq!(Relation::from_ranges(4..7, ..6), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, ..=6), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌────────────────────────── ─ ─
    // t:                └──────────────┘
    assert_eq!(Relation::from_ranges(4.., 3..6), EXPECTED);
    assert_eq!(Relation::from_ranges(4.., 3..=6), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t:                └──────────────┘
    assert_eq!(Relation::from_ranges(4..7, 3..6), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, 3..=6), EXPECTED);
}

#[test]
fn starts() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Starts { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t:                     └────────────────────────── ─ ─
    assert_eq!(Relation::from_ranges(4..7, 4..), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, 4..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t:                     └───────────────────┘
    assert_eq!(Relation::from_ranges(4..7, 4..8), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, 4..=8), EXPECTED);
}

#[test]
fn is_started_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Starts { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌────────────────────────── ─ ─
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(4.., 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(4.., 4..=7), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌───────────────────┐
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(4..8, 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=8, 4..=7), EXPECTED);
}

#[test]
fn encloses() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Contains { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                ┌─────────────────────────────── ─ ─
    // t:                     └─────────┘
    assert_eq!(Relation::from_ranges(3.., 4..6), EXPECTED);
    assert_eq!(Relation::from_ranges(3.., 4..=6), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                ┌───────────────────┐
    // t:                     └─────────┘
    assert_eq!(Relation::from_ranges(3..7, 4..6), EXPECTED);
    assert_eq!(Relation::from_ranges(3..=7, 4..=6), EXPECTED);
}

#[test]
fn is_enclosed_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Contains { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌─────────┐
    // t:                └─────────────────────────────── ─ ─
    assert_eq!(Relation::from_ranges(4..6, 3..), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=6, 3..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌─────────┐
    // t:                └───────────────────┘
    assert_eq!(Relation::from_ranges(4..6, 3..7), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=6, 3..=7), EXPECTED);
}

#[test]
fn finishes() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Finishes { is_inverted: false });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t: ─ ─ ───────────────────────────────┘
    assert_eq!(Relation::from_ranges(4..7, ..7), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, ..=7), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌──────────────┐
    // t:                └───────────────────┘
    assert_eq!(Relation::from_ranges(4..7, 3..7), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=7, 3..=7), EXPECTED);
}

#[test]
fn is_finished_by() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Finishes { is_inverted: true });

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // t: ─ ─ ───────────────────────────────┐
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(..7, 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(..=7, 4..=7), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                ┌───────────────────┐
    // t:                     └──────────────┘
    assert_eq!(Relation::from_ranges(3..7, 4..7), EXPECTED);
    assert_eq!(Relation::from_ranges(3..=7, 4..=7), EXPECTED);
}

#[test]
fn equals() {
    const EXPECTED: Result<Relation, IntervalError> = Ok(Relation::Equals);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ─────────────────────────────────────────── ─ ─
    // t: ─ ─ ─────────────────────────────────────────── ─ ─
    assert_eq!(Relation::from_ranges(.., ..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                          ┌───────────────────── ─ ─
    // t:                          └───────────────────── ─ ─
    assert_eq!(Relation::from_ranges(5.., 5..), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s: ─ ─ ─────────────────────┐
    // t: ─ ─ ─────────────────────┘
    assert_eq!(Relation::from_ranges(..5, ..5), EXPECTED);
    assert_eq!(Relation::from_ranges(..=5, ..=5), EXPECTED);

    //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
    // s:                     ┌─────────┐
    // t:                     └─────────┘
    assert_eq!(Relation::from_ranges(4..6, 4..6), EXPECTED);
    assert_eq!(Relation::from_ranges(4..=6, 4..=6), EXPECTED);
}
