import solutions

solution_map = {
    "1": solutions.day01,
    "2": solutions.day02,
    "3": solutions.day03,
    "4": solutions.day04,
    "5": solutions.day05,
    "6": solutions.day06,
    "7": solutions.day07,
    "8": solutions.day08,
}

while True:
    day = input("Which day would you like to run (1-8): ")
    print()

    if day == "exit":
        break
    elif day == "all":
        for solution in solution_map.values():
            solution()
    elif day in solution_map:
        solution_map[day]()
    else:
        print("Invalid input")
