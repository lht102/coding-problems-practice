import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_longestSubsequence(self):
        s = Solution()
        self.assertEqual(s.longestSubsequence("1001010", 5), 5)
        self.assertEqual(s.longestSubsequence("00101001", 1), 6)
        self.assertEqual(
            s.longestSubsequence(
                "111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101",
                11713332,
            ),
            96,
        )


if __name__ == "__main__":
    unittest.main()
