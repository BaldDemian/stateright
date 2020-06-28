//! Utilities for tests.

/// A machine that cycles between two states.
pub mod binary_clock {
    use crate::*;

    #[derive(Clone)]
    pub struct BinaryClock;

    #[derive(Clone, Debug, PartialEq)]
    pub enum BinaryClockAction { GoLow, GoHigh }

    pub type BinaryClockState = i8;

    impl Model for BinaryClock {
        type State = BinaryClockState;
        type Action = BinaryClockAction;

        fn init_states(&self) -> Vec<Self::State> {
            vec![0, 1]
        }

        fn actions(&self, state: &Self::State, actions: &mut Vec<Self::Action>) {
            if *state == 0 {
                actions.push(BinaryClockAction::GoHigh);
            } else {
                actions.push(BinaryClockAction::GoLow);
            }
        }

        fn next_state(&self, _state: &Self::State, action: Self::Action) -> Option<Self::State> {
            match action {
                BinaryClockAction::GoLow  => Some(0),
                BinaryClockAction::GoHigh => Some(1),
            }
        }

        fn properties(&self) -> Vec<Property<Self>> {
            vec![
                Property::always("in [0, 1]", |_, state| {
                    0 <= *state && *state <= 1
                }),
            ]
        }
    }
}

/// A system that solves a linear Diophantine equation in `u8`.
pub mod linear_equation_solver {
    use crate::*;

    /// Given `a`, `b`, and `c`, finds `x` and `y` such that `a*x + b*y = c` where all values are
    /// in `u8`.
    pub struct LinearEquation { pub a: u8, pub b: u8, pub c: u8 }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Guess { IncreaseX, IncreaseY }

    impl Model for LinearEquation {
        type State = (u8, u8);
        type Action = Guess;

        fn init_states(&self) -> Vec<Self::State> {
            vec![(0, 0)]
        }

        fn actions(&self, _state: &Self::State, actions: &mut Vec<Self::Action>) {
            actions.push(Guess::IncreaseX);
            actions.push(Guess::IncreaseY);
        }

        fn next_state(&self, state: &Self::State, action: Self::Action) -> Option<Self::State> {
            let (x, y) = *state;
            match &action {
                Guess::IncreaseX => Some((x.wrapping_add(1), y)),
                Guess::IncreaseY => Some((x, y.wrapping_add(1))),
            }
        }

        fn properties(&self) -> Vec<Property<Self>> {
            vec![
                Property::sometimes("solvable", |equation, solution| {
                    let LinearEquation { a, b, c } = equation;
                    let (x, y) = solution;

                    // dereference and enable wrapping so the equation is succinct
                    use std::num::Wrapping;
                    let (x, y) = (Wrapping(*x), Wrapping(*y));
                    let (a, b, c) = (Wrapping(*a), Wrapping(*b), Wrapping(*c));

                    a*x + b*y == c
                }),
            ]
        }
    }
}
