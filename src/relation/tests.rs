use super::*;

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

mod precedes {
    use super::*;

    const PRECEDES: Relation = Relation::Precedes { is_inverted: false };
    const IS_PRECEDED_BY: Relation = Relation::Precedes { is_inverted: true };

    mod interval_to {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ────────────────┐
        // t:                          └───────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = IntervalTo { end: 4 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 5 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), PRECEDES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_PRECEDED_BY);

            assert!(s.precedes(&t));
            assert!(t.is_preceded_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ────────────────┐
        // t:                          └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalTo { end: 4 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 5, end: 8 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), PRECEDES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_PRECEDED_BY);

            assert!(s.precedes(&t));
            assert!(t.is_preceded_by(&s));
        }
    }

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌──────────────┐
        // t:                          └───────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = Interval { start: 1, end: 4 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 5 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), PRECEDES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_PRECEDED_BY);

            assert!(s.precedes(&t));
            assert!(t.is_preceded_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌──────────────┐
        // t:                          └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 1, end: 4 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 5, end: 8 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), PRECEDES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_PRECEDED_BY);

            assert!(s.precedes(&t));
            assert!(t.is_preceded_by(&s));
        }
    }
}

mod meets {
    use super::*;

    const MEETS: Relation = Relation::Meets { is_inverted: false };
    const IS_MET_BY: Relation = Relation::Meets { is_inverted: true };

    mod interval_to {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────┐
        // t:                          └───────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = IntervalTo { end: 5 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 5 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), MEETS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_MET_BY);

            assert!(s.meets(&t));
            assert!(t.is_met_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────┐
        // t:                          └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalTo { end: 5 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 5, end: 8 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), MEETS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_MET_BY);

            assert!(s.meets(&t));
            assert!(t.is_met_by(&s));
        }
    }

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌───────────────────┐
        // t:                          └───────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = Interval { start: 1, end: 5 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 5 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), MEETS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_MET_BY);

            assert!(s.meets(&t));
            assert!(t.is_met_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌───────────────────┐
        // t:                          └───────────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 1, end: 5 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 5, end: 9 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), MEETS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_MET_BY);

            assert!(s.meets(&t));
            assert!(t.is_met_by(&s));
        }
    }
}

mod overlaps {
    use super::*;

    const OVERLAPS: Relation = Relation::Overlaps { is_inverted: false };
    const IS_OVERLAPPED_BY: Relation = Relation::Overlaps { is_inverted: true };

    mod interval_to {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ──────────────────────────┐
        // t:                     └────────────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = IntervalTo { end: 6 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 4 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), OVERLAPS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_OVERLAPPED_BY);

            assert!(s.overlaps(&t));
            assert!(t.is_overlapped_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ──────────────────────────┐
        // t:                     └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalTo { end: 5 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), OVERLAPS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_OVERLAPPED_BY);

            assert!(s.overlaps(&t));
            assert!(t.is_overlapped_by(&s));
        }
    }

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌──────────────┐
        // t:                     └────────────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = Interval { start: 3, end: 6 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 4 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), OVERLAPS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_OVERLAPPED_BY);

            assert!(s.overlaps(&t));
            assert!(t.is_overlapped_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌──────────────┐
        // t:                     └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 3, end: 6 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), OVERLAPS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_OVERLAPPED_BY);

            assert!(s.overlaps(&t));
            assert!(t.is_overlapped_by(&s));
        }
    }
}

mod starts {
    use super::*;

    const STARTS: Relation = Relation::Starts { is_inverted: false };
    const IS_STARTED_BY: Relation = Relation::Starts { is_inverted: true };

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                     └────────────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 4 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), STARTS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_STARTED_BY);

            assert!(s.starts(&t));
            assert!(t.is_started_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                     └───────────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 8 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), STARTS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_STARTED_BY);

            assert!(s.starts(&t));
            assert!(t.is_started_by(&s));
        }
    }
}

mod contains {
    use super::*;

    const CONTAINS: Relation = Relation::Contains { is_inverted: false };
    const IS_CONTAINED_BY: Relation = Relation::Contains { is_inverted: true };

    mod interval_full {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────────────────────────── ─ ─
        // t:                     └─────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalFull.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 6 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), CONTAINS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_CONTAINED_BY);

            assert!(s.contains(&t));
            assert!(t.is_contained_by(&s));
        }
    }

    mod interval_to {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ──────────────────────────┐
        // t:           └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalTo { end: 6 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 2, end: 5 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), CONTAINS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_CONTAINED_BY);

            assert!(s.contains(&t));
            assert!(t.is_contained_by(&s));
        }
    }

    mod interval_from {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌─────────────────────────────── ─ ─
        // t:                     └──────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = IntervalFrom { start: 3 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), CONTAINS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_CONTAINED_BY);

            assert!(s.contains(&t));
            assert!(t.is_contained_by(&s));
        }
    }

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌───────────────────┐
        // t:                     └─────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 3, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 4, end: 6 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), CONTAINS);
            assert_eq!(Relation::from_intervals(&t, &s), IS_CONTAINED_BY);

            assert!(s.contains(&t));
            assert!(t.is_contained_by(&s));
        }
    }
}

mod finishes {
    use super::*;

    const FINISHES: Relation = Relation::Finishes { is_inverted: false };
    const IS_FINISHED_BY: Relation = Relation::Finishes { is_inverted: true };

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t: ─ ─ ───────────────────────────────┘
        #[test]
        fn vs_interval_to() {
            let s: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalTo { end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), FINISHES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_FINISHED_BY);

            assert!(s.finishes(&t));
            assert!(t.is_finished_by(&s));
        }

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                └───────────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 4, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 3, end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), FINISHES);
            assert_eq!(Relation::from_intervals(&t, &s), IS_FINISHED_BY);

            assert!(s.finishes(&t));
            assert!(t.is_finished_by(&s));
        }
    }
}

mod equals {
    use super::*;

    const EQUALS: Relation = Relation::Equals;

    mod interval_full {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────────────────────────── ─ ─
        // t: ─ ─ ─────────────────────────────────────────── ─ ─
        #[test]
        fn vs_interval_full() {
            let s: NonEmpty<_> = IntervalFull.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFull.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), EQUALS);
            assert_eq!(Relation::from_intervals(&t, &s), EQUALS);

            assert!(s.equals(&t));
            assert!(t.equals(&s));
        }
    }

    mod interval_to {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ───────────────────────────────┐
        // t: ─ ─ ───────────────────────────────┘
        #[test]
        fn vs_interval_to() {
            let s: NonEmpty<_> = IntervalTo { end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalTo { end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), EQUALS);
            assert_eq!(Relation::from_intervals(&t, &s), EQUALS);

            assert!(s.equals(&t));
            assert!(t.equals(&s));
        }
    }

    mod interval_from {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌─────────────────────────────── ─ ─
        // t:                └─────────────────────────────── ─ ─
        #[test]
        fn vs_interval_from() {
            let s: NonEmpty<_> = IntervalFrom { start: 3 }.try_into().unwrap();
            let t: NonEmpty<_> = IntervalFrom { start: 3 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), EQUALS);
            assert_eq!(Relation::from_intervals(&t, &s), EQUALS);

            assert!(s.equals(&t));
            assert!(t.equals(&s));
        }
    }

    mod interval {
        use super::*;

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌───────────────────┐
        // t:                └───────────────────┘
        #[test]
        fn vs_interval() {
            let s: NonEmpty<_> = Interval { start: 3, end: 7 }.try_into().unwrap();
            let t: NonEmpty<_> = Interval { start: 3, end: 7 }.try_into().unwrap();

            assert_eq!(Relation::from_intervals(&s, &t), EQUALS);
            assert_eq!(Relation::from_intervals(&t, &s), EQUALS);

            assert!(s.equals(&t));
            assert!(t.equals(&s));
        }
    }
}
