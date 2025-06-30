import unittest
from .solution import Solution


class TestSolution(unittest.TestCase):
    def test_(self):
        s = Solution()
        self.assertEqual(s.numSubseq([3, 5, 6, 7], 9), 4)
        self.assertEqual(s.numSubseq([3, 3, 6, 8], 10), 6)
        self.assertEqual(s.numSubseq([2, 3, 3, 4, 6, 7], 12), 61)
        self.assertEqual(s.numSubseq([5, 2, 4, 1, 7, 6, 8], 16), 127)


if __name__ == "__main__":
    unittest.main()
