from typing import List
from itertools import pairwise


class Solution:
    def minimizeMax(self, nums: List[int], p: int) -> int:
        nums.sort()
        lo, hi = 0, nums[-1] - nums[0]
        while lo < hi:
            mid = (hi - lo) // 2 + lo
            if self.is_valid(nums, p, mid):
                hi = mid
            else:
                lo = mid + 1
        return lo

    def is_valid(self, nums: List[int], p: int, diff: int) -> bool:
        it = pairwise(nums)
        cnt = 0
        for x, y in it:
            if abs(x - y) <= diff:
                cnt += 1
                next(it, None)
        return cnt >= p
