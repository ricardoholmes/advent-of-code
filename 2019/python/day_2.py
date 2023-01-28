def intcode(memory):
    index = 0
    instruction = 0
    while memory[index] != 99:
        instruction = memory[index]
        if instruction == 1:
            memory[memory[index+3]] = memory[memory[index+1]] + memory[memory[index+2]]
        elif instruction == 2:
            memory[memory[index+3]] = memory[memory[index+1]] * memory[memory[index+2]]
        
        index += 4
    
    return memory[0]

def part_one(inp):
    inp[1] = 12
    inp[2] = 2
    return intcode(inp)

def part_two(inp):
    for noun in range(100):
        for verb in range(100):
            inp_instance = list(inp)
            inp_instance[1] = noun
            inp_instance[2] = verb
            try:
                if intcode(inp_instance) == 19690720:
                    return (100 * noun) + verb
            except IndexError:
                continue

with open('./puzzle_inputs/Day2', 'r') as f:
    inp = list(map(int, f.readline().split(',')))

print(f'Part 1: {part_one(list(inp))}')
print(f'Part 2: {part_two(list(inp))}')
