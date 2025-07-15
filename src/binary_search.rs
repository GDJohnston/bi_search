use std::cmp::Ordering;

/// Implements the binary search algorithm
pub fn binary_search(data: &[u32], x: &u32) -> bool {
    let mut l = 0;
    let mut r = data.len() - 1;

    while l <= r {
        let m = (l + r) / 2; //(l.add(r) as f32).div(2.0).floor() as usize;
        match data[m].cmp(x) {
            Ordering::Equal => break,
            Ordering::Greater => r = m - 1,
            Ordering::Less => l = m + 1,
        }
    }
    l <= r
}
