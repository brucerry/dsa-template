//! Kadane's algorithm finds the largest sum of a nonempty contiguous subarray.
//! Returns `None` for empty input. Subarray sums must fit in `i64`.

pub fn kadane(nums: &[i64]) -> Option<i64> {
    let (&first, rest) = nums.split_first()?;
    let mut ending_here = first;
    let mut best = first;
    for &num in rest {
        ending_here = ending_here.max(0) + num;
        best = best.max(ending_here);
    }
    Some(best)
}
