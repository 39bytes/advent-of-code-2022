import os
import ast
from functools import cmp_to_key

def compare_int(a, b):
    if a < b:
        return 1
    elif a > b:
        return -1
    else:
        return 0

def compare_packets(left, right):
    for a, b in zip(left, right):
        if isinstance(a, list) and isinstance(b, list):
            cmp = compare_packets(a, b)
            if compare_packets(a, b) != 0:
                return cmp
        elif isinstance(a, int) and isinstance(b, list):
            a = [a]
            cmp = compare_packets(a, b)
            if cmp != 0:
                return cmp
        elif isinstance(a, list) and isinstance(b, int):
            b = [b]
            cmp = compare_packets(a, b)
            if cmp != 0:
                return cmp
        elif isinstance(a, int) and isinstance(b, int):
            cmp = compare_int(a, b)
            if cmp != 0:
                return cmp
    if len(left) < len(right):
        return 1
    elif len(left) > len(right):
        return -1
    return 0

def part1(packets):
    i = 1
    index_sum = 0
    while packets:
        left = packets.pop(0)
        right = packets.pop(0)
        if compare_packets(left, right) == 1:
            index_sum += i
        i += 1
    return index_sum

def part2(packets):
    packets.append([[2]])
    packets.append([[6]])
    sorted_packets = sorted(packets, key=cmp_to_key(compare_packets), reverse=True)
    with open("output.txt", "w") as f:
        for packet in sorted_packets:
            f.write(str(packet) + "\n")
    
    first = sorted_packets.index([[2]]) + 1
    second = sorted_packets.index([[6]]) + 1

    return first * second

with open(os.path.join(os.path.dirname(__file__), "..\\..\\inputs\\day13.txt")) as f:
    data = f.read()
    lines = data.splitlines()
    packets = []
    for line in lines:
        if line:
            packets.append(ast.literal_eval(line))
    
    print("Solution to part 1: ", part1(packets[:]))
    print("Solution to part 2: ", part2(packets[:]))


