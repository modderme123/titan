use std::str::FromStr;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::map_opt;
use nom::IResult;
use nom::sequence::preceded;
use num_traits::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, ToPrimitive, FromPrimitive)]
pub enum RegisterSlot {
    Zero = 0,
    AssemblerTemporary = 1,
    Value0 = 2,
    Value1 = 3,
    Parameter0 = 4,
    Parameter1 = 5,
    Parameter2 = 6,
    Parameter3 = 7,
    Temporary0 = 8,
    Temporary1 = 9,
    Temporary2 = 10,
    Temporary3 = 11,
    Temporary4 = 12,
    Temporary5 = 13,
    Temporary6 = 14,
    Temporary7 = 15,
    Saved0 = 16,
    Saved1 = 17,
    Saved2 = 18,
    Saved3 = 19,
    Saved4 = 20,
    Saved5 = 21,
    Saved6 = 22,
    Saved7 = 23,
    Temporary8 = 24,
    Temporary9 = 25,
    Kernel0 = 26,
    Kernel1 = 27,
    GeneralPointer = 28,
    StackPointer = 29,
    FramePointer = 30,
    ReturnAddress = 31,
}

impl RegisterSlot {
    fn from_string(input: &str) -> Option<RegisterSlot> {
        Some(match input {
            "$zero" => RegisterSlot::Zero,
            "$at" => RegisterSlot::AssemblerTemporary,
            "$v0" => RegisterSlot::Value0,
            "$v1" => RegisterSlot::Value1,
            "$a0" => RegisterSlot::Parameter0,
            "$a1" => RegisterSlot::Parameter1,
            "$a2" => RegisterSlot::Parameter2,
            "$a3" => RegisterSlot::Parameter3,
            "$t0" => RegisterSlot::Temporary0,
            "$t1" => RegisterSlot::Temporary1,
            "$t2" => RegisterSlot::Temporary2,
            "$t3" => RegisterSlot::Temporary3,
            "$t4" => RegisterSlot::Temporary4,
            "$t5" => RegisterSlot::Temporary5,
            "$t6" => RegisterSlot::Temporary6,
            "$t7" => RegisterSlot::Temporary7,
            "$s0" => RegisterSlot::Saved0,
            "$s1" => RegisterSlot::Saved1,
            "$s2" => RegisterSlot::Saved2,
            "$s3" => RegisterSlot::Saved3,
            "$s4" => RegisterSlot::Saved4,
            "$s5" => RegisterSlot::Saved5,
            "$s6" => RegisterSlot::Saved6,
            "$s7" => RegisterSlot::Saved7,
            "$t8" => RegisterSlot::Temporary8,
            "$t9" => RegisterSlot::Temporary9,
            "$k0" => RegisterSlot::Kernel0,
            "$k1" => RegisterSlot::Kernel1,
            "$gp" => RegisterSlot::GeneralPointer,
            "$sp" => RegisterSlot::StackPointer,
            "$fp" => RegisterSlot::FramePointer,
            "$ra" => RegisterSlot::ReturnAddress,

            _ => return None
        })
    }
}

fn register_named(input: &str) -> IResult<&str, RegisterSlot> {
    map_opt(alpha1, |text: &str| RegisterSlot::from_string(text))(input)
}

fn register_numbered(input: &str) -> IResult<&str, RegisterSlot> {
    map_opt(digit1, |text| {
        u32::from_str(text).ok()
            .map_or(None, |number| FromPrimitive::from_u32(number))
    })(input)
}

pub fn register(input: &str) -> IResult<&str, RegisterSlot> {
    preceded(char('$'), alt((
        register_named,
        register_numbered
    )))(input)
}