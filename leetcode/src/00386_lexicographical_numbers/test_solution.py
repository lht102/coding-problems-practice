import unittest
from .solution import Solution


class TestTwoSum(unittest.TestCase):
    def test_lexicalOrder(self):
        s = Solution()
        self.assertEqual(
            s.lexicalOrder(13), [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        )
        self.assertEqual(s.lexicalOrder(2), [1, 2])


if __name__ == "__main__":
    unittest.main()
