from bisect import bisect_right
from collections import Counter
from itertools import accumulate 


class Solution:
    def minimumDeletions(self, word: str, k: int) -> int:
        c = Counter(word)
        nums = sorted(list(c.values()))
        psum = list(accumulate(nums, initial=0))
        res = psum[-1]
        for i, num in enumerate(nums):
            x = num + k
            j = bisect_right(nums, x, lo=i)
            res = min(res, psum[-1] - psum[j] - (len(nums) - j) * x + psum[i])
        return res
