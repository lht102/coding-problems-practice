import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_maxRunTime(self):
        s = Solution()
        self.assertEqual(s.maxRunTime(2, [3, 3, 3]), 4)
        self.assertEqual(s.maxRunTime(2, [1, 1, 1, 1]), 2)
        self.assertEqual(s.maxRunTime(3, [10, 10, 3, 5]), 8)
        self.assertEqual(
            s.maxRunTime(
                12, [11, 89, 16, 32, 70, 67, 35, 35, 31, 24, 41, 29, 6, 53, 78, 83]
            ),
            43,
        )


if __name__ == "__main__":
    unittest.main()
