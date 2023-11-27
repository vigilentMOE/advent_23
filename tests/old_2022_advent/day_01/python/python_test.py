import unittest
from old_2022_advent.old_2022_day01.python.day01 import read_input_file, \
    calculate_calories, find_max_calorie_elf


class Test(unittest.TestCase):

    def test_read_input_file(self):
        with open('test_file.txt', 'w') as file:
            file.write('1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\
                       \n8000\n9000\n\n10000\n')

        expected_output = [[1000, 2000, 3000], [4000], \
                           [5000, 6000], [7000, 8000, 9000], [10000]]

        actual_output = read_input_file('test_file.txt')

        self.assertEqual(actual_output, expected_output)

    def test_calculate_calories(self):

        data = [[1000, 2000, 3000], [4000], [5000, 6000], \
                [7000, 8000, 9000], [10000]]
        result = calculate_calories(data)

        self.assertEqual(result, [6000, 4000, 11000, 24000, 10000])

    def test_find_max_calorie_elf(self):

        data = [6000, 4000, 11000, 24000, 10000]
        result = find_max_calorie_elf(data)

        self.assertEqual(result, (24000, 3))
