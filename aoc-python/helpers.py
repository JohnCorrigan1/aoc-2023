def get_lines(day):
    with open (f"../inputs/day{day}.txt") as input:
        lines = input.read().split("\n")
        lines.pop()
    return lines


def get_input(day):
    with open (f"../inputs/day{day}.txt") as input:
        data = input.read()
    return data 

