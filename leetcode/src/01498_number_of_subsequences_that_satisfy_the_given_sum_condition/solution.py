from typing import List
from itertools import accumulate, repeat
from bisect import bisect_right


class Solution:
    def numSubseq(self, nums: List[int], target: int) -> int:
        M = 10**9 + 7
        nums.sort()
        n = len(nums)
        pows = list(
            accumulate(repeat(0, n - 1), lambda acc, _: (acc * 2) % M, initial=1)
        )

        res = 0
        for i, num in enumerate(nums):
            k = target - num
            j = bisect_right(nums, k)
            if i < j:
                res = (res + pows[j - 1 - i]) % M
        return res
