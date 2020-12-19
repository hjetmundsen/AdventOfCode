def _run_instructions(instructions: list[str]) -> tuple[int, bool]:
    visited = set()
    acc, i = 0, 0
    signs = {"+": 1, "-": -1}

    while True:
        if i == len(instructions):
            return (acc, True)

        if i in visited or i > len(instructions):
            return (acc, False)

        visited.add(i)

        code, val = instructions[i].split(" ")

        if code == "nop":
            i += 1
        elif code == "acc":
            acc += int(val[1:]) * signs[val[0]]
            i += 1
        else:
            i += int(val[1:]) * signs[val[0]]


def _find_jmp_and_nop(instructions: list[str]) -> list[int]:
    return [
        i
        for i, instruction in enumerate(instructions)
        if instruction.split(" ")[0] in {"jmp", "nop"}
    ]


def _swap_codes(instructions: list[str], index: int) -> list[str]:
    code, val = instructions[index].split(" ")
    code = "jmp" if code == "nop" else "nop"
    return " ".join([code, val])


def _star1() -> int:
    instructions = [line.strip() for line in open("../../inputs/day8.txt", "r")]
    acc, _ = _run_instructions(instructions)
    return acc


def _star2() -> int:
    instructions = [line.strip() for line in open("../../inputs/day8.txt", "r")]
    jmp_nop_indices = _find_jmp_and_nop(instructions)

    print([instructions[i] for i in jmp_nop_indices])

    for index in jmp_nop_indices:
        instructions[index] = _swap_codes(instructions, index)
        acc, reached_end = _run_instructions(instructions)

        if reached_end:
            return acc

        instructions[index] = _swap_codes(instructions, index)


def day08():
    print("DAY 8\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
