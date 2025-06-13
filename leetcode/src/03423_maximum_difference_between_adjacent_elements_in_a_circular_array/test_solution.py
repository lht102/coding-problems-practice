import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_maxAdjacentDistance(self):
        s = Solution()
        self.assertEqual(s.maxAdjacentDistance([1, 2, 4]), 3)
        self.assertEqual(s.maxAdjacentDistance([-5, -10, -5]), 5)


if __name__ == "__main__":
    unittest.main()
