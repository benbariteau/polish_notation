macro_rules! npn {
    [() -> ()] => {};
    [() -> ($stack:tt)] => {$stack};
    [($head:tt $($tail:tt)*) -> ($stack_head:tt + $($stack:tt)*)] => { npn!(($($tail)*) -> (($stack_head + $head) $($stack)*)) };
    [$first:tt $second:tt $($tail:tt)*] => { npn!(($($tail)*) -> ($second $first)) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_add() { assert_eq!(npn!(+ 1 2), 3); }

    /*
    #[test]
    fn basic_subtract() { assert_eq!(npn!(- 1 2), -1); }

    #[test]
    fn basic_multiply() { assert_eq!(npn!(* 3 2), 6); }

    #[test]
    fn basic_divide() { assert_eq!(npn!(/ 8 4), 2); }
    */
}
