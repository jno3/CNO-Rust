use crate::lexer::{Final, Table, Token, Word};
use std::{fs, io::Write};

const FILENAME: &str = "outt.s";
#[derive(Clone, Debug)]
pub enum Register {
    Info(String, bool),
}

fn load(registers: &mut Vec<Register>, token: &u32, registers_tracker: &mut Vec<usize>) -> String {
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

fn load_var(
    registers: &mut Vec<Register>,
    registers_tracker: &mut Vec<usize>,
    table: &mut Table,
    var_index: &usize,
) {
    let mut has = false;
    let mut ret = false;
    let mut var_register_index = 0;
    for (index, register) in registers.iter_mut().enumerate() {
        let Register::Info(register_name, register_free) = register;
        if *register_free {
            let var_name = table.symbols.get(*var_index).unwrap();
            let var_register = table.registers.get(index);

            match var_register {
                Some(actual_index) => {
                    has = true;
                    var_register_index = *actual_index;
                }
                None => (),
            }
            let content = format!("\tmovq\t{}(%rip), {}\n", var_name, register_name);
            write(&content);
            *register_free = false;
            registers_tracker.push(index);
            ret = true;
            break;
        }
    }
    if has {
        let Register::Info(_, register_free) = registers.get_mut(var_register_index).unwrap();
        *register_free = true;
    }
    if ret {
        // let var_status = table.status.get_mut(*var_index).unwrap();
        // *var_status = false;
        return;
    }

    panic!("out of registers!");
}

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
    println!("{:?}", registers_tracker);
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
    registers_tracker.pop();

    registers_tracker.push(r1_index);
}

fn comp(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>, how: &str) {
    let len = registers_tracker.len();
    let r1_index = *registers_tracker.get(len - 2).unwrap();
    let r2_index = *registers_tracker.get(len - 1).unwrap();
    let Register::Info(r2_name, _) = registers.get(r2_index).unwrap();
    let Register::Info(r1_name, _) = registers.get(r1_index).unwrap();

    let content = format!(
        "\tcmpq\t{}, {}\n\t{}\t{}b\n\tandq\t$255,{}\n",
        r2_name, r1_name, how, r2_name, r2_name
    );
    write(&content);

    let Register::Info(_, r1_free) = registers.get_mut(r1_index).unwrap();
    *r1_free = true;

    registers_tracker.pop();
    registers_tracker.pop();

    registers_tracker.push(r2_index);
}

fn equals(registers: &mut Vec<Register>, registers_tracker: &mut Vec<usize>, var_name: &String) {
    let len = registers_tracker.len();
    let r_index = *registers_tracker.get(len - 1).unwrap();

    let Register::Info(r_name, r_free) = registers.get_mut(r_index).unwrap();

    let content = format!("\tmovq\t{}, {}(%rip)\n", r_name, var_name);
    write(&content);

    *r_free = true;

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

fn int(var_name: &String) {
    let content = format!("\t.comm\t{},8,8\n", var_name);
    write(&content);
}

pub fn interpret(tokens: &Vec<&Token>, table: &mut Table) {
    fs::File::create(FILENAME).unwrap();
    write_preamble();
    let mut registers: Vec<Register> = Vec::new();
    let mut registers_tracker: Vec<usize> = Vec::new();
    let mut var_tracker: usize = 0;

    for i in 8..=11 {
        let register_name = format!("%r{}", i);
        let register = Register::Info(register_name, true);
        registers.push(register);
    }

    for (_index, token) in tokens.iter().enumerate() {
        if let Token::Value(..) = token {
            match token {
                Token::Value(Final::Integer(value)) => {
                    load(&mut registers, value, &mut registers_tracker);
                }
                Token::Value(Final::Symbol(var_index)) => {
                    let var_status = table.status.get(*var_index).unwrap();
                    if *var_status {
                        load_var(&mut registers, &mut registers_tracker, table, var_index);
                    } else {
                        var_tracker = *var_index;
                    }
                }
                _ => (),
            }
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
                }
                Token::Symbol(Word::EqualsEquals, _) => {
                    comp(&mut registers, &mut registers_tracker, "sete");
                }
                Token::Symbol(Word::DiffEquals, _) => {
                    comp(&mut registers, &mut registers_tracker, "setne");
                }
                Token::Symbol(Word::LowerThan, _) => {
                    comp(&mut registers, &mut registers_tracker, "setl");
                }
                Token::Symbol(Word::GreaterThan, _) => {
                    comp(&mut registers, &mut registers_tracker, "setg");
                }
                Token::Symbol(Word::LowerEquals, _) => {
                    comp(&mut registers, &mut registers_tracker, "setle");
                }
                Token::Symbol(Word::GreaterEquals, _) => {
                    comp(&mut registers, &mut registers_tracker, "setge");
                }

                Token::Symbol(Word::Equals, _) => {
                    let var_name = table.symbols.get(var_tracker).unwrap();
                    let var_status = table.status.get_mut(var_tracker).unwrap();
                    equals(&mut registers, &mut registers_tracker, var_name);
                    *var_status = true;
                    
                }
                Token::Symbol(Word::Int, _) => {
                    let var_name = table.symbols.get(var_tracker).unwrap();
                    int(var_name);
                }
                Token::Symbol(Word::Print, _) => {
                    print(&mut registers, &mut registers_tracker);
                }
                Token::Symbol(Word::Semicolon, _) => {}
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
