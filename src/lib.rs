macro_rules! npn {
    [() -> ()] => {};
    [() -> ($stack:tt)] => {$stack};
    [($head:tt $($tail:tt)*) -> (+ + $($stack:tt)*)] => { npn!(($($tail)*) -> ($head + + $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head + $head) $($stack)*)) };
    [($head:tt $($tail:tt)*) -> ($stack_head:tt - $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head - $head) $($stack)*)) };
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

    /*
    #[test]
    fn basic_multiply() { assert_eq!(npn!(* 3 2), 6); }

    #[test]
    fn basic_divide() { assert_eq!(npn!(/ 8 4), 2); }
    */
}
