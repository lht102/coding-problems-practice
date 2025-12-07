from typing import List
from itertools import islice


class Solution:
    def maxRunTime(self, n: int, batteries: List[int]) -> int:
        if n > len(batteries):
            return 0
        nums = sorted(batteries)
        m = len(batteries) - n
        extra = sum(islice(nums, m))
        live = nums[m:]
        for i in range(n - 1):
            diff = (live[i + 1] - live[i]) * (i + 1)
            if extra < diff:
                return live[i] + extra // (i + 1)
            extra -= diff
        return live[n - 1] + extra // n
