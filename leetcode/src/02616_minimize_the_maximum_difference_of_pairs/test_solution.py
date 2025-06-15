import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_minimizeMax(self):
        s = Solution()
        self.assertEqual(s.minimizeMax([10, 1, 2, 7, 1, 3], 2), 1)
        self.assertEqual(s.minimizeMax([4, 2, 1, 2], 1), 0)
        self.assertEqual(s.minimizeMax([3, 3, 5, 1, 0, 5, 6, 6], 4), 1)
        self.assertEqual(s.minimizeMax([3, 4, 2, 3, 2, 1, 2], 3), 1)


if __name__ == "__main__":
    unittest.main()
