import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_distributeCandies(self):
        s = Solution()
        self.assertEqual(s.distributeCandies(5, 2), 3)
        self.assertEqual(s.distributeCandies(3, 3), 10)


if __name__ == "__main__":
    unittest.main()
