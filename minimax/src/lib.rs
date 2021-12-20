pub fn minimax<T, I>(mut i: T) -> Option<(I, I)>
where
    T: Iterator<Item = I>,
    I: Ord + Copy,
{
    if let Some(first) = i.next() {
        let mut min = first;
        let mut max = first;
        for item in i {
            if item < min {
                min = item;
            } else if item > max {
                max = item;
            }
        }
        Some((min, max))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
