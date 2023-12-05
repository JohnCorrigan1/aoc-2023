from helpers import get_lines

# Path: day1.py
input = get_lines(1)

word_nums = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9"
}

def q1():
    total = 0
    for line in input:
        nums = [] 
        for char in line:
            if char.isdigit():
                nums.append(char)
        total += int(nums[0] + nums[-1])
    return total


def q2():
    total = 0
    for line in input:
        nums = []
        new_line = line 
        for word in word_nums:
            new_line = new_line.replace(word, word_nums[word])
        for char in new_line:
            if char.isdigit():
                nums.append(char)
        total += int(nums[0] + nums[-1])
    return total

print("Day 1")
print("Part 1:", q1())
print("Part 2:", q2())


