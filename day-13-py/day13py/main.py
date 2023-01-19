"""Main module for the project."""
import os
import ast
from itertools import zip_longest
from typing import Union, List, Tuple
import functools
import httpx
from dotenv import load_dotenv
from loguru import logger

PacketItem = Union[list, int]


def parse_problem(input_string: str) -> List[Tuple[List, List]]:
    logger.debug("Parsing")
    result = list()
    previous = None
    for line in input_string.split("\n"):
        clean_line = line.strip()
        if clean_line == "":
            continue
        parsed_line = ast.literal_eval(clean_line)
        if previous is None:
            previous = parsed_line
        else:
            result.append((
                previous,
                parsed_line,
            ))
            previous = None
    return result


def is_ordered(left: PacketItem, right: PacketItem) -> bool:
    # logger.debug(f"comparing: {left=} - {right=}")
    if isinstance(left, int) and isinstance(right, int):
        # logger.debug("comparing ints")
        return left <= right
    elif isinstance(left, list) and isinstance(right, list):
        for el_left, el_right in zip_longest(left, right):
            if el_left is None:
                return True
            elif el_right is None:
                return False
            elif el_left == el_right:
                continue
            else:
                return is_ordered(el_left, el_right)
        return True
    else:
        # logger.debug("converting to lists")
        if not isinstance(left, list):
            left = [left]
        if not isinstance(right, list):
            right = [right]
        return is_ordered(left, right)


def solve_first_part(input_string: str):
    logger.info("Solving first part")
    parsed = parse_problem(input_string)
    result = 0
    for i, pair in enumerate(parsed):
        left, right = pair
        if is_ordered(left, right):
            result += (i + 1)
    return result


def comparison_order(left, right) -> int:
    if left == right:
        return 0
    if is_ordered(left, right):
        return -1
    else:
        return 1


def compute_dividers(sorted_packets: List) -> int:
    logger.debug("Computing divider")
    first, second = [[2]], [[6]]
    first_idx, second_idx = None, None
    for i, value in enumerate(sorted_packets):
        if value == first:
            first_idx = i + 1
        if value == second:
            second_idx = i + 1
    if first_idx is None or second_idx is None:
        raise RuntimeError(f"Could not find dividers")
    logger.info(f"First index: {first_idx}")
    logger.info(f"Second index: {second_idx}")
    return first_idx * second_idx


def order_packets(list_of_packets: List) -> List:
    logger.debug(f"Ordering {len(list_of_packets)} packets")
    cmp_func = functools.cmp_to_key(comparison_order)
    list_of_packets.sort(key=cmp_func)
    from pprint import pprint
    pprint(list_of_packets)
    return list_of_packets


def prepare_for_second_part(input_string: str) -> List:
    parsed = parse_problem(input_string)
    flat_packets = list()
    for left, right in parsed:
        flat_packets.append(left)
        flat_packets.append(right)
    logger.debug("Inserting dividers")
    first_divider = [[2]]
    second_divider = [[6]]
    if first_divider not in flat_packets:
        logger.debug("Inserting first divider")
        flat_packets.append(first_divider)
    if second_divider not in flat_packets:
        logger.debug("Inserting second divider")
        flat_packets.append(second_divider)
    return flat_packets


def solve_second_part(input_string: str):
    logger.info("Solving second part")
    flat_packets = prepare_for_second_part(input_string)
    ordered_packets = order_packets(flat_packets)
    return compute_dividers(ordered_packets)


def get_cookie_value() -> str:
    load_dotenv("../.env")
    value = os.environ["ADVENT_COOKIE"]
    return value

def _build_problem_url(day: int) -> str:
    return f"https://adventofcode.com/2022/day/{day}/input"

def get_problem_input() -> str:
    logger.info("Getting problem input")
    response = httpx.get(
        _build_problem_url(13), 
        headers={
        "Cookie": get_cookie_value()
        }
    )
    return response.text

def main():
    """Define the main function for the project."""
    input_string = get_problem_input()
    first_solution = solve_first_part(input_string)
    logger.info(f"First solution is: {first_solution}")

    second_solution = solve_second_part(input_string)
    logger.info(f"Second solution is: {second_solution}")
    return "ok"

if __name__ == "__main__":
    main()
