import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_minimumDeletions(self):
        s = Solution()
        self.assertEqual(s.minimumDeletions("aabcaba", 0), 3)
        self.assertEqual(s.minimumDeletions("dabdcbdcdcd", 2), 2)
        self.assertEqual(s.minimumDeletions("aaabaaa", 2), 1)
        self.assertEqual(s.minimumDeletions("ahahnhahhah", 1), 2)
        self.assertEqual(s.minimumDeletions("dabdcbdcdcd", 2), 2)


if __name__ == "__main__":
    unittest.main()
