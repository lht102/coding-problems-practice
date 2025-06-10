import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_maxDifference(self):
        s = Solution()
        self.assertEqual(s.maxDifference("aaaaabbc"), 3)
        self.assertEqual(s.maxDifference("abcabcab"), 1)


if __name__ == "__main__":
    unittest.main()
