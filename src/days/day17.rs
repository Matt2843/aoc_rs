use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Adv(usize),     
    Bxl(usize),
    Bst(usize),
    Jnz(usize),     
    Bxc,    
    Out(usize), 
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn from_usize(i: usize, operand: usize) -> Self {
        match i {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => unreachable!()
        }
    }

    fn combo_val(combo_op: usize, registers: &[usize]) -> usize {
        match combo_op {
            0..=3 => combo_op,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => unreachable!()
        }
    }
    fn eval(self: &Instruction, registers: &mut [usize]) -> Option<usize> {
        match self {
            // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 
            // to the power of the instruction's combo operand. The result of the division operation is truncated to an integer and then written to the A register.
            Instruction::Adv(combo) => {
                let cv = Self::combo_val(*combo, registers);
                registers[0] >>= cv;
                None
            }
            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B
            Instruction::Bxl(literal) => { 
                registers[1] ^= literal;
                None
            } 
            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            Instruction::Bst(combo) => {
                let cv = Self::combo_val(*combo, registers);
                registers[1] = cv % 8;
                None
            }
            // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of 
            // its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            Instruction::Jnz(literal) => {
                if registers[0] == 0 {
                    None
                } else {
                    Some(*literal)
                }
            }
            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. 
            // (For legacy reasons, this instruction reads an operand but ignores it.)
            Instruction::Bxc => {
                registers[1] ^= registers[2];
                None
            }
            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
            Instruction::Out(combo) => {
                let cv = Self::combo_val(*combo, registers);
                Some(cv % 8)
            }
            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
            Instruction::Bdv(combo) => {
                let cv = Self::combo_val(*combo, registers);
                registers[1] = registers[0] >> cv;
                None
            }
            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
            Instruction::Cdv(combo) => {
                let cv = Self::combo_val(*combo, registers);
                registers[2] = registers[0] >> cv;
                None
            }
        }
    }
}

fn run(registers: &mut [usize], program: &[usize]) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut outs = vec![];
    loop {
        if instruction_pointer >= program.len() {
            break;
        }
        let instruction = Instruction::from_usize(program[instruction_pointer], program[instruction_pointer+1]);
        if let Some(val) = instruction.eval(registers) {
            match instruction {
                Instruction::Jnz(_) => {instruction_pointer = val; continue;},
                Instruction::Out(_) => outs.push(val),
                _ => unreachable!()
            }
        }     
        instruction_pointer += 2;
    }
    outs
}

// My program: 2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0
// b = a % 8
// b ^= 1
// c = a >> b
// b ^= 5
// b ^= c
// out (b % 8)
// a >== 3
// jump 0 if a != 0
fn brute(program: &[usize], res: usize) -> Option<usize> {
    if let Some(&last) = program.last() {
        (0..8).find_map(|t| {
            let a = (res << 3) + t;
            let mut b = a % 8;
            b ^= 1;
            let c = a >> b;
            b ^= 5;
            b ^= c;
            if b % 8 == last {
                brute(&program[..program.len() - 1], a)
            } else {
                None
            }
        })
    } else {
        Some(res)
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let (registers, program) = input.trim().split_once("\n\n").unwrap();
    let mut registers = registers
        .lines()
        .flat_map(|l| l.split_once(": ").unwrap().1.split(',').flat_map(|n| n.parse::<usize>()))
        .collect_vec();
    let program = program.split_once(": ").unwrap()
        .1.trim().split(',').flat_map(|n| n.parse::<usize>())
        .collect_vec();

    let part1 = run(&mut registers, &program).iter().join(",");
    println!("{part1}");
    let part2 = brute(&program, 0).unwrap();
    assert_eq!(program, run(&mut [part2,0,0], &program));
    (part1.split(',').join("").parse().unwrap(), part2)
}
