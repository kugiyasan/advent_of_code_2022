from typing import Callable, List
from pathlib import Path

operations = [
    lambda x: x * 19,
    lambda x: x + 6,
    lambda x: x * x,
    lambda x: x + 3,
]


class Monkey:
    __slots__ = ("items", "operation", "test")

    def __init__(
        self, items: List[int], operation: Callable[[int], int], test: List[int]
    ) -> None:
        self.items = items
        self.operation = operation
        self.test = test

    def __repr__(self) -> str:
        # return f"Monkey({self.items!r}, {self.operation!r}, {self.test!r})"
        return f"Monkey({self.items})"


def read_monkeys() -> List[Monkey]:
    here = Path(__file__).parent
    path = here.parent / "example"
    monkeys = []

    with open(path) as f:
        f = f.read()
        for i, block in enumerate(f.split("\n\n")):
            block = block.split("\n")
            items = [int(n) for n in block[1].split(": ")[1].split(", ")]
            operation = operations[i]
            test = [int(line.split(" ")[-1]) for line in block[3:]]
            monkeys.append(Monkey(items, operation, test))

    return monkeys


def main() -> None:
    monkeys = read_monkeys()

    print("\n".join(str(m) for m in monkeys))
    print()

    inspection_count = [0] * len(monkeys)

    for i in range(20):
        if i % 10 == 0:
            print(f"round {i}")

        for i, monkey in enumerate(monkeys):
            for item in monkey.items:
                inspection_count[i] += 1
                item = monkey.operation(item) // 3

                divider = monkey.test[0]
                true_throw = monkey.test[1]
                false_throw = monkey.test[2]

                if item % divider == 0:
                    monkeys[true_throw].items.append(item)
                else:
                    monkeys[false_throw].items.append(item)

            monkey.items = []

    print("\n".join(str(m) for m in monkeys))
    print(f"{inspection_count=}")

    value1, value2 = sorted(inspection_count)[-2:]
    print(f"part1 answer: {value1 * value2}")


if __name__ == "__main__":
    main()
