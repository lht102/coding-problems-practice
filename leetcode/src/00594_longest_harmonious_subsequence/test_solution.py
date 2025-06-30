import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_findLHS(self):
        s = Solution()
        self.assertEqual(s.findLHS([1, 3, 2, 2, 5, 2, 3, 7]), 5)
        self.assertEqual(s.findLHS([1, 2, 3, 4]), 2)
        self.assertEqual(s.findLHS([1, 1, 1, 1]), 0)
        self.assertEqual(s.findLHS([1, 3, 5]), 0)


if __name__ == "__main__":
    unittest.main()
