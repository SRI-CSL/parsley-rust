// Copyright (c) 2019-2021 SRI International.
// All rights reserved.
//
//    This file is part of the Parsley parser.
//
//    Parsley is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    Parsley is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::iccmax_lib::iccmax_prim::{MPetOptions, Operations};

type IccError = String;
type IccResult<T> = std::result::Result<T, IccError>;

#[derive(Debug, Clone)]
pub struct ExecutionTree {
    stack:        u16,
    max_stack:    u16,
    paths:        Option<Vec<ExecutionTree>>,
    instructions: Vec<Operations>,
    completed:    bool,
    pos_array:    Vec<MPetOptions>,
}

impl ExecutionTree {
    pub fn new(
        stack: u16, max_stack: u16, paths: Option<Vec<ExecutionTree>>,
        instructions: Vec<Operations>, completed: bool, pos_array: Vec<MPetOptions>,
    ) -> Self {
        Self {
            stack,
            max_stack,
            paths,
            instructions,
            completed,
            pos_array,
        }
    }

    pub fn execute(&mut self) -> IccResult<()> {
        /*
         * 1. For every instruction, call compute_stack_effects()
         *      a. If conditions and Sel statements lead to branches
         *      b. Copy over all instructions to both children, along with
         * the correct branches      c. Call function recursively on
         * each path 2. Set completed to true
         */
        let mut counter = 0;
        while counter < self.instructions.len() {
            let signature = self.instructions[counter].clone().signature();
            let function_operations = self.instructions[counter].clone().function_operations();
            let data_list = self.instructions[counter].clone().data().data_list();
            match signature.as_str() {
                "if" | "sel" => {
                    let mut paths: Vec<ExecutionTree> = vec![];
                    for c in 0 .. function_operations.len() {
                        let mut part: Vec<Operations> = self.instructions[counter + 1 ..].to_vec();
                        let mut tmp_func = function_operations[c].clone();
                        tmp_func.append(&mut part);
                        let pos_array_copy = self.pos_array.clone();
                        let e = ExecutionTree::new(
                            self.stack,
                            self.max_stack,
                            None,
                            tmp_func,
                            false,
                            pos_array_copy,
                        );
                        paths.push(e);
                    }
                    self.instructions = vec![];
                    self.paths = Some(paths);
                    break
                },
                _ => {
                    self.compute_stack_effects(signature.as_str(), data_list)?;
                },
            }
            counter += 1;
        }
        match &self.paths {
            Some(p) => {
                for mut path in p.to_vec() {
                    path.execute()?;
                }
            },
            None => {
                self.instructions = vec![];
            },
        }
        self.completed = true;
        Ok(())
    }

    pub fn compute_stack_effects(&mut self, operation: &str, arg2: Vec<u8>) -> IccResult<()> {
        match operation {
            "data" => {
                self.stack += 1;
            },

            "in  " => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                for _i in 0 .. t + 1 {
                    self.stack += 1;
                }
            },
            "out " => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < t + 1 {
                    return Err(String::from("Stack underflowed on out operation"))
                }

                for _i in 0 .. t + 1 {
                    self.stack -= 1;
                }
            },
            "tget" => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                for _i in 0 .. t + 1 {
                    self.stack += 1;
                }
            },
            "tput" => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < t + 1 {
                    return Err(String::from("Stack underflowed on tput operation"))
                }
                for _i in 0 .. t + 1 {
                    self.stack -= 1;
                }
            },
            "tsav" => {
                // Does not impact stack
            },

            "env " => {
                self.stack += 1;
            },
            //
            "curv" => {
                // Must be cvst
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let gen = self.pos_array[s as usize].clone().gen();
                match gen {
                    Some(v) => {
                        if v.signature() != "cvst".to_string() {
                            return Err(String::from("Expected a cvst subelement"))
                        }
                    },
                    None => return Err(String::from("Expected a cvst subelement")),
                }
            },
            "mtx " => {
                // Must be a matf
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let gen = self.pos_array[s as usize].clone().gen();
                match gen {
                    Some(v) => {
                        if v.signature() != "matf".to_string() {
                            return Err(String::from("Expected a matf subelement"))
                        }
                    },
                    None => return Err(String::from("Expected a matf subelement")),
                }
            },
            "clut" => {
                // Must be clut
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let gen = self.pos_array[s as usize].clone().gen();
                match gen {
                    Some(v) => {
                        if v.signature() != "clut".to_string() {
                            return Err(String::from("Expected a clut subelement"))
                        }
                    },
                    None => return Err(String::from("Expected a clut subelement")),
                }
            },
            "calc" => {
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let calc = self.pos_array[s as usize].clone().calc();
                match calc {
                    Some(v) => {
                        if v.signature() != true {
                            return Err(String::from("Expected a calc subelement"))
                        }
                    },
                    None => return Err(String::from("Expected a calc subelement")),
                }
            },
            "tint" => {
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let gen = self.pos_array[s as usize].clone().gen();
                match gen {
                    Some(v) => {},
                    None => return Err(String::from("Did not expect a calc subelement")),
                }
            },
            "elem" => {
                let s = ((arg2[0] as u32) << 24)
                    + ((arg2[1] as u32) << 16)
                    + ((arg2[2] as u32) << 8)
                    + (arg2[3] as u32);
                if s >= self.pos_array.len() as u32 {
                    return Err(String::from("Not enough subelements in MPET"))
                }
                let gen = self.pos_array[s as usize].clone().gen();
                match gen {
                    Some(v) => {},
                    None => return Err(String::from("Did not expect a calc subelement")),
                }
            },
            // Stack Operations
            "copy" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < s + 1 {
                    return Err(String::from("Stack underflowed on copy operation"))
                }
                self.stack += (s + 1) * (t + 1);
            },
            "rotl" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);

                if (self.stack as u16) < s + 1 {
                    return Err(String::from("Stack underflowed on rotl operation"))
                }
                // Size of stack doesn't change
            },
            "rotr" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);

                if self.stack < s + 1 {
                    return Err(String::from("Stack underflowed on rotr operation"))
                }
                // Size of stack doesn't change
            },
            "posd" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                // Find the value at the sth position of the stack (0 is top),
                // push that value t+1 times to the top of the stack
                if self.stack < s + 1 {
                    return Err(String::from("Stack underflowed on posd operation"))
                }
                self.stack += t + 1;
            },
            "flip" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (self.stack as u16) < s + 1 {
                    return Err(String::from("Not enough stack elements to pop"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for flip"))
                }
                // Size of stack doesn't change
            },
            "pop " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if (self.stack as u16) < s + 1 {
                    return Err(String::from("Not enough stack elements to pop"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for pop"))
                }
                self.stack -= s + 1;
            },
            // Matrix Operations
            "solv" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                // matrix is s+1 * t+1, y is s+1, resulting is X is t + 1 + 1.
                let substract_val = (((s + 1) * (t + 1)) + (s + 1) - (t + 1 + 1));
                if (self.stack as u16) < substract_val {
                    return Err(String::from("Not enough stack elements to pop"))
                }

                self.stack -= substract_val;
            },
            "tran" => {
                // num elements remain same.
            },
            // Sequence Functional Operations
            // TODO: Table 100 seems to use top S+2 values instead of the conventional S+1
            "sum " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on sum"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sum"))
                }

                self.stack -= s + 1;
            },
            "prod" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on prod"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for prod"))
                }
                self.stack -= s + 1;
            },
            "min " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on min"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for min"))
                }
                self.stack -= s + 1;
            },
            "max " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on max"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for max"))
                }
                self.stack -= s + 1;
            },
            "and " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on and"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for and"))
                }
                self.stack -= s + 1;
            },
            "or  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s + 2 > self.stack as u16 {
                    return Err(String::from("Stack underflow on or"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for or"))
                }
                self.stack -= s + 1;
            },
            // Functional Vector Operation
            // S is u16
            // T is 0
            "pi  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s != 0 {
                    return Err(String::from("s must be 0 for pi"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for pi"))
                }
                self.stack += 1;
            },
            "+INF" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s != 0 {
                    return Err(String::from("s must be 0 for +INF"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for +INF"))
                }
                self.stack += 1;
            },
            "-INF" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s != 0 {
                    return Err(String::from("s must be 0 for -INF"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for -INF"))
                }
                self.stack += 1;
            },
            "NaN " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if s != 0 {
                    return Err(String::from("s must be 0 for NaN"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for NaN"))
                }
                self.stack += 1;
            },
            "add " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for add",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for add"))
                }
                self.stack -= s + 1;
            },
            "sub " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sub",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sub"))
                }
                self.stack -= s + 1;
            },
            "mul " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for mul",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for mul"))
                }
                self.stack -= s + 1;
            },
            "div " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for div",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for div"))
                }
                self.stack -= s + 1;
            },
            "mod " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for mod",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for mod"))
                }
                self.stack -= s + 1;
            },
            "pow " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for pow",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for pow"))
                }
                self.stack -= s + 1;
            },
            "gama" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for gama",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for gama"))
                }
                self.stack -= 1;
            },
            "sadd" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sadd",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sadd"))
                }
                self.stack -= 1;
            },
            "ssub" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for ssub",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ssub"))
                }
                self.stack -= 1;
            },
            "smul" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for smul",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for smul"))
                }
                self.stack -= 1;
            },
            "sdiv" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sdiv",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sdiv"))
                }
                self.stack -= 1;
            },
            "sq  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for sq"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sq"))
                }
            },
            "sqrt" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sqrt",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sqrt"))
                }
            },
            "cb  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for cb"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for cb"))
                }
            },
            "cbrt" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sqrt",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sqrt"))
                }
            },
            "abs " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for abs",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for abs"))
                }
            },
            "neg " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for neg",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for neg"))
                }
            },
            "rond" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for rond",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for rond"))
                }
            },
            "flor" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for flor",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for flor"))
                }
            },
            "ceil" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for ceil",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ceil"))
                }
            },
            "trnc" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for trunc",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for trunc"))
                }
            },
            "sign" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sign",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sign"))
                }
            },
            "exp " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for exp",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for exp"))
                }
            },
            "log " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for log",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for log"))
                }
            },
            "ln  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ln"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ln"))
                }
            },
            "sin " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sin",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sin"))
                }
            },
            "cos " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for cos",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for cos"))
                }
            },
            "tan " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for tan",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for tan"))
                }
            },
            "asin" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for asin",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for asin"))
                }
            },
            "acos" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for acos",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for acos"))
                }
            },
            "atan" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for atan",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for atan"))
                }
            },
            "atn2" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for atn2",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for atn2"))
                }
                self.stack -= s + 1;
            },
            "ctop" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for ctop",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for lt"))
                }
            },
            "ptoc" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for ptoc",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ptoc"))
                }
            },
            "rnum" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for rnum",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for rnum"))
                }
            },
            "lt  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for lt"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for lt"))
                }
                self.stack = self.stack - (s + 1);
            },
            "le  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for le"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for le"))
                }
                self.stack = self.stack - (s + 1);
            },
            "eq  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for eq"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for eq"))
                }
                self.stack = self.stack - (s + 1);
            },
            "near" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for near",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for near"))
                }
                self.stack = self.stack - (s + 1);
            },
            "ge  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ge"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ge"))
                }
                self.stack = self.stack - (s + 1);
            },
            "gt  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for gt"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for gt"))
                }
                self.stack = self.stack - (s + 1);
            },
            "vmin" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vmin",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vmin"))
                }
                self.stack = self.stack - (s + 1);
            },
            "vmax" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vmax",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vmax"))
                }
                self.stack = self.stack - (s + 1);
            },
            "vand" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vand",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vand"))
                }
                self.stack = self.stack - (s + 1);
            },
            "vor " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vor",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vor"))
                }
                self.stack = self.stack - (s + 1);
            },
            "tLab" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for tLab",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for tLabb"))
                }
            },
            "tXYZ" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for tXYZ",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for tXYZ"))
                }
            },

            // anomalous
            /*
            The fJab operator invokes an indexed JabToXYZElement which takes 3 input arguments and results in 3 output arguments.
            The tJab operator invokes an indexed XYZToJabElmeent which takes 3 input arguments and result in 3 output arguments.
            */
            "fJab" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for fJab",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for fJab"))
                }
            },
            "tJab" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for tJab",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for tJab"))
                }
            },
            "fLab" => {
                // Identical to tXYZ
                // No effect on stack size
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for fLab",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for fLab"))
                }
            },
            "not " => {
                // Identical to vnot
                // No stack operations
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for not",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for not"))
                }
            },
            "ne  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ne"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ne"))
                }
                self.stack -= s + 1;
            },
            "vxor" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vxor",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vxor"))
                }
                self.stack -= s + 1;
            },
            "vnot" => {
                // Identical to not
                // No stack operations
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.stack < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vnot",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vnot"))
                }
            },

            _ => {
                return Err(String::from(format!(
                    "Unknown operator found {:?}",
                    operation
                )))
            },
        }
        return Ok(())
    }
}
