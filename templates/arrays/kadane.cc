/*
 * Kadane's algorithm finds the maximum sum of a contiguous subarray.
 * The input must be nonempty. Runs in O(n) time and uses O(1) extra space.
 */

#include <algorithm>
#include <vector>

int kadane(std::vector<int>& nums) {
    int res = nums[0], cursum = 0;
    for (int num : nums) {
        cursum = std::max(cursum + num, num);
        res = std::max(res, cursum);
    }
    return res;
}