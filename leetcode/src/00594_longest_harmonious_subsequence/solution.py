from typing import List
from collections import Counter


class Solution:
    def findLHS(self, nums: List[int]) -> int:
        c = Counter(nums)
        return max((c[num] + c[num + 1] for num in c if num + 1 in c), default=0)
