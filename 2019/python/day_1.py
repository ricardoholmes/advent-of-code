with open('./puzzle_inputs/Day1', 'r') as f:
    inp = filter(None, f.read().split('\n'))

part_one_sum = 0
part_two_sum = 0
for i in inp:
    fuel_req = (int(i) // 3) - 2
    part_one_sum += fuel_req

    part_two_sum += fuel_req
    while fuel_req // 3 > 2:
        fuel_req = (fuel_req // 3) - 2
        part_two_sum += fuel_req

print(f'Part 1: {part_one_sum}')
print(f'Part 2: {part_two_sum}')
