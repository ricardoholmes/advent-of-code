# checks if num is between boundary_one and boundary_two,
# regardless of which one is larger
def num_in_range(num, boundary_one, boundary_two):
    in_range = boundary_one <= num and num <= boundary_two
    in_range = in_range or (boundary_one >= num and num >= boundary_two)
    return in_range

# read and parse puzzle input
with open('./puzzle_inputs/Day3', 'r') as f:
    inp = f.read().split('\n')

wire_one = inp[0].split(',')
wire_two = inp[1].split(',')

# get all turning points of wire 1
wire_one_coords = [[0, 0]]
wire_one_lengths = [0] # total length of wire up to turning point
coord = [0, 0]
for i in wire_one:
    direction = i[0]
    magnitude = int(i[1:])
    if direction == 'U':
        coord[1] += magnitude
    elif direction == 'D':
        coord[1] -= magnitude
    elif direction == 'R':
        coord[0] += magnitude
    elif direction == 'L':
        coord[0] -= magnitude

    wire_one_coords.append(tuple(coord))
    wire_one_lengths.append((0 if len(wire_one_lengths) == 0 else wire_one_lengths[-1]) + magnitude)

intersections = []
intersections_coords = []
intersections_magnitudes = []
total_magnitude = 0
coord = [0, 0]
for i in wire_two:
    direction = i[0]
    magnitude = int(i[1:])

    # if moving on y axis
    if direction == 'U' or direction == 'D':
        if direction == 'D':
            relative_direction = -1
        else:
            relative_direction = 1

        prev_coord = list(coord)
        coord[1] += magnitude * relative_direction
        for index, value in enumerate(wire_one_coords[:-1]):
            if num_in_range(value[1], prev_coord[1], coord[1]) and num_in_range(prev_coord[0], value[0], wire_one_coords[index+1][0]):
                intersections.append(abs(coord[0]) + abs(value[1]))
                intersections_coords.append([coord[0], value[1]])

                steps_wire_one = wire_one_lengths[index] + abs(coord[0] - value[0])
                steps_wire_two = total_magnitude + abs(value[1] - prev_coord[1])
                intersections_magnitudes.append(steps_wire_one + steps_wire_two)

    # if moving on x axis
    elif direction == 'R' or direction == 'L':
        if direction == 'L':
            relative_direction = -1
        else:
            relative_direction = 1

        prev_coord = list(coord)
        coord[0] += magnitude * relative_direction
        for index, value in enumerate(wire_one_coords[:-1]):
            if num_in_range(value[0], prev_coord[0], coord[0]) and num_in_range(prev_coord[1], value[1], wire_one_coords[index+1][1]):
                intersections.append(abs(value[0]) + abs(coord[1]))
                intersections_coords.append([value[0], coord[1]])

                steps_wire_one = wire_one_lengths[index] + abs(coord[1] - value[1])
                steps_wire_two = total_magnitude + abs(value[0] - prev_coord[0])
                intersections_magnitudes.append(steps_wire_one + steps_wire_two)

    total_magnitude += magnitude


intersections.remove(0)
intersections_magnitudes.remove(0)

closest_intersection_dist = min(intersections)
shortest_intersection = min(intersections_magnitudes)

print(f'Part 1: {closest_intersection_dist}')
print(f'Part 2: {shortest_intersection}')
