# for wait in range(0, 8):
#   print(f"wait: {wait}--------")

#   for x in range(1, 8):
#     y = wait * max(x - wait, 0)
#     print(f"x: {x}, y: {y}")
# 138915

from math import sqrt, ceil, floor

def possible_dists(duration: int, target: int):
    return [wait * (duration - wait) for wait in range(1, duration)]


print(possible_dists(7, 9))
print(possible_dists(15, 40))
print(possible_dists(30, 200))


def min_max_winning_dists(duration: int, target: int) -> tuple[float, float]:
    target = target + 1 # trying to beat the target
    max_num = duration + sqrt((duration ** 2) - (4 * target))
    max = (max_num) / 2
    min_num = duration - sqrt((duration ** 2) - (4 * target))
    min = (min_num) / 2
    return ceil(min), floor(max)


print(min_max_winning_dists(7, 9))
print(min_max_winning_dists(15, 40))
print(min_max_winning_dists(30, 200))


def num_clears(duration: int, target: int):
    return [dist for dist in possible_dists(duration, target) if dist > target]


print(len(num_clears(7, 9)), num_clears(7, 9))
print(len(num_clears(15, 40)), num_clears(15, 40))
print(len(num_clears(30, 200)), num_clears(30, 200))
