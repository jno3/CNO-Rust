use crate::lexer::{Final, Token, Word};
use std::{fs, io::Write};

const FILENAME: &str = "outt.s";
#[derive(Clone, Debug)]
pub enum Register {
    Info(String, bool),
}

fn load(registers: &mut Vec<Register>, token: &i32, registers_tracker: &mut Vec<usize>) -> String {
    for (index, register) in registers.iter_mut().enumerate() {
        let Register::Info(register_name, register_free) = register;
        if *register_free {
            let content = format!("\tmovq\t${}, {}\n", token, register_name);
            write(&content);
            *register_free = false;
            registers_tracker.push(index);
            return register_name.clone();
        }
    }
    panic!("out of registers!");
}

// fn op_2_helper(registers: &mut Vec<Register>) -> (usize, usize) {
//     let mut i = 0;
//     let mut registers_helper: Vec<usize> = Vec::new();
//     for (index, register) in registers.iter().enumerate().rev() {
//         let Register::Info(_, free) = register;
//         if !free {
//             registers_helper.push(index);
//             i += 1;
//         }
//         if i == 2 {
//             let r1_index = registers_helper.get(1).unwrap();
//             let r2_index = registers_helper.get(0).unwrap();
//             return (*r1_index, *r2_index);
//         }
//     }
//     panic!("err")
// }

// fn op_1_helper(registers: &mut Vec<Register>) -> usize {
//     let mut i = 0;
//     let mut registers_helper: Vec<usize> = Vec::new();
//     for (index, register) in registers.iter().enumerate().rev() {
//         let Register::Info(_, free) = register;
//         if !free {
//             registers_helper.push(index);
//             i += 1;
//         }
//         if i == 1 {
//             let r_index = registers_helper.get(0).unwrap();
//             return *r_index;
//         }
//     }
//     panic!("err")
// }

fn add(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>) {
    // let (r1_index, r2_index) = op_2_helper(registers);

    let len = registers_tracker.len();
    let r1_index = *registers_tracker.get(len - 2).unwrap();
    let r2_index = *registers_tracker.get(len - 1).unwrap();
    let Register::Info(r2_name, _) = registers.get(r2_index).unwrap();
    let Register::Info(r1_name, _) = registers.get(r1_index).unwrap();

    let content = format!("\taddq\t{}, {}\n", r1_name, r2_name);
    write(&content);
    let Register::Info(_, r1_free) = registers.get_mut(r1_index).unwrap();
    *r1_free = true;

    registers_tracker.pop();
    registers_tracker.pop();

    registers_tracker.push(r2_index);
}

fn sub(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>) {
    // let (r1_index, r2_index) = op_2_helper(registers);
    let len = registers_tracker.len();
    let r1_index = *registers_tracker.get(len - 2).unwrap();
    let r2_index = *registers_tracker.get(len - 1).unwrap();
    let Register::Info(r2_name, _) = registers.get(r2_index).unwrap();
    let Register::Info(r1_name, _) = registers.get(r1_index).unwrap();

    let content = format!("\tsubq\t{}, {}\n", r2_name, r1_name);
    write(&content);
    let Register::Info(_, r2_free) = registers.get_mut(r2_index).unwrap();
    *r2_free = true;

    registers_tracker.pop();
}

fn mul(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>) {
    // let (r1_index, r2_index) = op_2_helper(registers);
    let len = registers_tracker.len();
    let r1_index = *registers_tracker.get(len - 2).unwrap();
    let r2_index = *registers_tracker.get(len - 1).unwrap();
    let Register::Info(r2_name, _) = registers.get(r2_index).unwrap();
    let Register::Info(r1_name, _) = registers.get(r1_index).unwrap();


    let content = format!("\timulq\t{}, {}\n", r1_name, r2_name);
    write(&content);
    let Register::Info(_, r1_free) = registers.get_mut(r1_index).unwrap();
    *r1_free = true;
    
    registers_tracker.pop();
    registers_tracker.pop();

    registers_tracker.push(r2_index);
}

fn div(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>) {
    // let (r1_index, r2_index) = op_2_helper(registers);
    let len = registers_tracker.len();
    let r1_index = *registers_tracker.get(len - 2).unwrap();
    let r2_index = *registers_tracker.get(len - 1).unwrap();
    let Register::Info(r2_name, _) = registers.get(r2_index).unwrap();
    let Register::Info(r1_name, _) = registers.get(r1_index).unwrap();

    let content = format!(
        "\tmovq\t{}, %rax\n\tcqo\n\tidivq\t{}\n\tmovq\t%rax, {}\n",
        r1_name, r2_name, r1_name
    );
    write(&content);

    let Register::Info(_, r2_free) = registers.get_mut(r2_index).unwrap();
    *r2_free = true;

    registers_tracker.pop();
}

fn print(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>) {
    let len = registers_tracker.len();
    let r_index = *registers_tracker.get(len - 1).unwrap();

    let Register::Info(r_name, r_free) = registers.get_mut(r_index).unwrap();

    let content = format!("\tmovq\t{}, %rdi\n\tcall\tprintint\n", r_name);
    write(&content);

    *r_free = true;

    registers_tracker.pop();
}

pub fn interpret(tokens: &Vec<&Token>) {
    fs::File::create(FILENAME).unwrap();
    write_preamble();
    let mut registers: Vec<Register> = Vec::new();
    let mut registers_tracker: Vec<usize> = Vec::new();

    for i in 8..=11 {
        let register_name = format!("%r{}", i);
        let register = Register::Info(register_name, true);
        registers.push(register);
    }

    for token in tokens {
        if let Token::Value(Final::Integer(value)) = token {
            load(&mut registers, value, &mut registers_tracker);
        } else {
            match token {
                Token::Symbol(Word::Add, _) => {
                    add(&mut registers, &mut registers_tracker);
                }
                Token::Symbol(Word::Sub, _) => {
                    sub(&mut registers, &mut registers_tracker);
                }
                Token::Symbol(Word::Mul, _) => {
                    mul(&mut registers, &mut registers_tracker);
                }
                Token::Symbol(Word::Div, _) => {
                    div(&mut registers, &mut registers_tracker);
                },
                Token::Symbol(Word::Print, _) => {
                    print(&mut registers, &mut registers_tracker);
                }
                _ => (),
            }
        }
    }

    write_postamble();
}

fn write(content: &str) {
    fs::OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .unwrap()
        .write(content.as_bytes())
        .unwrap();
}

fn write_preamble() {
    let preamble = "\t.text
.LC0:
\t.string\t \"%d\\n\"
printint:
\tpushq\t%rbp
\tmovq\t%rsp, %rbp
\tsubq\t$16, %rsp
\tmovl\t%edi, -4(%rbp)
\tmovl\t-4(%rbp), %eax
\tmovl\t%eax, %esi
\tleaq	.LC0(%rip), %rdi
\tmovl	$0, %eax
\tcall	printf@PLT
\tnop
\tleave
\tret

\t.globl\tmain
\t.type\tmain, @function
main:
\tpushq\t%rbp
\tmovq	%rsp, %rbp


";
    write(preamble);
}

fn write_postamble() {
    let postamble = "
\tmovl\t$0, %eax
\tpopq\t%rbp
\tret\n";
    write(postamble);
}
