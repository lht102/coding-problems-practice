class Solution:
    def longestSubsequence(self, s: str, k: int) -> int:
        cur_sum = 0
        res = 0
        bit_len = k.bit_length()
        for i, ch in enumerate(s[::-1]):
            if ch == "1":
                if i < bit_len and (1 << i) + cur_sum <= k:
                    cur_sum += 1 << i
                    res += 1
            else:
                res += 1
        return res
