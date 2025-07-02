use std::{io, char};
use crate::lexer::Tokens;

pub fn parse(tokens: &Vec<Tokens>) -> io::Result<()> {
    let mut stack: [u32; 30000] = [0; 30000];
    let mut ptr: usize = 0;
    let mut program_counter = 0;

    while program_counter < tokens.len() {
        match tokens.get(program_counter) {
            Some(token) => {
                match token {
                    Tokens::Left => {
                        if ptr == 0 {
                            panic!("Pointer out of array bounds");
                        }

                        ptr -= 1;
                        program_counter += 1;
                    },
                    Tokens::Right => {
                        if ptr == 29999 {
                            panic!("Pointer out of array bounds");
                        }

                        ptr += 1;
                        program_counter += 1;
                    },
                    Tokens::Plus => {
                        stack[ptr] = stack[ptr].wrapping_add(1);
                        program_counter += 1;
                    },
                    Tokens::Minus => {
                        stack[ptr] = stack[ptr].wrapping_sub(1);
                        program_counter += 1;
                    },
                    Tokens::Write => {
                        if let Some(value) = char::from_u32(stack[ptr]) {
                            print!("{}", value);
                        }

                        program_counter += 1;
                    },
                    Tokens::Read => {
                        let mut buffer: String = String::new();
                        io::stdin().read_line(&mut buffer)?;

                        if let Some(ch) = buffer.chars().next() {
                            stack[ptr] = ch as u32;
                        }

                        program_counter += 1;
                    },
                    Tokens::LParen => {
                        if stack[ptr] == 0 {
                            let mut balance = 1;
                            while balance > 0 {
                                program_counter += 1;
                                if program_counter >= tokens.len() {
                                    panic!("Unmatched '['");
                                }

                                match &tokens[program_counter] {
                                    Tokens::LParen => balance += 1,
                                    Tokens::RParen => balance -= 1,
                                    _ => {}
                                }
                            }
                        } else {
                            program_counter += 1;
                        }
                    },
                    Tokens::RParen => {
                        if stack[ptr] != 0 {
                            let mut balance = 1;
                            while balance > 0 {
                                if program_counter == 0 {
                                    panic!("Unmatched ']'");
                                }

                                program_counter -= 1;

                                match &tokens[program_counter] {
                                    Tokens::LParen => balance -= 1,
                                    Tokens::RParen => balance += 1,
                                    _ => {}
                                }
                            }
                        } else {
                            program_counter += 1;
                        }
                    }
                }
            },
            None => ()
        }
    }

    Ok(())
}