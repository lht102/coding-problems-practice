import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_minMaxDifference(self):
        s = Solution()
        self.assertEqual(s.minMaxDifference(11891), 99009)
        self.assertEqual(s.minMaxDifference(90), 99)


if __name__ == "__main__":
    unittest.main()
