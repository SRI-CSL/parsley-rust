// Copyright (c) 2019-2020 SRI International.
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

use crate::iccmax_lib::execution_tree::ExecutionTree;
use crate::pcore::parsebuffer::{
    ErrorKind, LocatedVal, ParseBuffer, ParseBufferT, ParseResult, ParsleyParser,
};
use crate::pcore::prim_binary::{BinaryMatcher, Endian, UInt16P, UInt32P, UInt8P};
use crate::pcore::prim_combinators::{Alt, Alternate, Star};

type IccError = String;
type IccResult<T> = std::result::Result<T, IccError>;

pub fn resolve_operations(operation: Operations, mut stack: &mut Vec<f32>) -> IccResult<()> {
    let before_stack = stack.clone();
    let signature = operation.clone().data().signature();
    let data = operation.clone().data().data();
    let data_list = operation.clone().data().data_list();
    let function_operations = operation.clone().function_operations();
    match operation.signature().as_str() {
        "if" => {
            for option in function_operations {
                let mut tmp_stack = stack.clone();
                for function in option {
                    let function_1 = function.clone();
                    let function_2 = function.clone();
                    let function_data = function.clone().data().data();
                    let function_data_list = function.clone().data().data_list();
                    if function.signature() == "if" {
                        resolve_operations(function_2, &mut tmp_stack)?;
                    } else if function_1.signature() == "sel" {
                        resolve_operations(function_2, &mut tmp_stack)?;
                    } else {
                        compute_operations(
                            &function_2.signature(),
                            function_data,
                            function_data_list,
                            &mut tmp_stack,
                        )?;
                    }
                }
            }
        },
        "sel" => {
            for option in function_operations {
                let mut tmp_stack = stack.clone();
                for function in option {
                    let function_1 = function.clone();
                    let function_2 = function.clone();
                    let function_data = function.clone().data().data();
                    let function_data_list = function.clone().data().data_list();
                    if function.signature() == "if" {
                        resolve_operations(function_2, &mut tmp_stack)?;
                    } else if function_1.signature() == "sel" {
                        resolve_operations(function_2, &mut tmp_stack)?;
                    } else {
                        compute_operations(
                            &function_2.signature(),
                            function_data,
                            function_data_list,
                            &mut tmp_stack,
                        )?;
                    }
                }
            }
        },
        _ => {
            compute_operations(&signature, data, data_list, &mut stack)?;
        },
    };
    println!("{:?}     {:?}    {:?}", signature, before_stack, stack);
    return Ok(())
}

pub fn compute_operations(
    operation: &str, arg1: f32, arg2: Vec<u8>, stack: &mut Vec<f32>,
) -> IccResult<()> {
    match operation {
        "data" => {
            stack.push(arg1);
        },

        "in  " => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            for _i in 0 .. t + 1 {
                stack.push(0.0);
            }
        },
        "out " => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            for _i in 0 .. t + 1 {
                stack.pop();
            }
        },
        "tget" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            for _i in 0 .. t + 1 {
                stack.push(0.0);
            }
        },
        "tput" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            for _i in 0 .. t + 1 {
                stack.pop();
            }
        },
        "tsav" => {
            // Does not impact stack
        },

        "env " => {},
        //
        "curv" => {},
        "mtx " => {},
        "clut" => {},
        "calc" => {},
        "tint" => {},
        "elem" => {},
        // Stack Operations
        "copy" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            let mut tmp: Vec<f32> = vec![];
            if stack.len() < (s as usize) + 1 {
                return Err(String::from("Stack underflowed on copy operation"))
            }
            for _i in 0 .. s + 1 {
                tmp.insert(0, stack.pop().unwrap());
            }
            for _i in 0 .. t + 2 {
                for t in &tmp {
                    stack.push(*t);
                }
            }
        },
        "rotl" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if (stack.len() as u16) < s + 1 {
                return Err(String::from("Not enough stack elements to rtol"))
            }

            // rotate left top S+1 elements T+1 positions on stack
            let mut tmp: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                tmp.insert(0, stack.pop().unwrap());
            }
            // rotate left
            tmp.rotate_left((t + 1).into()); // t+1 was type u16. .into() converts to usize
                                             // reinsert into stack
            for _i in 0 .. s + 1 {
                stack.push(tmp.remove(0));
            }
        },
        "rotr" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if stack.len() < (s as usize) + 1 {
                return Err(String::from("Stack underflowed on rotr operation"))
            }
            // rotate right top S+1 elements T+1 positions on stack
            let mut tmp: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                tmp.insert(0, stack.pop().unwrap());
            }
            // rotate right
            tmp.rotate_right((t + 1).into());

            // reinsert into stack
            for _i in 0 .. s + 1 {
                stack.push(tmp.remove(0));
            }
        },
        "posd" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            // Find the value at the sth position of the stack (0 is top),
            // push that value t+1 times to the top of the stack
            if stack.len() < (s as usize) + 1 {
                return Err(String::from("Stack underflowed on posd operation"))
            }
            let mut value: f32 = 0.0;
            let mut flag = false;
            for counter in (0 .. s + 1).rev() {
                if counter == 0 {
                    value = stack[counter as usize];
                    flag = true;
                    break
                }
            }
            if !flag {
                return Err(String::from("Stack underflowed on posd operation"))
            }
            for _i in 0 .. t + 1 {
                stack.push(value);
            }
        },
        "flip" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if (stack.len() as u16) < s + 1 {
                return Err(String::from("Not enough stack elements to pop"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for flip"))
            }
            let mut tmp: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                tmp.insert(0, stack.pop().unwrap());
            }
            for _i in 0 .. s + 1 {
                stack.push(tmp.pop().unwrap());
            }
        },
        "pop " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if (stack.len() as u16) < s + 1 {
                return Err(String::from("Not enough stack elements to pop"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for pop"))
            }
            for _i in 0 .. s + 1 {
                stack.pop();
            }
        },
        // Matrix Operations
        "solv" => {},
        "tran" => {},
        // Sequence Functional Operations
        // TODO: Table 100 seems to use top S+2 values instead of the conventional S+1
        "sum " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on sum"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sum"))
            }

            let mut sum = 0.0;
            for _i in 0 .. s + 2 {
                sum += stack.pop().unwrap();
            }
            stack.push(sum);
        },
        "prod" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on prod"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for prod"))
            }

            let mut sum = 1.0;
            for _i in 0 .. s + 2 {
                sum *= stack.pop().unwrap();
            }
            stack.push(sum);
        },
        "min " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on min"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for min"))
            }

            let mut m = f32::MAX;
            for _i in 0 .. s + 2 {
                let s = stack.pop().unwrap();
                if s < m {
                    m = s;
                }
            }
            stack.push(m);
        },
        "max " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on max"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for max"))
            }

            let mut m = f32::MIN;
            for _i in 0 .. s + 2 {
                let s = stack.pop().unwrap();
                if s > m {
                    m = s;
                }
            }
            stack.push(m);
        },
        "and " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on and"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for and"))
            }

            let mut sum = 1.0;
            for _i in 0 .. s + 2 {
                let s = stack.pop().unwrap();
                if s < 0.5 {
                    sum = 0.0;
                }
            }
            stack.push(sum);
        },
        "or  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s + 2 > stack.len() as u16 {
                return Err(String::from("Stack underflow on or"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for or"))
            }

            let mut sum = 0.0;
            for _i in 0 .. s + 2 {
                let s = stack.pop().unwrap();
                if s >= 0.5 {
                    sum = 1.0;
                }
            }
            stack.push(sum);
        },
        // Functional Vector Operation
        // S is u16
        // T is 0
        "pi  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s != 0 {
                return Err(String::from("s must be 0 for pi"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for pi"))
            }
            stack.push(3.14)
        },
        "+INF" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s != 0 {
                return Err(String::from("s must be 0 for +INF"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for +INF"))
            }
            stack.push(f32::INFINITY);
        },
        "-INF" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s != 0 {
                return Err(String::from("s must be 0 for -INF"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for -INF"))
            }
            stack.push(f32::NEG_INFINITY);
        },
        "NaN " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);

            if s != 0 {
                return Err(String::from("s must be 0 for NaN"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for NaN"))
            }
            stack.push(f32::NAN);
        },
        "add " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for add",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for add"))
            }
            let mut arr_x: Vec<f32> = vec![];
            let mut arr_y: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap() + arr_x[i as usize]);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_y.pop().unwrap());
            }
        },
        "sub " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sub",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sub"))
            }
            let mut arr_x: Vec<f32> = vec![];
            let mut arr_y: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() - arr_y[i as usize]);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "mul " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for mul",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for mul"))
            }
            let mut arr_x: Vec<f32> = vec![];
            let mut arr_y: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                arr_y.push(arr_x[i as usize] * stack.pop().unwrap());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_y.pop().unwrap());
            }
        },
        "div " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for div",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for div"))
            }
            let mut arr_x: Vec<f32> = vec![];
            let mut arr_y: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() / arr_y[i as usize]);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "mod " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            // TODO
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for mod",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for mod"))
            }
        },
        "pow " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for pow",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for pow"))
            }
            let mut arr_x: Vec<f32> = vec![];
            let mut arr_y: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                arr_x.push(f32::powf(stack.pop().unwrap(), arr_y[i as usize]));
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "gama" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for gama",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for gama"))
            }
            let y = stack.pop().unwrap();
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(f32::powf(stack.pop().unwrap(), y));
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sadd" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sadd",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sadd"))
            }
            let y = stack.pop().unwrap();
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() + y);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "ssub" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for ssub",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for ssub"))
            }
            let y = stack.pop().unwrap();
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() - y);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "smul" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for smul",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for smul"))
            }
            let y = stack.pop().unwrap();
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() * y);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sdiv" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sdiv",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sdiv"))
            }
            let y = stack.pop().unwrap();
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_x.push(stack.pop().unwrap() / y);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sq  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from("Stack underflow, not enough arguments for sq"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sq"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s * s);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sqrt" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sqrt",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sqrt"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.sqrt());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "cb  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from("Stack underflow, not enough arguments for cb"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for cb"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s * s * s);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "cbrt" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sqrt",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sqrt"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.cbrt());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "abs " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for abs",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for abs"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                if s < 0.0 {
                    arr_x.push(-s);
                } else {
                    arr_x.push(s);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "neg " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for neg",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for neg"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(-s);
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "rond" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            //TODO
        },
        "flor" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for flor",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for flor"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.floor());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "ceil" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for ceil",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for ceil"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.ceil());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "trnc" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for trunc",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for trunc"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.trunc());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sign" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sign",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sign"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                if s < 0.0 {
                    arr_x.push(-1.0);
                } else if s > 0.0 {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "exp " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for exp",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for exp"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.exp());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "log " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for log",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for log"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.log10());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "ln  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from("Stack underflow, not enough arguments for ln"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for ln"))
            }
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.ln());
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "sin " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for sin",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for sin"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.sin());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "cos " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for cos",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for cos"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.cos());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "tan " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for tan",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for tan"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.tan());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "asin" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for asin",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for asin"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.asin());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "acos" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for acos",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for acos"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.acos());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "atan" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for atan",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for atan"))
            }

            let mut arr_x: Vec<f32> = vec![];

            for _i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                arr_x.push(s.atan());
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "atn2" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (s + 1).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for atn2",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for atn2"))
            }

            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }

            for i in 0 .. s + 1 {
                let s = stack.pop().unwrap();
                // y.atan2(x)
                arr_x.push(arr_y[i as usize].atan2(s));
            }

            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "ctop" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "ptoc" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "rnum" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "lt  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from("Stack underflow, not enough arguments for lt"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for lt"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i < arr_y[i as usize] {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "le  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from("Stack underflow, not enough arguments for le"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for le"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i <= arr_y[i as usize] {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "eq  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from("Stack underflow, not enough arguments for eq"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for eq"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i == arr_y[i as usize] {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "near" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            // TODO
        },
        "ge  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from("Stack underflow, not enough arguments for ge"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for ge"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i >= arr_y[i as usize] {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "gt  " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from("Stack underflow, not enough arguments for gt"))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for gt"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i > arr_y[i as usize] {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "vmin" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for vmin",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for vmin"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                arr_x.push(f32::min(x_i, arr_y[i as usize]));
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "vmax" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for vmax",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for vmax"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                arr_x.push(f32::max(x_i, arr_y[i as usize]));
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "vand" => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for vand",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for vand"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i >= 0.5 && arr_y[i as usize] >= 0.5 {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "vor " => {
            let s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
            let t = ((arg2[2] as u16) >> 8) + (arg2[3] as u16);
            if stack.len() < (2 * (s + 1)).into() {
                return Err(String::from(
                    "Stack underflow, not enough arguments for vand",
                ))
            }
            if t != 0 {
                return Err(String::from("t must be 0 for vor"))
            }
            let mut arr_y: Vec<f32> = vec![];
            let mut arr_x: Vec<f32> = vec![];
            for _i in 0 .. s + 1 {
                arr_y.push(stack.pop().unwrap());
            }
            for i in 0 .. s + 1 {
                let x_i = stack.pop().unwrap();
                if x_i >= 0.5 || arr_y[i as usize] >= 0.5 {
                    arr_x.push(1.0);
                } else {
                    arr_x.push(0.0);
                }
            }
            for _i in 0 .. s + 1 {
                stack.push(arr_x.pop().unwrap());
            }
        },
        "tLab" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "tXYZ" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },

        // anomalous
        "fJab" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "tJab" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "fLab" => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },
        "not " => {
            let _s = ((arg2[0] as u16) >> 8) + (arg2[1] as u16);
        },

        _ => {
            return Err(String::from(format!(
                "Unknown operator found {:?}",
                operation
            )))
        },
    }
    if stack.len() > 65535 {
        return Err(String::from(format!(
            "Stack overflow: length is {:?}",
            stack.len()
        )))
    }
    return Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PositionNumber {
    position: u32,
    size:     u32,
}
impl PositionNumber {
    pub fn new(position: u32, size: u32) -> Self { Self { position, size } }
    pub fn position(self) -> u32 { self.position }
    pub fn size(self) -> u32 { self.size }
}
pub struct PositionNumberP;
impl ParsleyParser for PositionNumberP {
    type T = LocatedVal<PositionNumber>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        let mut int32 = UInt32P::new(Endian::Big);

        let position = int32.parse(buf)?;

        let size = int32.parse(buf)?;

        let g = PositionNumber::new(*position.val(), *size.val());
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MPetElement {
    signature:          bool,
    input_channels:     u16,
    output_channels:    u16,
    number_of_elements: u32,
    proc_table:         Vec<PositionNumber>,
    data_list:          Vec<u8>,
}
impl MPetElement {
    pub fn new(
        signature: bool, input_channels: u16, output_channels: u16, number_of_elements: u32,
        proc_table: Vec<PositionNumber>, data_list: Vec<u8>,
    ) -> Self {
        Self {
            signature,
            input_channels,
            output_channels,
            number_of_elements,
            proc_table,
            data_list,
        }
    }
    pub fn signature(self) -> bool { self.signature }
    pub fn data_list(self) -> Vec<u8> { self.data_list }
    pub fn number_of_elements(self) -> u32 { self.number_of_elements }
    pub fn proc_table(self) -> Vec<PositionNumber> { self.proc_table }
    pub fn input_channels(self) -> u16 { self.input_channels }
    pub fn output_channels(self) -> u16 { self.output_channels }
}
pub struct MPetElementP;
impl ParsleyParser for MPetElementP {
    type T = LocatedVal<MPetElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut g1 = BinaryMatcher::new(b"mpet");

        let start = buf.get_cursor();
        let signature = g1.parse(buf)?;

        let mut uint32_parser = UInt32P::new(Endian::Big);
        let reserved = uint32_parser.parse(buf)?;

        // This field must be 0
        assert_eq!(reserved.unwrap(), 0);

        let mut g2 = UInt16P::new(Endian::Big);
        let input_channels = g2.parse(buf)?;

        let output_channels = g2.parse(buf)?;

        let number_of_elements = uint32_parser.parse(buf)?;

        // TODO: number_of_elements must be >= 0

        // 8N times 64 UInt
        let mut counter = 0;
        let mut proc_table: Vec<PositionNumber> = Vec::with_capacity(65535);
        while counter < *number_of_elements.val() {
            let mut parser = PositionNumberP;
            let proc = parser.parse(buf)?;
            proc_table.push(*proc.val());
            counter = counter + 1;
        }
        let mut int8 = UInt8P;
        let mut star_int = Star::new(&mut int8);

        let data = star_int.parse(buf)?;
        let mut data_list: Vec<u8> = vec![];
        for datum in data.val() {
            data_list.push(*datum.val());
        }

        let g = MPetElement::new(
            *signature.val(),
            *input_channels.val(),
            *output_channels.val(),
            *number_of_elements.val(),
            proc_table,
            data_list,
        );
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GeneralElement {
    signature:       u32,
    input_channels:  u16,
    output_channels: u16,
}
impl GeneralElement {
    pub fn new(signature: u32, input_channels: u16, output_channels: u16) -> Self {
        Self {
            signature,
            input_channels,
            output_channels,
        }
    }
    pub fn signature(self) -> u32 { self.signature }
    pub fn input_channels(self) -> u16 { self.input_channels }
    pub fn output_channels(self) -> u16 { self.output_channels }
}
pub struct GeneralElementP;
impl ParsleyParser for GeneralElementP {
    type T = LocatedVal<GeneralElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut g1 = UInt32P::new(Endian::Big);

        let start = buf.get_cursor();
        let signature = g1.parse(buf)?;

        let reserved = g1.parse(buf)?;
        // This field must be 0
        assert_eq!(reserved.unwrap(), 0);

        let mut g3 = UInt16P::new(Endian::Big);
        let input_channels = g3.parse(buf)?;
        let output_channels = g3.parse(buf)?;

        let g = GeneralElement::new(
            *signature.val(),
            *input_channels.val(),
            *output_channels.val(),
        );
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CalculatorElement {
    signature:              bool,
    input_channels:         u16,
    output_channels:        u16,
    number_of_subelements:  u32,
    main_function_position: u32,
    main_function_size:     u32,
    subelement_positions:   Vec<PositionNumber>,
}
impl CalculatorElement {
    pub fn new(
        signature: bool, input_channels: u16, output_channels: u16, number_of_subelements: u32,
        main_function_position: u32, main_function_size: u32,
        subelement_positions: Vec<PositionNumber>,
    ) -> Self {
        Self {
            signature,
            input_channels,
            output_channels,
            number_of_subelements,
            main_function_position,
            main_function_size,
            subelement_positions,
        }
    }
    pub fn signature(self) -> bool { self.signature }
    pub fn input_channels(self) -> u16 { self.input_channels }
    pub fn output_channels(self) -> u16 { self.output_channels }
}
pub struct CalculatorElementP;
impl ParsleyParser for CalculatorElementP {
    type T = LocatedVal<CalculatorElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut g1 = BinaryMatcher::new(b"calc");

        let start = buf.get_cursor();
        let signature = g1.parse(buf)?;

        let mut g2 = UInt32P::new(Endian::Big);
        let reserved = g2.parse(buf)?;
        // This field must be 0
        assert_eq!(reserved.unwrap(), 0);

        let mut g3 = UInt16P::new(Endian::Big);
        let input_channels = g3.parse(buf)?;
        let output_channels = g3.parse(buf)?;
        let number_of_subelements = g2.parse(buf)?;

        let mut g4 = UInt32P::new(Endian::Big);
        let main_function_position = g4.parse(buf)?;
        let main_function_size = g4.parse(buf)?;

        let mut main_buf = ParseBuffer::new_view(
            buf,
            main_function_position.unwrap() as usize,
            main_function_size.unwrap() as usize,
        );
        let mut func_parser = CalcFunctionP;
        let _func_result = func_parser.parse(&mut main_buf)?;

        let mut subelement_positions: Vec<PositionNumber> = vec![];

        let mut counter = 0;
        while counter < number_of_subelements.unwrap() {
            // TODO: Seek to every subelement
            let mut position_number_parser = PositionNumberP;
            let p = position_number_parser.parse(buf)?;
            let position_result_unwrapped = p.unwrap();

            let seek_position = position_result_unwrapped.position();
            let seek_size = position_result_unwrapped.size();
            let _subelement_buf =
                ParseBuffer::new_view_cut(buf, seek_position as usize, seek_size as usize);
            //println!();
            //ParseBuffer::buf_to_string(&mut subelement_buf);
            //let subelement_result = func_parser.parse(&mut subelement_buf)?;

            subelement_positions.push(p.unwrap());
            counter = counter + 1;
        }

        let g = CalculatorElement::new(
            *signature.val(),
            *input_channels.val(),
            *output_channels.val(),
            *number_of_subelements.val(),
            *main_function_position.val(),
            *main_function_size.val(),
            subelement_positions,
        );
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataOperation {
    signature: String,
    data:      f32,
    data_list: Vec<u8>,
}
// Eq isn't implemented for f32, so had to remove
// Similarly, Copy isn't implemented for String since it is on the Heap
impl<'a> DataOperation {
    pub fn new(signature: String, data: f32, data_list: Vec<u8>) -> Self {
        Self {
            signature,
            data,
            data_list,
        }
    }

    pub fn signature(self) -> String { self.signature }
    pub fn data(self) -> f32 { self.data }
    pub fn data_list(self) -> Vec<u8> { self.data_list }
}
pub struct DataOperationP;
impl ParsleyParser for DataOperationP {
    type T = LocatedVal<Operations>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();

        let mut g1 = UInt8P;
        let g1_result = g1.parse(buf)?;
        let g2_result = g1.parse(buf)?;
        let g3_result = g1.parse(buf)?;
        let g4_result = g1.parse(buf)?;

        let mut signature_result = (*g1_result.val() as char).to_string();
        signature_result = signature_result + &(*g2_result.val() as char).to_string();
        signature_result = signature_result + &(*g3_result.val() as char).to_string();
        signature_result = signature_result + &(*g4_result.val() as char).to_string();

        let mut data_parser = UInt32P::new(Endian::Big);
        let d1_result = data_parser.parse(buf)?;
        let data = d1_result.unwrap().to_be_bytes();
        let data_copy = data.clone();
        let f32_data = f32::from_be_bytes(data_copy);

        let func_operations: Vec<Vec<Operations>> = vec![];
        let t_operators: Vec<Operations> = vec![];
        let signature_result_copy = signature_result.clone();
        let g = DataOperation::new(signature_result, f32_data, data.to_vec());
        let g1 = Operations::new(
            signature_result_copy,
            [].to_vec(),
            func_operations,
            0,
            t_operators,
            g,
        );
        Ok(LocatedVal::new(g1, start, buf.get_cursor()))
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Operations {
    signature:            String,
    number_of_operations: Vec<u32>,
    function_operations:  Vec<Vec<Operations>>,
    t_value:              u32,
    t_operators:          Vec<Operations>,
    data:                 DataOperation,
}
impl Operations {
    pub fn new(
        signature: String, number_of_operations: Vec<u32>,
        function_operations: Vec<Vec<Operations>>, t_value: u32, t_operators: Vec<Operations>,
        data: DataOperation,
        /*
         * These types need to change from DataOperation to Operations
         */
    ) -> Self {
        Self {
            signature,
            number_of_operations,
            function_operations,
            t_value,
            t_operators,
            data,
        }
    }
    pub fn function_operations(self) -> Vec<Vec<Operations>> { self.function_operations }
    pub fn t_operators(self) -> Vec<Operations> { self.t_operators }
    pub fn t_value(self) -> u32 { self.t_value }
    pub fn data(self) -> DataOperation { self.data }

    pub fn signature(self) -> String { self.signature }
    pub fn number_of_operations(self) -> u32 {
        let count = match self.signature.as_str() {
            "if" => {
                if self.number_of_operations[1] > 0 {
                    self.number_of_operations[0] + self.number_of_operations[1] + 2
                } else {
                    self.number_of_operations[0] + 1
                }
            },
            "sel" => {
                // sel - 1
                // case - n
                // dft - 1 //if present
                // all n
                // dft t
                let mut num: u32;
                num = 1 + self.number_of_operations.len() as u32;
                if self.t_value > 0 {
                    num += 1 + self.t_value;
                }
                for n in self.number_of_operations {
                    num += n;
                }
                num
            },
            _ => 1,
        };
        return count
    }
}
pub struct OperationsP;
impl ParsleyParser for OperationsP {
    type T = LocatedVal<Operations>;
    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let start = buf.get_cursor();
        let mut option1 = IfElseP;
        let mut option2 = DataOperationP;
        let mut option3 = SelectP;
        let mut complex = Alternate::new(&mut option1, &mut option3);
        let mut data_parser = Alternate::new(&mut complex, &mut option2);
        let data_result = data_parser.parse(buf)?;

        match data_result.unwrap() {
            Alt::Left(v) => {
                match v.unwrap() {
                    // IfElseP
                    Alt::Left(w) => Ok(LocatedVal::new(w.unwrap(), start, buf.get_cursor())),
                    // SelectP
                    Alt::Right(w) => Ok(LocatedVal::new(w.unwrap(), start, buf.get_cursor())),
                }
            },
            // This is the Data Operation
            Alt::Right(v) => Ok(LocatedVal::new(v.unwrap(), start, buf.get_cursor())),
        }
    }
}

pub struct SelectP;
impl ParsleyParser for SelectP {
    type T = LocatedVal<Operations>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut sel_parser = BinaryMatcher::new(b"sel ");
        let mut case_parser = BinaryMatcher::new(b"case");
        let mut dflt_parser = BinaryMatcher::new(b"dflt");

        let start = buf.get_cursor();
        match sel_parser.parse(buf) {
            Ok(_v) => {
                let mut uint32_parser = UInt32P::new(Endian::Big);
                let t_result = uint32_parser.parse(buf)?;
                let _reserved = t_result.unwrap();
                // TODO: ensure this is 0

                let mut number_of_operations: Vec<u32> = vec![];
                let mut function_operations: Vec<Vec<Operations>> = vec![];
                let mut t_value = 0;
                let mut t_operators: Vec<Operations> = vec![];
                loop {
                    match case_parser.parse(buf) {
                        Ok(_w) => {
                            let u_wrapped = uint32_parser.parse(buf)?;
                            let u_value = u_wrapped.unwrap();
                            number_of_operations.push(u_value);
                        },
                        Err(_) => break,
                    }
                }
                match dflt_parser.parse(buf) {
                    Ok(_v) => {
                        let t_wrapped = uint32_parser.parse(buf)?;
                        t_value = t_wrapped.unwrap();
                    },
                    Err(_) => {},
                }
                for op in number_of_operations.clone() {
                    let mut cur_function: Vec<Operations> = vec![];
                    // TODO: There may be an error here...
                    for _i in 0 .. op {
                        let mut data_parser = OperationsP;
                        let data_result = data_parser.parse(buf)?;
                        cur_function.push(data_result.unwrap());
                    }
                    function_operations.push(cur_function);
                }
                // TODO: Same as above, this is problematic.
                for _op in 0 .. t_value {
                    let mut data_parser = OperationsP;
                    let data_result = data_parser.parse(buf)?;
                    t_operators.push(data_result.unwrap());
                }
                let g = Operations::new(
                    "sel".to_string(),
                    number_of_operations,
                    function_operations,
                    t_value,
                    t_operators,
                    DataOperation::new("sel".to_string(), 0.0, [].to_vec()),
                );
                Ok(LocatedVal::new(g, start, buf.get_cursor()))
            },
            Err(e) => return Err(e),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfElse {
    signature:                bool,
    t_value:                  u32,
    u_value:                  u32,
    function_operations_if:   Vec<DataOperation>,
    function_operations_else: Vec<DataOperation>,
}
impl IfElse {
    pub fn new(
        signature: bool, t_value: u32, u_value: u32, function_operations_if: Vec<DataOperation>,
        function_operations_else: Vec<DataOperation>,
    ) -> Self {
        Self {
            signature,
            t_value,
            u_value,
            function_operations_if,
            function_operations_else,
        }
    }
    pub fn number_of_operations(self) -> u32 { self.t_value + self.u_value + 2 }
    pub fn t_value(self) -> u32 { self.t_value }
    pub fn u_value(self) -> u32 { self.u_value }
    pub fn function_operations_if(self) -> Vec<DataOperation> { self.function_operations_if }
    pub fn function_operations_else(self) -> Vec<DataOperation> { self.function_operations_else }
}

pub struct IfElseP;
impl ParsleyParser for IfElseP {
    type T = LocatedVal<Operations>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut if_parser = BinaryMatcher::new(b"if  ");
        let mut else_parser = BinaryMatcher::new(b"else");

        let start = buf.get_cursor();
        let mut if_operations: Vec<Operations> = vec![];
        let mut else_operations: Vec<Operations> = vec![];
        let mut number_of_operations: Vec<u32> = vec![];
        let mut function_operations: Vec<Vec<Operations>> = vec![];
        match if_parser.parse(buf) {
            Ok(_v) => {
                let mut uint32_parser = UInt32P::new(Endian::Big);
                let t_result = uint32_parser.parse(buf)?;
                let t_value = t_result.unwrap();
                let mut u_value = 0;
                let mut u = false;
                match else_parser.parse(buf) {
                    Ok(_w) => {
                        let u_result = uint32_parser.parse(buf)?;
                        u_value = u_result.unwrap();
                        u = true;
                    },
                    Err(_) => {},
                }
                let mut operation = OperationsP;
                let mut counter = 0;
                while counter < t_value {
                    let operation_r = operation.parse(buf)?;
                    let operation_result_clone = operation_r.clone().unwrap();
                    let operation_result = operation_r.unwrap();
                    if_operations.push(operation_result);
                    counter = counter + operation_result_clone.number_of_operations();
                }
                // If else present, then we need to parse U as well
                if u {
                    counter = 0;
                    while counter < u_value {
                        let operation_r = operation.parse(buf)?;
                        let operation_result_clone = operation_r.clone().unwrap();
                        let operation_result = operation_r.unwrap();
                        else_operations.push(operation_result);
                        counter = counter + operation_result_clone.number_of_operations();
                    }
                }
                number_of_operations.push(t_value);
                number_of_operations.push(u_value);
                function_operations.push(if_operations);
                function_operations.push(else_operations);
                let t_operators_empty: Vec<Operations> = vec![];
                let g = Operations::new(
                    "if".to_string(),
                    number_of_operations,
                    function_operations,
                    0,
                    t_operators_empty,
                    DataOperation::new("if".to_string(), 0.0, [].to_vec()),
                );
                //let g = IfElse::new(true, t_value, u_value, if_operations, else_operations);
                Ok(LocatedVal::new(g, start, buf.get_cursor()))
            },
            Err(e) => return Err(e),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct CalcFunction {
    signature:            bool,
    reserved:             u32,
    number_of_operations: u32,
}
impl CalcFunction {
    pub fn new(signature: bool, reserved: u32, number_of_operations: u32) -> Self {
        Self {
            signature,
            reserved,
            number_of_operations,
        }
    }
}

pub struct CalcFunctionP;
impl ParsleyParser for CalcFunctionP {
    type T = LocatedVal<CalcFunction>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut g1 = BinaryMatcher::new(b"func");

        let start = buf.get_cursor();
        let signature = g1.parse(buf)?;

        let mut g2 = UInt32P::new(Endian::Big);
        let reserved = g2.parse(buf)?;
        // This field must be 0
        assert_eq!(reserved.unwrap(), 0);

        let mut g3 = UInt32P::new(Endian::Big);
        let number_of_operations = g3.parse(buf)?;
        // TODO: Not sure if this should be >= 1?

        let mut counter = 0;

        let mut stack: Vec<f32> = vec![];
        let mut instructions: Vec<Operations> = vec![];
        while counter < number_of_operations.unwrap() {
            // This can be an If condition, a Case, or Data Operation.
            let mut data_parser = OperationsP;
            let start = buf.get_cursor();
            let data_result = data_parser.parse(buf)?;
            let data_result_clone = data_result.clone().unwrap();
            let data_result_clone2 = data_result.clone().unwrap();
            let count = data_result.clone().unwrap().number_of_operations();
            instructions.push(data_result_clone2);

            let ret = resolve_operations(data_result_clone, &mut stack);
            if let Err(s) = &ret {
                let err = ErrorKind::GuardError(s.clone());
                let err = LocatedVal::new(err, start, buf.get_cursor());
                return Err(err)
            }
            //println!("{:?} {:?}", data_result_clone, count);
            counter = counter + count;
        }
        let exec = ExecutionTree::new(0, 0, None, instructions, false);
        println!("{:?}", exec);

        let g = CalcFunction::new(
            *signature.val(),
            *reserved.val(),
            *number_of_operations.val(),
        );
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TaggedElement {
    signature: u32,
    offset:    u32,
    size:      u32,
}
impl TaggedElement {
    pub fn new(signature: u32, offset: u32, size: u32) -> Self {
        Self {
            signature,
            offset,
            size,
        }
    }
    pub fn signature(self) -> u32 { self.signature }
    pub fn size(self) -> u32 { self.size }
    pub fn offset(self) -> u32 { self.offset }
}
pub struct TaggedElementP;
impl ParsleyParser for TaggedElementP {
    type T = LocatedVal<TaggedElement>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut gp = UInt32P::new(Endian::Big);

        let start = buf.get_cursor();
        let signature = gp.parse(buf)?;
        let offset = gp.parse(buf)?;
        let size = gp.parse(buf)?;

        let g = TaggedElement::new(*signature.val(), *offset.val(), *size.val());
        Ok(LocatedVal::new(g, start, buf.get_cursor()))
    }
}

// TODO: finish this definition
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Header {
    version:     u8,
    vendorid:    u8,
    guid_prefix: u8,
}
impl Header {
    pub fn new(version: u8, vendorid: u8, guid_prefix: u8) -> Self {
        Self {
            version,
            vendorid,
            guid_prefix,
        }
    }
}

pub struct HeaderP;
impl ParsleyParser for HeaderP {
    type T = LocatedVal<Header>;

    fn parse(&mut self, buf: &mut dyn ParseBufferT) -> ParseResult<Self::T> {
        let mut parser = UInt8P;
        let start = buf.get_cursor();
        let mut counter = 0;
        while counter < 128 {
            let _res = parser.parse(buf)?;
            counter = counter + 1;
        }
        let h = Header::new(0, 0, 0);
        Ok(LocatedVal::new(h, start, buf.get_cursor()))
    }
}
#[cfg(test)]
mod test_iccmax_prim {
    use super::{resolve_operations, DataOperationP};
    use crate::pcore::parsebuffer::{ParseBuffer, ParsleyParser};
    #[test]
    fn test_position_number() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_mpet_element() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_general_element() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_calculator() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_data_operation() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_operations_if() {
        // Test recursive
        assert_eq!(0, 0);
    }
    #[test]
    fn test_operations_sel() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_operations_dataoperation() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_operations_copy() {
        let mut parser = DataOperationP;
        let v = Vec::from("copy\x00\x01\x00\x01".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(6, stack.len());
    }
    #[test]
    fn test_operations_copy_err() {
        let mut parser = DataOperationP;
        let v = Vec::from("copy\x05\x05\x05\x05".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        assert!(resolve_operations(r, &mut stack).is_err());
    }
    #[test]
    fn test_operations_posd() {
        let mut parser = DataOperationP;
        let v = Vec::from("posd\x00\x01\x00\x01".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        assert_eq!(2.0, stack[stack.len() - 1]);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(4, stack.len());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_posd_err() {
        let mut parser = DataOperationP;
        let v = Vec::from("posd\x05\x05\x05\x05".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        assert!(resolve_operations(r, &mut stack).is_err());
    }
    #[test]
    fn test_operations_sequential_functions_sum() {
        let mut parser = DataOperationP;
        let v = Vec::from("sum \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(5.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_sequential_functions_prod() {
        let mut parser = DataOperationP;
        let v = Vec::from("prod\x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(4.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_sequential_functions_min() {
        let mut parser = DataOperationP;
        let v = Vec::from("min \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_sequential_functions_max() {
        let mut parser = DataOperationP;
        let v = Vec::from("max \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(3.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_sequential_functions_and() {
        let mut parser = DataOperationP;
        let v = Vec::from("and \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let r_n = r.clone();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        let mut stack_n: Vec<f32> = vec![];
        stack_n.push(1.0);
        stack_n.push(2.0);
        stack_n.push(0.0);
        assert!(resolve_operations(r_n, &mut stack_n).is_ok());
        assert_eq!(0.0, stack_n.pop().unwrap());
    }
    #[test]
    fn test_operations_sequential_functions_or() {
        let mut parser = DataOperationP;
        let v = Vec::from("or  \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let r_n = r.clone();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.5);
        stack.push(0.0);
        stack.push(0.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        let mut stack_n: Vec<f32> = vec![];
        stack_n.push(0.1);
        stack_n.push(0.2);
        stack_n.push(0.3);
        assert!(resolve_operations(r_n, &mut stack_n).is_ok());
        assert_eq!(0.0, stack_n.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_add() {
        let mut parser = DataOperationP;
        let v = Vec::from("add \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.5);
        stack.push(1.0);
        stack.push(3.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(6.0, stack.pop().unwrap());
        assert_eq!(3.5, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_sub() {
        let mut parser = DataOperationP;
        let v = Vec::from("sub \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(7.5);
        stack.push(7.0);
        stack.push(3.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(4.5, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_mul() {
        let mut parser = DataOperationP;
        let v = Vec::from("mul \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.5);
        stack.push(1.0);
        stack.push(3.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(5.0, stack.pop().unwrap());
        assert_eq!(1.5, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_div() {
        let mut parser = DataOperationP;
        let v = Vec::from("div \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(4.0);
        stack.push(1.0);
        stack.push(4.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.2, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_pow() {
        let mut parser = DataOperationP;
        let v = Vec::from("pow \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(32.0, stack.pop().unwrap());
        assert_eq!(8.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_gama() {
        let mut parser = DataOperationP;
        let v = Vec::from("gama\x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(3.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(27.0, stack.pop().unwrap());
        assert_eq!(8.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_sadd() {
        let mut parser = DataOperationP;
        let v = Vec::from("sadd\x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(5.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(8.0, stack.pop().unwrap());
        assert_eq!(7.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_ssub() {
        let mut parser = DataOperationP;
        let v = Vec::from("ssub\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(-1.0, stack.pop().unwrap());
        assert_eq!(-2.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_smul() {
        let mut parser = DataOperationP;
        let v = Vec::from("smul\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(9.0, stack.pop().unwrap());
        assert_eq!(6.0, stack.pop().unwrap());
        assert_eq!(6.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_sdiv() {
        let mut parser = DataOperationP;
        let v = Vec::from("sdiv\x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.5, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_flip() {
        let mut parser = DataOperationP;
        let v = Vec::from("flip\x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(3.0, stack.pop().unwrap());
        assert_eq!(2.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_flip_err() {
        let mut parser = DataOperationP;
        let v = Vec::from("flip\x05\x05\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        assert!(resolve_operations(r, &mut stack).is_err());
    }

    #[test]
    fn test_operations_vector_lt() {
        let mut parser = DataOperationP;
        let v = Vec::from("lt  \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(1.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_le() {
        let mut parser = DataOperationP;
        let v = Vec::from("le  \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(0.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_eq() {
        let mut parser = DataOperationP;
        let v = Vec::from("eq  \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(3.0);
        stack.push(3.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_gt() {
        let mut parser = DataOperationP;
        let v = Vec::from("gt  \x00\x01\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_ge() {
        let mut parser = DataOperationP;
        let v = Vec::from("ge  \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(2.0);
        stack.push(2.0);
        stack.push(3.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_vmin() {
        let mut parser = DataOperationP;
        let v = Vec::from("vmin\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(2.0);
        stack.push(4.0);
        stack.push(1.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(3.0, stack.pop().unwrap());
    }
    #[test]
    fn test_operations_vector_vmax() {
        let mut parser = DataOperationP;
        let v = Vec::from("vmax\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(2.0);
        stack.push(4.0);
        stack.push(1.0);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(3.0, stack.pop().unwrap());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(4.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_vand() {
        let mut parser = DataOperationP;
        let v = Vec::from("vand\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.0);
        stack.push(0.5);
        stack.push(2.0);
        stack.push(4.0);
        stack.push(0.5);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(0.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_vor() {
        let mut parser = DataOperationP;
        let v = Vec::from("vor \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.0);
        stack.push(0.3);
        stack.push(2.0);
        stack.push(0.0);
        stack.push(0.5);
        stack.push(3.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(0.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_sq() {
        let mut parser = DataOperationP;
        let v = Vec::from("sq  \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(4.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(4.0, stack.pop().unwrap());
        assert_eq!(16.0, stack.pop().unwrap());
        assert_eq!(9.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_abs() {
        let mut parser = DataOperationP;
        let v = Vec::from("abs \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(-4.0);
        stack.push(-2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(4.0, stack.pop().unwrap());
        assert_eq!(3.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_neg() {
        let mut parser = DataOperationP;
        let v = Vec::from("neg \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(-4.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(-2.0, stack.pop().unwrap());
        assert_eq!(4.0, stack.pop().unwrap());
        assert_eq!(-3.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_sqrt() {
        let mut parser = DataOperationP;
        let v = Vec::from("sqrt\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(9.0);
        stack.push(4.0);
        stack.push(1.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(3.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_cbrt() {
        let mut parser = DataOperationP;
        let v = Vec::from("cbrt\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.0);
        stack.push(8.0);
        stack.push(1.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(3.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_flor() {
        let mut parser = DataOperationP;
        let v = Vec::from("flor\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(8.0, stack.pop().unwrap());
        assert_eq!(27.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_ceil() {
        let mut parser = DataOperationP;
        let v = Vec::from("ceil\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(9.0, stack.pop().unwrap());
        assert_eq!(28.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_trunc() {
        let mut parser = DataOperationP;
        let v = Vec::from("trnc\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(8.0, stack.pop().unwrap());
        assert_eq!(27.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_exp() {
        let mut parser = DataOperationP;
        let v = Vec::from("exp \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.1_f32.exp(), stack.pop().unwrap());
        assert_eq!(8.9_f32.exp(), stack.pop().unwrap());
        assert_eq!(27.6_f32.exp(), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_log() {
        let mut parser = DataOperationP;
        let v = Vec::from("log \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.1_f32.log10(), stack.pop().unwrap());
        assert_eq!(8.9_f32.log10(), stack.pop().unwrap());
        assert_eq!(27.6_f32.log10(), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_ln() {
        let mut parser = DataOperationP;
        let v = Vec::from("ln  \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(8.9);
        stack.push(1.1);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(1.1_f32.ln(), stack.pop().unwrap());
        assert_eq!(8.9_f32.ln(), stack.pop().unwrap());
        assert_eq!(27.6_f32.ln(), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_sign() {
        let mut parser = DataOperationP;
        let v = Vec::from("sign\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(27.6);
        stack.push(-8.9);
        stack.push(0.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(0.0, stack.pop().unwrap());
        assert_eq!(-1.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_cb() {
        let mut parser = DataOperationP;
        let v = Vec::from("cb  \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(4.0);
        stack.push(2.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(8.0, stack.pop().unwrap());
        assert_eq!(64.0, stack.pop().unwrap());
        assert_eq!(27.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_rotate_left() {
        let mut parser = DataOperationP;
        let v = Vec::from("rotl\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(1.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(3.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
        assert_eq!(2.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_rotate_left_err() {
        let mut parser = DataOperationP;
        let v = Vec::from("rotl\x05\x05\x05\x05".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        assert!(resolve_operations(r, &mut stack).is_err());
    }

    #[test]
    fn test_operations_vector_rotate_right() {
        let mut parser = DataOperationP;
        let v = Vec::from("rotr\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(3.0);
        stack.push(2.0);
        stack.push(1.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(2.0, stack.pop().unwrap());
        assert_eq!(3.0, stack.pop().unwrap());
        assert_eq!(1.0, stack.pop().unwrap());
    }

    #[test]
    fn test_operations_rotate_right_err() {
        let mut parser = DataOperationP;
        let v = Vec::from("rotr\x05\x05\x05\x05".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        assert!(resolve_operations(r, &mut stack).is_err());
    }

    #[test]
    fn test_operations_vector_sin() {
        let mut parser = DataOperationP;
        let v = Vec::from("sin \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(45.0);
        stack.push(70.0);
        stack.push(120.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::sin(120.0), stack.pop().unwrap());
        assert_eq!(f32::sin(70.0), stack.pop().unwrap());
        assert_eq!(f32::sin(45.0), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_cos() {
        let mut parser = DataOperationP;
        let v = Vec::from("cos \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(45.0);
        stack.push(70.0);
        stack.push(120.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::cos(120.0), stack.pop().unwrap());
        assert_eq!(f32::cos(70.0), stack.pop().unwrap());
        assert_eq!(f32::cos(45.0), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_tan() {
        let mut parser = DataOperationP;
        let v = Vec::from("tan \x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(45.0);
        stack.push(70.0);
        stack.push(120.0);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::tan(120.0), stack.pop().unwrap());
        assert_eq!(f32::tan(70.0), stack.pop().unwrap());
        assert_eq!(f32::tan(45.0), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_asin() {
        let mut parser = DataOperationP;
        let v = Vec::from("asin\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.4);
        stack.push(0.5);
        stack.push(0.27);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::asin(0.27), stack.pop().unwrap());
        assert_eq!(f32::asin(0.5), stack.pop().unwrap());
        assert_eq!(f32::asin(0.4), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_acos() {
        let mut parser = DataOperationP;
        let v = Vec::from("acos\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.28);
        stack.push(0.78);
        stack.push(0.4);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::acos(0.4), stack.pop().unwrap());
        assert_eq!(f32::acos(0.78), stack.pop().unwrap());
        assert_eq!(f32::acos(0.28), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_atan() {
        let mut parser = DataOperationP;
        let v = Vec::from("atan\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.5);
        stack.push(0.3);
        stack.push(0.78);
        assert!(resolve_operations(r, &mut stack).is_ok());
        assert_eq!(f32::atan(0.78), stack.pop().unwrap());
        assert_eq!(f32::atan(0.3), stack.pop().unwrap());
        assert_eq!(f32::atan(0.5), stack.pop().unwrap());
    }

    #[test]
    fn test_operations_vector_atan2() {
        let mut parser = DataOperationP;
        let v = Vec::from("atn2\x00\x02\x00\x00".as_bytes());
        let mut parsebuffer = ParseBuffer::new(v);
        let result = parser.parse(&mut parsebuffer);
        let r = result.unwrap().unwrap();
        let mut stack: Vec<f32> = vec![];
        stack.push(0.5);
        stack.push(0.3);
        stack.push(0.78);
        stack.push(0.5);
        stack.push(0.5);
        stack.push(0.5);
        assert!(resolve_operations(r, &mut stack).is_ok());
        let y = 0.5f32;
        assert_eq!(y.atan2(0.78), stack.pop().unwrap());
        assert_eq!(y.atan2(0.3), stack.pop().unwrap());
        assert_eq!(y.atan2(0.5), stack.pop().unwrap());
    }
}
