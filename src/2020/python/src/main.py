import solutions


while True:
    day = input("Which day would you like to run (1-7): ")

    if day == "exit":
        break
    elif day == "1":
        solutions.day01()
    elif day == "2":
        solutions.day02()
    elif day == "3":
        solutions.day03()
    elif day == "4":
        solutions.day04()
    elif day == "5":
        solutions.day05()
    elif day == "6":
        solutions.day06()
    elif day == "7":
        solutions.day07()
    elif day == "8":
        solutions.day08()
    else:
        print("Invalid input")
