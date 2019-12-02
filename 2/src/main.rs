fn intcode_computer(mut program: [i32; 137]) -> i32 {
    for i in (0..program.len()).filter(|&x| x % 4 == 0) {
        let opcode = program[i];
        let parameter_one = program[program[i + 1] as usize];
        let parameter_two = program[program[i + 2] as usize];
        let address_three = program[i + 3] as usize;

        if opcode == 1 {
            program[address_three] = parameter_one + parameter_two;
        } else if opcode == 2 {
            program[address_three] = parameter_one * parameter_two;
        } else {
            break;
        }
    }

    return program[0];
}

fn find_input(mut program: [i32; 137]) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;

            let element_zero = intcode_computer(program);

            if element_zero == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    return 0;
}

fn main() {
    let program: [i32; 137] = [1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 6, 23, 1, 23, 6, 27, 1, 13, 27, 31, 2, 13, 31, 35, 1, 5, 35, 39, 2, 39, 13, 43, 1, 10, 43, 47, 2, 13, 47, 51, 1, 6, 51, 55, 2, 55, 13, 59, 1, 59, 10, 63, 1, 63, 10, 67, 2, 10, 67, 71, 1, 6, 71, 75, 1, 10, 75, 79, 1, 79, 9, 83, 2, 83, 6, 87, 2, 87, 9, 91, 1, 5, 91, 95, 1, 6, 95, 99, 1, 99, 9, 103, 2, 10, 103, 107, 1, 107, 6, 111, 2, 9, 111, 115, 1, 5, 115, 119, 1, 10, 119, 123, 1, 2, 123, 127, 1, 127, 6, 0, 99, 2, 14, 0, 0];

    let answer = find_input(program);

    println!("Answer is: {:?}", answer);
}
