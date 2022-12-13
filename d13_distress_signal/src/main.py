from itertools import zip_longest
from pathlib import Path
from typing import List


def main() -> None:
    here = Path(__file__).parent
    # path = here.parent / "example"
    path = here.parent / "input"
    inp = []

    with open(path) as f:
        f = f.read().strip()
        for i, block in enumerate(f.split("\n\n")):
            line1, line2 = block.split("\n")
            line1 = eval(line1)
            line2 = eval(line2)

            comparison = compare(line1, line2)
            print(f"Pair {i + 1}: {comparison}")
            if comparison == 1:
                inp.append(i + 1)

    print(inp)
    print(sum(inp))


def compare(line1: List[any], line2: List[any]) -> int:
    for a, b in zip_longest(line1, line2):
        if a is None:
            return 1
        if b is None:
            return -1

        if type(a) == int and type(b) == int:
            if a < b:
                return 1
            if a > b:
                return -1
        else:
            if type(a) == int:
                a = [a]

            if type(b) == int:
                b = [b]

            comparison = compare(a, b)
            if comparison != 0:
                return comparison

    return 0


if __name__ == "__main__":
    main()
