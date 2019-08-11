import random
from itertools import product
from typing import NamedTuple, List, FrozenSet, Tuple

KEY_SPACE_LOWER = -1
KEY_SPACE_UPPER = 1
KEY_SPACE_WIDTH = KEY_SPACE_UPPER - KEY_SPACE_LOWER


class Node(NamedTuple):
    index: int
    key_space: "KeySpace"
    # We store the index of nodes rather than the nodes themselves. This fixes
    # issues with the `neighbours` nodes becoming out of date as we change their
    # neighbours.
    neighbours: FrozenSet[int] = frozenset()

    def with_neighbours(self, neighbours: FrozenSet[int]) -> "Node":
        return Node(self.index, self.key_space, neighbours)


class KeySpace(NamedTuple):
    position: Tuple[float]

    @classmethod
    def random(cls, key_space_dimensions: int) -> "KeySpace":
        return KeySpace(
            tuple(
                float(random.uniform(KEY_SPACE_LOWER, KEY_SPACE_UPPER))
                for _ in range(key_space_dimensions)
            )
        )

    def distance(self, other: "KeySpace", wrapped=True) -> float:
        assert len(self.position) == len(other.position)
        total = float(0)
        for a, b in zip(self.position, other.position):
            distance = abs(a - b)
            if wrapped:
                distance = min(
                    distance,
                    abs((a + KEY_SPACE_WIDTH) - b),
                    abs((a - KEY_SPACE_WIDTH) - b),
                )
            total += distance ** 2
        return total ** 0.5


class Args(NamedTuple):
    num_nodes: int
    key_space_dimensions: int
    max_neighbours: int
    num_tests: int

    @classmethod
    def create(cls, arg_lists) -> List["Args"]:
        return [
            Args(*args, num_tests=arg_lists.num_tests)
            for args in product(
                arg_lists.num_nodes,
                arg_lists.key_space_dimensions,
                arg_lists.max_neighbours,
            )
        ]

    def __str__(self) -> str:
        return ",".join(
            [
                f"n={self.num_nodes}",
                f"e={self.max_neighbours}",
                f"d={self.key_space_dimensions}",
            ]
        )