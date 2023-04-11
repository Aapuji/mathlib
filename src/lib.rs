pub fn some_fn(a: i32) -> i32 {
    a + 10
}


#[cfg(test)]
mod tests {
    use crate::some_fn;

    #[test]
    fn test_some_fn() {
        for i in -10 .. 10 {
            assert_eq!(some_fn(i), i+1);
        }
    }
}