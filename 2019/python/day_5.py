def run_intcode(memory):
    index = 0
    while memory[index] != 99:
        instruction_str = str(memory[index]).zfill(5)
        instruction = int(instruction_str[3:])
        parameter_modes = instruction_str[:3]

        # 3 parameter instructions
        if instruction in [1, 2, 7, 8]:
            # parameter modes
            if int(parameter_modes[2]) == 1:
                param_one = memory[index+1]  # immediate
            else:
                param_one = memory[memory[index+1]] # position

            if int(parameter_modes[1]) == 1:
                param_two = memory[index+2] # immediate
            else:
                param_two = memory[memory[index+2]] # position

            # calculate value
            value = 0
            if instruction == 1:
                value = param_one + param_two

            elif instruction == 2:
                value = param_one * param_two
            
            elif instruction == 7:
                value = 1 if param_one < param_two else 0

            elif instruction == 8:
                value = 1 if param_one == param_two else 0

            # store value
            memory[memory[index+3]] = value # never immediate so issok

            if memory[index+3] != index:
                index += 4

        # jumps - 2 parameters
        elif instruction == 5 or instruction == 6:
            if int(parameter_modes[2]) == 1:
                param_one = memory[index+1]
            else:
                param_one = memory[memory[index+1]]

            if int(parameter_modes[1]) == 1:
                param_two = memory[index+2]
            else:
                param_two = memory[memory[index+2]]

            jump = False
            if instruction == 5:
                jump = param_one != 0
            elif instruction == 6:
                jump = param_one == 0
            
            if jump:
                index = param_two
            else:
                index += 3

        # 1 parameter instructions
        elif instruction == 3 or instruction == 4:
            if instruction == 3:
                memory[memory[index+1]] = int(input('Input: '))
                if memory[index+1] == index:
                    continue

            elif instruction == 4:
                if int(parameter_modes[2]) == 1:
                    print(memory[index+1])
                else:
                    print(memory[memory[index+1]])

            index += 2
        
        else:
            raise ValueError(f'Invalid instruction {instruction_str}')


with open('./puzzle_inputs/Day5', 'r') as f:
    inp = list(map(int, f.readline().split(',')))

print('Part 1 running... Set input as 1')
run_intcode(list(inp))

print('\nPart 2 running... Set input as 5')
run_intcode(list(inp))
