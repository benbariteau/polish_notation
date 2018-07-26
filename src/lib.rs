//#![recursion_limit="128"]

macro_rules! npn {
    [() -> ()] => {};
    [() -> ($stack:tt)] => {$stack};
    [(+ $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (+ $($stack)*)) };
    [(- $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (- $($stack)*)) };
    [(* $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (* $($stack)*)) };
    [(/ $($tail:tt)*) -> ($($stack:tt)*)] => { npn!(($($tail)*) -> (/ $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (+ $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (+ + $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- - $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* * $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ / $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / / $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (+ - $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- + $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* - $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- * $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (+ * $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* + $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ + $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ - $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / - $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (/ * $($stack:tt)*)] => { npn!(($($tail)*) -> ($head / * $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (+ / $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + / $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (- / $($stack:tt)*)] => { npn!(($($tail)*) -> ($head - / $($stack)*)) };
    [($head:tt $($tail:tt)*) -> (* / $($stack:tt)*)] => { npn!(($($tail)*) -> ($head * / $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head + $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt - $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head - $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt * $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head * $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt / $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head / $head) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second + $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt - $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second - $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt * $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second * $stack_first) $($stack)*)) };
    [($($tail:tt)*) -> ($stack_first:tt $stack_second:tt / $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_second / $stack_first) $($stack)*)) };
    [$first:tt $second:tt $($tail:tt)*] => { npn!(($($tail)*) -> ($second $first)) };
}

#[cfg(test)]
mod tests {
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
}
