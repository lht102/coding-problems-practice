from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hm = {}
        for i, num in enumerate(nums):
            j = hm.get(target - num)
            if j is not None:
                return [j, i]
            hm[num] = i
        return []
