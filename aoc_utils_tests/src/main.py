from pathlib import Path
import aoc_utils


def part1(path: Path) -> None:
    inp = []

    with open(path) as f:
        for line in f.readlines():
            line = line.strip()
            inp.append(line)

    # aoc_utils.submit("1")


def part2(path: Path) -> None:
    inp = []

    with open(path) as f:
        for line in f.readlines():
            line = line.strip()
            inp.append(line)

    # aoc_utils.submit("1", True)


def main() -> None:
    here = Path(__file__).parent
    path = here.parent / "example"
    # path = here.parent / "input"

    part1(path)
    part2(path)


if __name__ == "__main__":
    main()
