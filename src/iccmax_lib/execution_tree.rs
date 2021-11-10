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
    // stack:           u32,
    max_stack:       u32,
    paths:           Vec<u32>,
    instructions:    Vec<Operations>,
    // completed:       bool,
    pos_array:       Vec<MPetOptions>,
    input_channels:  u16,
    output_channels: u16,
}

impl ExecutionTree {
    pub fn new(
        max_stack: u32, paths: Vec<u32>, instructions: Vec<Operations>,
        pos_array: Vec<MPetOptions>, input_channels: u16, output_channels: u16,
    ) -> Self {
        Self {
            max_stack,
            paths,
            instructions,
            pos_array,
            input_channels,
            output_channels,
        }
    }

    pub fn paths(&mut self) -> Vec<u32> { self.paths.clone() }

    pub fn execute(&mut self) -> IccResult<()> {
        // println!("{:?}", self);
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
            let signature_copy = self.instructions[counter].clone().signature();
            let function_operations = self.instructions[counter].clone().function_operations();
            let data_list = self.instructions[counter].clone().data().data_list();
            match signature.as_str() {
                "if" | "sel" => {
                    // let mut paths: Vec<ExecutionTree> = vec![];

                    let mut new_stack: Vec<u32> = vec![];
                    for c in 0 .. function_operations.len() {
                        /*
                         * If we have only an if condition, then we still have two branches
                         */
                        if function_operations.len() == 1 {
                            new_stack.append(&mut self.paths);
                        }
                        for path in 0 .. self.paths.len() {
                            let f = function_operations[c].clone();
                            let mut e = ExecutionTree::new(
                                self.max_stack,
                                [self.paths[path]].to_vec(),
                                f,
                                self.pos_array.clone(),
                                self.input_channels,
                                self.output_channels,
                            );
                            e.execute()?;
                            new_stack.append(&mut e.paths());
                        }
                    }
                    if *new_stack.iter().min().unwrap() != *new_stack.iter().max().unwrap() {
                        self.paths = [
                            *new_stack.iter().min().unwrap(),
                            *new_stack.iter().max().unwrap(),
                        ]
                        .to_vec();
                    } else {
                        self.paths = [*new_stack.iter().min().unwrap()].to_vec();
                    }
                    //     let mut part: Vec<Operations> =
                    // self.instructions[counter + 1 ..].to_vec();
                    //     let mut tmp_func = function_operations[c].clone();
                    //     tmp_func.append(&mut part);
                    //     if signature_copy == "sel" {
                    //         if *stack == 0 {
                    //             return Err(String::from("Stack underflow on
                    // sel operation"))         }
                    //         *stack -= 1;
                    //     }
                    //     let pos_array_copy = self.pos_array.clone();
                    //     let e = ExecutionTree::new(
                    //         self.stack,
                    //         self.max_stack,
                    //         None,
                    //         tmp_func,
                    //         false,
                    //         pos_array_copy,
                    //         self.input_channels,
                    //         self.output_channels,
                    //     );
                    //     paths.push(e);
                    // }
                    // self.instructions = vec![];
                    // self.paths = Some(paths);
                    // break
                },
                _ => {
                    for path in 0 .. self.paths.len() {
                        let data_list_new = data_list.clone();
                        self.compute_stack_effects(signature.as_str(), data_list_new, path)?;
                    }
                },
            }
            if *self.paths.iter().max().unwrap() > self.max_stack {
                self.max_stack = *self.paths.iter().max().unwrap();
            }
            // println!("{:?} {:?}", self.max_stack, self.paths);
            counter += 1;
        }
        // match &self.paths {
        //     Some(p) => {
        //         for mut path in p.to_vec() {
        //             path.execute()?;
        //         }
        //     },
        //     None => {
        //         self.instructions = vec![];
        //     },
        // }
        // self.completed = true;
        Ok(())
    }

    pub fn compute_stack_effects(
        &mut self, operation: &str, arg2: Vec<u8>, counter: usize,
    ) -> IccResult<()> {
        match operation {
            "data" => {
                self.paths[counter] += 1;
            },

            "in  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if s >= self.input_channels {
                    return Err(String::from("Not enough input channels"))
                }
                if s + t >= self.input_channels {
                    return Err(String::from("Not enough input channels"))
                }
                for _i in 0 .. t + 1 {
                    self.paths[counter] += 1;
                }
            },
            "out " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if s >= self.output_channels {
                    return Err(String::from("Not enough output channels"))
                }
                if s + t >= self.output_channels {
                    return Err(String::from("Not enough output channels"))
                }
                if self.paths[counter] < (t + 1) as u32 {
                    return Err(String::from("Stack underflowed on out operation"))
                }

                for _i in 0 .. t + 1 {
                    self.paths[counter] -= 1;
                }
            },
            "tget" => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                for _i in 0 .. t + 1 {
                    self.paths[counter] += 1;
                }
            },
            "tput" => {
                let _s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (t + 1) as u32 {
                    return Err(String::from("Stack underflowed on tput operation"))
                }
                for _i in 0 .. t + 1 {
                    self.paths[counter] -= 1;
                }
            },
            "tsav" => {
                // Does not impact *stack
            },

            "env " => {
                self.paths[counter] += 1;
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
                        let v1 = v.clone();
                        let v2 = v.clone();
                        if v.signature() != "cvst".to_string() {
                            return Err(String::from("Expected a cvst subelement"))
                        }
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for curv subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
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
                        let v1 = v.clone();
                        let v2 = v.clone();
                        if v.signature() != "matf".to_string() {
                            return Err(String::from("Expected a matf subelement"))
                        }
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for mtx subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
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
                        let v1 = v.clone();
                        let v2 = v.clone();
                        if v.signature() != "clut".to_string() {
                            return Err(String::from("Expected a clut subelement"))
                        }
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for clut subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
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
                        let v1 = v.clone();
                        let v2 = v.clone();
                        if v.signature() != true {
                            return Err(String::from("Expected a calc subelement"))
                        }
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for calc subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
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
                    Some(v) => {
                        let v1 = v.clone();
                        let v2 = v.clone();
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for curv subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
                    },
                    None => return Err(String::from("Did not expect a tint subelement")),
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
                    Some(v) => {
                        let v1 = v.clone();
                        let v2 = v.clone();
                        let input_channels = v1.input_channels();
                        let output_channels = v2.output_channels();
                        if self.paths[counter] < input_channels as u32 {
                            return Err(String::from("Not enough stack values for elem subelement"))
                        }
                        self.paths[counter] = self.paths[counter] - (input_channels as u32)
                            + (output_channels as u32);
                    },
                    None => return Err(String::from("Did not expect a calc subelement")),
                }
            },
            // Stack Operations
            "copy" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflowed on copy operation"))
                }
                self.paths[counter] += ((s + 1) as u32) * ((t + 1) as u32);
            },
            "rotl" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let _t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflowed on rotl operation"))
                }
                // Size of stack doesn't change
            },
            "rotr" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let _t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflowed on rotr operation"))
                }
                // Size of stack doesn't change
            },
            "posd" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                // Find the value at the sth position of the stack (0 is top),
                // push that value t+1 times to the top of the stack
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflowed on posd operation"))
                }
                self.paths[counter] += (t + 1) as u32;
            },
            "flip" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Not enough stack elements to pop"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for pop"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            // Matrix Operations
            "solv" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                // matrix is s+1 * t+1, y is s+1, resulting is X is t + 1 + 1.
                let substract_val = ((s + 1) * (t + 1)) + (s + 1) - (t + 1 + 1);
                if self.paths[counter] < (substract_val as u32) {
                    return Err(String::from("Not enough stack elements to pop"))
                }

                self.paths[counter] -= substract_val as u32;
            },
            "tran" => {
                // num elements remain same.
            },
            // Sequence Functional Operations
            // TODO: Table 100 seems to use top S+2 values instead of the conventional S+1
            "sum " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on sum"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sum"))
                }

                self.paths[counter] -= (s + 1) as u32;
            },
            "prod" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on prod"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for prod"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "min " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on min"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for min"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "max " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on max"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for max"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "and " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on and"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for and"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "or  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);

                if (s + 2) as u32 > self.paths[counter] {
                    return Err(String::from("Stack underflow on or"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for or"))
                }
                self.paths[counter] -= (s + 1) as u32;
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
                self.paths[counter] += 1;
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
                self.paths[counter] += 1;
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
                self.paths[counter] += 1;
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
                self.paths[counter] += 1;
            },
            "add " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for add",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for add"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "sub " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sub",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sub"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "mul " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for mul",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for mul"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "div " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for div",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for div"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "mod " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for mod",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for mod"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "pow " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for pow",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for pow"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "gama" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for gama",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for gama"))
                }
                self.paths[counter] -= 1;
            },
            "sadd" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sadd",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sadd"))
                }
                self.paths[counter] -= 1;
            },
            "ssub" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for ssub",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ssub"))
                }
                self.paths[counter] -= 1;
            },
            "smul" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for smul",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for smul"))
                }
                self.paths[counter] -= 1;
            },
            "sdiv" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for sdiv",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sdiv"))
                }
                self.paths[counter] -= 1;
            },
            "sq  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for sq"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for sq"))
                }
            },
            "sqrt" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for cb"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for cb"))
                }
            },
            "cbrt" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ln"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ln"))
                }
            },
            "sin " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for atn2",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for atn2"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "ctop" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
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
                if self.paths[counter] < (2 * (s + 1)).into() {
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
                if self.paths[counter] < (s + 1).into() {
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
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for lt"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for lt"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "le  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for le"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for le"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "eq  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for eq"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for eq"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "near" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for near",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for near"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "ge  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ge"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ge"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "gt  " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for gt"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for gt"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "vmin" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vmin",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vmin"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "vmax" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vmax",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vmax"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "vand" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vand",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vand"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "vor " => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (2 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vor",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vor"))
                }
                self.paths[counter] = self.paths[counter] - (s + 1) as u32;
            },
            "tLab" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for tLab",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for tLab"))
                }
            },
            "tXYZ" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (3 * (s + 1)).into() {
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
                if self.paths[counter] < (3 * (s + 1)).into() {
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
                if self.paths[counter] < (3 * (s + 1)).into() {
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
                if self.paths[counter] < (3 * (s + 1)).into() {
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
                if self.paths[counter] < (3 * (s + 1)).into() {
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
                if self.paths[counter] < (3 * (s + 1)).into() {
                    return Err(String::from("Stack underflow, not enough arguments for ne"))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for ne"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "vxor" => {
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (3 * (s + 1)).into() {
                    return Err(String::from(
                        "Stack underflow, not enough arguments for vxor",
                    ))
                }
                if t != 0 {
                    return Err(String::from("t must be 0 for vxor"))
                }
                self.paths[counter] -= (s + 1) as u32;
            },
            "vnot" => {
                // Identical to not
                // No stack operations
                let s = ((arg2[0] as u16) << 8) + (arg2[1] as u16);
                let t = ((arg2[2] as u16) << 8) + (arg2[3] as u16);
                if self.paths[counter] < (3 * (s + 1)).into() {
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
        if self.paths[counter] > 65535 {
            return Err(String::from(format!(
                "Stack overflow: length is {:?}",
                self.paths[counter]
            )))
        }
        return Ok(())
    }
}
#[cfg(test)]
mod test_iccmax_execution {
    use super::ExecutionTree;
    use crate::iccmax_lib::iccmax_prim::OperationsP;
    use crate::pcore::parsebuffer::{ParseBuffer, ParsleyParser};

    #[test]
    fn test_operations_if() {
        let mut parser = OperationsP;
        let v = Vec::from("if  \x00\x00\x00\x01pi  \x00\x00\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut e = ExecutionTree::new(0, [1].to_vec(), [r].to_vec(), [].to_vec(), 0, 0);
        e.execute();
        assert_eq!(e.paths(), [1, 2].to_vec());
    }

    #[test]
    fn test_operations_if_if() {
        let mut parser = OperationsP;
        let v = Vec::from("if  \x00\x00\x00\x02else\x00\x00\x00\x01if  \x00\x00\x00\x01pi  \x00\x00\x00\x00pi  \x00\x00\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut e = ExecutionTree::new(0, [1].to_vec(), [r].to_vec(), [].to_vec(), 0, 0);
        e.execute();
        assert_eq!(e.paths(), [1, 2].to_vec());
    }

    #[test]
    fn test_operations_if_else() {
        let mut parser = OperationsP;
        let v = Vec::from("if  \x00\x00\x00\x04else\x00\x00\x00\x01if  \x00\x00\x00\x01else\x00\x00\x00\x01pi  \x00\x00\x00\x00pi  \x00\x00\x00\x00pi  \x00\x00\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut e = ExecutionTree::new(0, [1].to_vec(), [r].to_vec(), [].to_vec(), 0, 0);
        e.execute();
        assert_eq!(e.paths(), [2].to_vec());
    }
}
