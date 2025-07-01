import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_possibleStringCount(self):
        s = Solution()
        self.assertEqual(s.possibleStringCount("abbcccc"), 5)
        self.assertEqual(s.possibleStringCount("abcd"), 1)
        self.assertEqual(s.possibleStringCount("aaaa"), 4)
        self.assertEqual(s.possibleStringCount("ere"), 1)


if __name__ == "__main__":
    unittest.main()
