from day13py.main import (
    main, 
    parse_problem, 
    is_ordered, 
    solve_first_part,
    solve_second_part,
    prepare_for_second_part,
    order_packets,
)
import pytest


FIRST_EXAMPLE = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
    """


def test_parse_example():
    res = parse_problem(FIRST_EXAMPLE)
    expected = [
        (
            [1,1,3,1,1],
            [1,1,5,1,1]
        ), (
            [[1],[2,3,4]],
            [[1],4]
        ), (
            [9],
            [[8,7,6]]
        ), (
            [[4,4],4,4],
            [[4,4],4,4,4]
        ), (
            [7,7,7,7],
            [7,7,7]
        ), (
            [],
            [3]
        ), (
            [[[]]],
            [[]]
        ), (
            [1,[2,[3,[4,[5,6,7]]]],8,9],
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        )
    ]
    assert res == expected


@pytest.mark.parametrize("left, right, expected", [
    ([1], [2], True), 
    ([2], [1], False), 
    ([2], [[4]], True),
    ([2, 3, 4], [4], True),
    ([1,1,3,1,1], [1,1,5,1,1], True),
    ([[1],[2,3,4]], [[1],4], True),
    ([9], [[8,7,6]], False),
    ([[4,4],4,4], [[4,4],4,4,4], True),
    ([7,7,7,7], [7,7,7], False),
    ([], [3], True),
    ([[[]]], [[]], False),
    ([1,[2,[3,[4,[5,6,7]]]],8,9], [1,[2,[3,[4,[5,6,0]]]],8,9], False),
    ([1, [2, [3, [4, [5, 6, 0]]]], 8, 9], [[1], 4], True),
    ([[1], [2, 3, 4]], [[1], 4], True),
])
def test_is_ordered(left, right, expected): 
    assert is_ordered(left, right) is expected


def test_solve_first_part():
    first_solution = solve_first_part(FIRST_EXAMPLE)
    assert first_solution == 13


def test_order_packets():
    flat = prepare_for_second_part(FIRST_EXAMPLE)
    ordered = order_packets(flat)
    assert ordered[5] == [[1], 4]
    assert ordered[9] == [[2]]
    assert ordered[12] == [[4, 4], 4, 4, 4]
    assert ordered[13] == [[6]]


def test_solve_second_part():
    second_solution = solve_second_part(FIRST_EXAMPLE)
    assert second_solution == 140
