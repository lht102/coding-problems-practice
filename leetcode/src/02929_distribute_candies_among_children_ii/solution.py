class Solution:
    def distributeCandies(self, n: int, limit: int) -> int:
        acc = 0
        [
            acc := acc + max(min(limit, n - i) - max(0, n - i - limit) + 1, 0)
            for i in range(min(n, limit) + 1)
        ]
        return acc
