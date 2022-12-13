from functools import cmp_to_key
from itertools import zip_longest
from pathlib import Path
from typing import Any, List


def part1(path: Path) -> None:
    inp = []

    with open(path) as f:
        f = f.read().strip()
        for block in f.split("\n\n"):
            line1, line2 = block.split("\n")
            line1 = eval(line1)
            line2 = eval(line2)
            inp.append((line1, line2))

    result = []
    for i, (line1, line2) in enumerate(inp):
        comparison = compare(line1, line2)
        # print(f"Pair {i + 1}: {comparison}")
        if comparison == 1:
            result.append(i + 1)

    print(result)
    print("PART 1:", sum(result))


def part2(path: Path) -> None:
    inp = []

    with open(path) as f:
        for line in f.readlines():
            if line == "\n":
                continue
            line = eval(line)
            inp.append(line)

    key1 = [[2]]
    key2 = [[6]]
    inp.append(key1)
    inp.append(key2)

    result = sorted(inp, key=cmp_to_key(compare), reverse=True)
    i1 = result.index(key1) + 1
    i2 = result.index(key2) + 1

    print("\n".join(str(i) for i in result))
    print("PART 2:", i1, i2, i1 * i2)


def compare(line1: List[Any], line2: List[Any]) -> int:
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


def main() -> None:
    here = Path(__file__).parent
    path = here.parent / "example"
    # path = here.parent / "input"

    part1(path)
    part2(path)


if __name__ == "__main__":
    main()
