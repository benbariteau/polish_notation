#[macro_export]
macro_rules! npn {
    // base cases
    [() -> ()] => {};
    [() -> ($stack:tt)] => {$stack};

    // whenever there is an operator as the next token, always push it onto the stack
    [(+ $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (+ $($stack)*)) };
    [(- $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (- $($stack)*)) };
    [(* $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (* $($stack)*)) };
    [(/ $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (/ $($stack)*)) };

    // whenever the operator is at the top of the stack, always push the next token, since an
    // operator needs at least two values to work
    [($head:tt $($tail:tt)*) -> (+ $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / $($stack)*)) };

    // if we've gotten to here, we know that the first element on the stack isn't an operator and
    // neither is the next token, so we can safely produce an expresion from the next token and
    // top two elements on the stack and push it back onto the stack
    [($head:tt $($tail:tt)*) -> ($stack_head:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head + $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt - $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head - $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt * $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head * $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt / $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head / $head) $($stack)*)) };

    // sometimes the above rules fail to apply, usually in the case where an expression is placed
    // onto the stack by one of the last 4 rules, meaning that the top 3 elements on the stack may
    // be an expression that can be resolved.
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second + $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt - $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second - $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt * $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second * $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt / $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second / $stack_first) $($stack)*)) };

    // this is the initialization step
    [$first:tt $second:tt $($tail:tt)*] => { npn!(($($tail)*) -> ($second $first)) };
}

#[cfg(test)]
mod npn_tests {
    #[test]
    fn basic_add() { assert_eq!(npn!(+ 1 2), 3); }
    #[test]
    fn double_add() { assert_eq!(npn!(+ + 1 2 3), 6); }
    #[test]
    fn lots_of_add() {
        // any more than this and we hit the macro recursion limit
        assert_eq!(npn!(+ + + + + + + + + + + + + + + + + + + + + + + + + + + + + + + 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32), 528);
    }

    #[test]
    fn basic_subtract() { assert_eq!(npn!(- 1 2), -1); }
    #[test]
    fn double_subtract() { assert_eq!(npn!(- - 5 2 4), -1); }

    #[test]
    fn subtract_and_add() { assert_eq!(npn!(- + 1 2 3), 0); }
    #[test]
    fn add_and_subtract() { assert_eq!(npn!(+ - 1 2 3), 2); }
    #[test]
    fn adds_and_subtracts() { assert_eq!(npn!(+ - - + 1 2 - 3 + 4 5 6 7), 10); }


    #[test]
    fn basic_multiply() { assert_eq!(npn!(* 3 2), 6); }
    #[test]
    fn double_multiply() { assert_eq!(npn!(* 3 * 2 3), 18); }

    #[test]
    fn adds_substracts_and_multiplies_oh_my() { assert_eq!(npn!(* - - * * + + * * * 2 3 2 3 2 3 2 3 2 3 2), 482); }

    #[test]
    fn basic_divide() { assert_eq!(npn!(/ 8 4), 2); }
    #[test]
    fn double_divide() { assert_eq!(npn!(/ 24 / 12 4), 8); }

    #[test]
    fn all_division_pairs() {
        assert_eq!(npn!(/ + / - / * / / +  - / * / 128 2 4 2 64 192 4 2 4 2 32 2 16 8), 4);
    }

    #[test]
    fn wikipedia_tests() {
        assert_eq!(npn!(* - 5 6 7), -7);
        assert_eq!(npn!(- * / 15 - 7 + 1 1 3 + 2 + 1 1), 5);
    }

    #[test]
    fn test_with_non_numbers() {
        let foo = 3;
        let bar = 12;
        assert_eq!(npn!(/ bar foo), 4);

        fn baz() -> i64 { 2 }
        // TODO figure out how to do this without parens
        assert_eq!(npn!(/ bar (baz())), 6);

        // TODO figure out how to do this without parens
        assert_eq!(npn!(+ foo (-3)), 0);
    }
}

#[macro_export]
macro_rules! rpn {
    // base cases
    [() -> ()] => {};
    [() -> ($stack:tt)] => {$stack};

    // if we encounter an operator, take the last two expressions off the stack, and evaluate.
    [(+ $($tail:tt)*) -> ($stack_first:tt $stack_second:tt $($stack:tt)*)] => { rpn!(($($tail)*) -> (($stack_second + $stack_first) $($stack)*)) };
    [(- $($tail:tt)*) -> ($stack_first:tt $stack_second:tt $($stack:tt)*)] => { rpn!(($($tail)*) -> (($stack_second - $stack_first) $($stack)*)) };
    [(* $($tail:tt)*) -> ($stack_first:tt $stack_second:tt $($stack:tt)*)] => { rpn!(($($tail)*) -> (($stack_second * $stack_first) $($stack)*)) };
    [(/ $($tail:tt)*) -> ($stack_first:tt $stack_second:tt $($stack:tt)*)] => { rpn!(($($tail)*) -> (($stack_second / $stack_first) $($stack)*)) };

    // if this is non-operator (a value), push onto the stack
    [($head:tt $($tail:tt)*) -> ($($stack:tt)*)] => { rpn!(($($tail)*) -> ($head $($stack)*)) };

    // this is the initialization step
    [$first:tt $second:tt $($tail:tt)*] => { rpn!(($($tail)*) -> ($second $first)) };
}

#[cfg(test)]
mod rpn_tests {
    #[test]
    fn basic_add() { assert_eq!(rpn!(1 2 +), 3); }
    #[test]
    fn double_add() { assert_eq!(rpn!(1 2 + 3 +), 6); }

    #[test]
    fn basic_subtract() { assert_eq!(rpn!(1 2 -), -1); }
    #[test]
    fn double_subtract() { assert_eq!(rpn!(5 2 - 4 -), -1); }

    #[test]
    fn basic_multiply() { assert_eq!(rpn!(3 2 *), 6); }
    #[test]
    fn double_multiply() { assert_eq!(rpn!(3 2 * 3 *), 18); }

    #[test]
    fn basic_divide() { assert_eq!(rpn!(8 4 /), 2); }
    #[test]
    fn double_divide() { assert_eq!(rpn!(24 12 4 / /), 8); }

    #[test]
    fn all_operators() {
        assert_eq!(rpn!(2 3 + 5 * 15 - 5 /), 2);
    }

    #[test]
    fn wikipedia_tests() {
        assert_eq!(rpn!(15 7 1 1 + - / 3 * 2 1 1 + + -), 5);
    }

    #[test]
    fn test_with_non_numbers() {
        let foo = 3;
        let bar = 12;
        assert_eq!(rpn!(bar foo /), 4);

        fn baz() -> i64 { 2 }
        // TODO figure out how to do this without parens
        assert_eq!(rpn!(bar (baz()) /), 6);

        // TODO figure out how to do this without parens
        assert_eq!(rpn!(foo (-3) +), 0);
    }
}
