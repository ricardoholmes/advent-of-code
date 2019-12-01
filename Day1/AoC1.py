with open("input1.txt") as file:
    f = (file.read()).split("\n")
    total = 0
    for i in f:
        tempFuel = int(i)
        while ((tempFuel//3)-2) > 0:
            tempFuel = ((tempFuel//3)-2)
            total += tempFuel
print(total)