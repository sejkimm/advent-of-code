import os
from typing import List, Dict, Tuple

INF: int = 1_000_000_000

type_type: Dict[str, List[Tuple[int, int]]] = {
    "S": [(1, 0), (0, 1), (-1, 0), (0, -1)],
    "|": [(1, 0), (-1, 0)],
    "-": [(0, 1), (0, -1)],
    "L": [(-1, 0), (0, 1)],
    "J": [(1, 0), (0, -1)],
    "7": [(-1, 0), (0, 1)],
    "F": [(1, 0), (0, 1)],
    ".": [(0, 0)],
}

def parse_tile_map(input_lines: List[str]) -> Tuple[List[List[type_type]], Tuple[int, int]]:
    tile_map: List[List[type_type]] = []

    for row_idx, input_line in enumerate(input_lines):
        stripped_line: str = input_line.strip()
        row: List[type_type] = []

        for col_idx, char in enumerate(stripped_line):
            row.append(type_type[char])
            if char == "S":
                starting_point: Tuple[int, int] = (row_idx, col_idx)

        tile_map.append(row)

    return (tile_map, starting_point)


def solve(input_lines: List[str]) -> int:
    answer: int = 0

    tile_info: Tuple[List[List[type_type]], Tuple[int, int]] = parse_tile_map(input_lines)

    tile_map: List[List[type_type]] = tile_info[0]
    starting: Tuple[int, int] = tile_info[1]

    distance_from_starting: List[List[int]] = list(map(lambda col: [INF] * len(col), tile_map))

    bfs_queue: List[Tuple[int, int]] = [starting]
    distance_from_starting[starting[0]][starting[1]] = 0

    while len(bfs_queue) > 0:

    return answer


def main(**kwargs):
    input_file_path = os.path.join(os.path.dirname(__file__), kwargs["input_file_name"])

    with open(input_file_path, "r", encoding="utf-8") as input_file:
        input_lines: List[str] = input_file.readlines()

        answer: int = solve(input_lines)

        print(f"answer : {answer}")

    return answer


if __name__ == "__main__":
    # INPUT_FILE_NAME: str = "input.txt"
    INPUT_FILE_NAME: str = "example_input.txt"
    main(input_file_name=INPUT_FILE_NAME)
