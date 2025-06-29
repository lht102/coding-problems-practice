class Solution:
    def minMaxDifference(self, num: int) -> int:
        rem = num
        first_d = 0
        first_d_below_nine = 0
        while rem > 0:
            d = rem % 10
            if d != 9:
                first_d_below_nine = d
            first_d = d
            rem //= 10
        return self.remap_digit(num, first_d_below_nine, 9) - self.remap_digit(
            num, first_d, 0
        )

    def remap_digit(self, num: int, x: int, y: int) -> int:
        if x == y:
            return num
        res = num
        rem = num
        a = 1
        while rem > 0:
            if rem % 10 == x:
                res += (y - x) * a
            a *= 10
            rem //= 10
        return res
