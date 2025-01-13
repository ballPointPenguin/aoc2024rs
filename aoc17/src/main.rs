fn main() {
    let input = include_str!("../17-input.txt");
    let (registers, program) = parse_input(input);
    let mut computer = Computer::new(registers, program);

    computer.run();

    let result = computer.print();

    println!("{}", result);
}

#[derive(Debug)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

impl Registers {
    fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<u8>,
}

impl Program {
    fn new(instructions: Vec<u8>) -> Self {
        Self { instructions }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv = 0, // Division instruction for A
    Bxl = 1, // XOR with literal for B
    Bst = 2, // Set B from combo operand
    Jnz = 3, // Jump if A is not zero
    Bxc = 4, // XOR B and C registers
    Out = 5, // Output combo operand value
    Bdv = 6, // Division instruction for B
    Cdv = 7, // Division instruction for C
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

struct Computer {
    registers: Registers,
    program: Program,
    pointer: usize,
    output: Vec<u32>,
}

impl Computer {
    fn new(registers: Registers, program: Program) -> Self {
        Self {
            registers,
            program,
            pointer: 0,
            output: Vec::new(),
        }
    }

    fn get_combo_value(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn step(&mut self) -> bool {
        if self.pointer >= self.program.instructions.len() {
            return false;
        }

        let opcode =
            Opcode::try_from(self.program.instructions[self.pointer]).expect("Valid opcode");
        let operand = self.program.instructions[self.pointer + 1];

        self.execute(opcode, operand);

        // Advance instruction pointer (except for JNZ when A != 0)
        if !(matches!(opcode, Opcode::Jnz) && self.registers.a != 0) {
            self.pointer += 2;
        }

        true
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn execute(&mut self, opcode: Opcode, operand: u8) {
        match opcode {
            Opcode::Adv => {
                let num = self.registers.a;
                let combo = self.get_combo_value(operand);
                let den = 2_u32.pow(combo);
                self.registers.a = num / den;
            }
            Opcode::Bxl => {
                self.registers.b = self.registers.b ^ operand as u32;
            }
            Opcode::Bst => {
                let combo = self.get_combo_value(operand);
                self.registers.b = combo % 8;
            }
            Opcode::Jnz => {
                if self.registers.a != 0 {
                    self.pointer = operand as usize;
                }
            }
            Opcode::Bxc => {
                self.registers.b = self.registers.b ^ self.registers.c;
            }
            Opcode::Out => {
                let combo = self.get_combo_value(operand);
                self.output.push(combo % 8);
            }
            Opcode::Bdv => {
                let num = self.registers.a;
                let combo = self.get_combo_value(operand);
                let den = 2_u32.pow(combo);
                self.registers.b = num / den;
            }
            Opcode::Cdv => {
                let num = self.registers.a;
                let combo = self.get_combo_value(operand);
                let den = 2_u32.pow(combo);
                self.registers.c = num / den;
            }
        }
    }

    fn print(&self) -> String {
        self.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_input(input: &str) -> (Registers, Program) {
    let mut lines = input.lines();

    // Parse registers using iterator methods
    let [a, b, c] = lines
        .by_ref()
        .take(3)
        .map(|line| line.split_once(": ").unwrap().1.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // Skip the empty line
    lines.next();

    // Parse program
    let program = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    (Registers::new(a, b, c), Program::new(program))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    // New helper functions
    fn comp(a: u32, b: u32, c: u32, program: Vec<u8>) -> Computer {
        Computer::new(Registers::new(a, b, c), Program::new(program))
    }

    fn assert_registers(computer: &Computer, a: u32, b: u32, c: u32) {
        assert_eq!(
            (
                computer.registers.a,
                computer.registers.b,
                computer.registers.c
            ),
            (a, b, c),
            "Expected registers (a:{}, b:{}, c:{}), got (a:{}, b:{}, c:{})",
            a,
            b,
            c,
            computer.registers.a,
            computer.registers.b,
            computer.registers.c
        );
    }

    #[test]
    fn test_parse_input() {
        let (registers, program) = parse_input(INPUT);
        assert_eq!(registers.a, 729);
        assert_eq!(registers.b, 0);
        assert_eq!(registers.c, 0);
        assert_eq!(program.instructions, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_adv() {
        let mut computer = comp(100, 0, 0, vec![0, 1]);
        computer.run();
        assert_registers(&computer, 50, 0, 0); // 100 / 2^1 = 50

        let mut computer = comp(100, 0, 0, vec![0, 2]);
        computer.run();
        assert_registers(&computer, 25, 0, 0); // 100 / 2^2 = 25

        let mut computer = comp(100, 0, 0, vec![0, 3]);
        computer.run();
        assert_registers(&computer, 12, 0, 0); // 100 / 2^3 = 12

        let mut computer = comp(100, 0, 4, vec![0, 6]);
        computer.run();
        assert_registers(&computer, 6, 0, 4); // 100 / 2^4 = 6
    }

    #[test]
    fn test_bxl() {
        let mut computer = comp(0, 5, 0, vec![1, 3]);
        computer.run();
        assert_registers(&computer, 0, 6, 0); // 0101 ^ 0011 = 0110 (6)

        let mut computer = comp(0, 0, 0, vec![1, 3]);
        computer.run();
        assert_registers(&computer, 0, 3, 0); // 0000 ^ 0011 = 0011 (3)

        let mut computer = comp(0, 29, 0, vec![1, 7]);
        computer.run();
        assert_registers(&computer, 0, 26, 0); // 11101 ^ 00111 = 11
    }

    #[test]
    fn test_bst() {
        let mut computer = comp(0, 0, 0, vec![2, 2]);
        computer.run();
        assert_registers(&computer, 0, 2, 0); // 2 % 8 = 2

        let mut computer = comp(10, 0, 0, vec![2, 4]);
        computer.run();
        assert_registers(&computer, 10, 2, 0); // 10 % 8 = 2

        let mut computer = comp(0, 0, 27, vec![2, 6]);
        computer.run();
        assert_registers(&computer, 0, 3, 27); // 27 % 8 = 3

        let mut computer = comp(0, 0, 9, vec![2, 6]);
        computer.run();
        assert_registers(&computer, 0, 1, 9); // 27 % 8 = 3
    }

    #[test]
    fn test_jnz() {
        let mut computer = comp(0, 0, 0, vec![3, 4, 2, 2, 1, 3]);
        computer.run();
        // JNZ: noop; BST: set B to 2; BXL: set B to 2 ^ 3 = 1
        assert_registers(&computer, 0, 1, 0);

        let mut computer = comp(2, 0, 0, vec![3, 4, 2, 2, 1, 3]);
        computer.run();
        // JNZ: jump to BXL(4); BXL: set B to 0 ^ 3 = 3
        assert_registers(&computer, 2, 3, 0);

        let mut computer = comp(2, 0, 0, vec![3, 2, 2, 2, 1, 3]);
        computer.run();
        // JNZ: jump to BST(2); BST: set B to 2; BXL: set B to 2 ^ 3 = 1
        assert_registers(&computer, 2, 1, 0);
    }

    #[test]
    fn test_bxc() {
        let mut computer = comp(0, 0, 0, vec![4, 2]);
        computer.run();
        assert_registers(&computer, 0, 0, 0); // 0000 ^ 0000 = 0000 (0)

        let mut computer = comp(0, 8, 0, vec![4, 2]);
        computer.run();
        assert_registers(&computer, 0, 8, 0); // 1000 ^ 0000 = 1000 (8)

        let mut computer = comp(0, 0, 8, vec![4, 2]);
        computer.run();
        assert_registers(&computer, 0, 8, 8); // 0000 ^ 1000 = 1000 (8)

        let mut computer = comp(0, 8, 8, vec![4, 2]);
        computer.run();
        assert_registers(&computer, 0, 0, 8); // 1000 ^ 1000 = 0000 (0)

        let mut computer = comp(0, 2024, 43690, vec![4, 0]);
        computer.run();
        assert_registers(&computer, 0, 44354, 43690);
    }

    #[test]
    fn test_out() {
        let mut computer = comp(0, 0, 0, vec![5, 2]);
        computer.run();
        assert_eq!(computer.output, vec![2]);

        let mut computer = comp(13, 0, 0, vec![5, 2, 5, 4]);
        computer.run();
        assert_eq!(computer.output, vec![2, 5]); // 2 % 8 = 2; 13 % 8 = 5

        let mut computer = comp(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        computer.run();
        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_bdv() {
        let mut computer = comp(100, 0, 0, vec![6, 2]);
        computer.run();
        assert_registers(&computer, 100, 25, 0); // 100 / 2^2 = 25
    }

    #[test]
    fn test_cdv() {
        let mut computer = comp(100, 0, 0, vec![7, 2]);
        computer.run();
        assert_registers(&computer, 100, 0, 25); // 100 / 2^2 = 25
    }

    #[test]
    fn test_combinations() {
        let mut computer = comp(2024, 0, 0, vec![0, 1, 5, 4]);
        computer.run();
        assert_eq!(computer.registers.a, 1012);
        assert_eq!(computer.output, vec![4]);

        let mut computer = comp(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run();
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.registers.a, 0);

        let (registers, program) = parse_input(INPUT);
        let mut computer = Computer::new(registers, program);
        computer.run();
        assert_eq!(computer.print(), "4,6,3,5,6,3,5,2,1,0");
    }
}
