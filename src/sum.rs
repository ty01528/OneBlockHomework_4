pub fn sum(numbers: &[u32]) -> Option<u32> {
    let mut result= 0u32;
    for &number in numbers {
        result = match result.checked_add(number as u32) {
            Some(n) => n,
            None => return None,
        };
    }
    Some(result)
}
