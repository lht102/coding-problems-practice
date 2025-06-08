from typing import List


class Solution:
    def lexicalOrder(self, n: int) -> List[int]:
        res = []
        stack = []
        for i in range(9, 0, -1):
            if i <= n:
                stack.append(i)
        while stack:
            cur_num = stack.pop()
            res.append(cur_num)
            for next_dig in range(9, -1, -1):
                next_number = cur_num * 10 + next_dig
                if next_number <= n:
                    stack.append(next_number)
        return res
