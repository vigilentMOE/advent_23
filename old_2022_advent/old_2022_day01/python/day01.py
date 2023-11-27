from os.path import join, dirname
from pydantic.types import List
from typing import Tuple


def read_input_file(file_path) -> List[List[int]]:
    """
    Read the input file and return the data in a suitable format.
    """
    with open(file_path, 'r') as file:
        data: List[List[int]] = []
        current_list: List[int] = []
        for line in file:
            stripped_line = line.strip()
            if stripped_line == '':
                data.append(current_list)
                current_list = []
            else:
                current_list.append(int(stripped_line))
        if current_list:
            data.append(current_list)

    return data


def calculate_calories(data: List[List[int]]) -> List[int]:
    """
    Calculate the total calories carried by each elf.
    """
    return [sum(elf_calories) for elf_calories in data]


def find_max_calorie_elf(calories: List[int]) -> Tuple[int, int]:
    """
    Find the elf carrying the most calories.
    """
    max_calories = calories[0]
    max_index = 0

    for i in range(1, len(calories)):
        if calories[i] > max_calories:
            max_calories = calories[i]
            max_index = i

    # Return the maximum calories and the index at which it was found
    return max_calories, max_index


def main():
    file_path = join(dirname(__file__), 'calories.txt')

    data: List[List[int]] = read_input_file(file_path)

    calories: List[int] = calculate_calories(data) 

    max_calorie_elf: Tuple[int, int] = find_max_calorie_elf(calories)
    highest_count: int = max_calorie_elf[0]
    print(f'The elf carrying the most calories is carrying a total of '
            f'{highest_count} calories.')


if __name__ == "__main__":
    main()
