# Input: 136760-595730

count = 0
for i in range(136760, 595730+1):
    increasing = True
    adjacent = False
    prev_digit = -1
    for digit in str(i):
        if prev_digit == digit:
            adjacent = True

        if int(digit) < int(prev_digit):
            increasing = False
            break

        prev_digit = digit

    if increasing and adjacent:
        count += 1

print(f'Part 1: {count}')

count = 0
for i in range(136760, 595730+1):
    increasing = True
    adjacent = False

    num = str(i)
    prev_prev_digit = -1
    prev_digit = -1
    for j in range(len(num)):
        digit = int(num[j])
        next_digit = int(num[j+1]) if j+1 < len(num) else 10
    
        if digit == prev_digit and digit != prev_prev_digit and digit != next_digit:
            adjacent = True

        if next_digit < digit:
            increasing = False
            break

        prev_prev_digit = prev_digit
        prev_digit = digit

    if increasing and adjacent:
        count += 1

print(f'Part 2: {count}')
