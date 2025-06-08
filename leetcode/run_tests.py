import unittest
import os
import sys


def discover_and_run_tests():
    test_dir = os.path.dirname(os.path.abspath(__file__))
    src_dir = os.path.join(test_dir, "src")

    parent_dir = os.path.dirname(test_dir)
    if parent_dir not in sys.path:
        sys.path.insert(0, parent_dir)

    print(f"Looking for tests in: {src_dir}")
    print(f"Python path: {sys.path}")

    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.startswith("test_") and file.endswith(".py"):
                print(f"Found test file: {os.path.join(root, file)}")

    loader = unittest.TestLoader()
    suite = loader.discover(src_dir, pattern="test_*.py")

    if suite.countTestCases() == 0:
        print("No tests found. Check if test files match pattern 'test_*.py'")
        return

    runner = unittest.TextTestRunner(verbosity=2)
    runner.run(suite)


if __name__ == "__main__":
    discover_and_run_tests()
