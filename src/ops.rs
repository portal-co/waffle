//! Operators.

use wasmparser::{Ieee32, Ieee64, MemoryImmediate, Type};

use crate::{FuncId, GlobalId, LocalId, MemoryId, TableId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Memory {
    pub align: u8,
    pub offset: u64,
    pub memory: MemoryId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    Unreachable,
    Nop,

    Call { function_index: FuncId },
    CallIndirect { index: FuncId, table_index: TableId },
    Return,
    LocalSet { local_index: LocalId },
    LocalTee { local_index: LocalId },
    LocalGet { local_index: LocalId },
    Select,
    TypedSelect { ty: Type },
    GlobalGet { global_index: GlobalId },
    GlobalSet { global_index: GlobalId },

    I32Load { memory: Memory },
    I64Load { memory: Memory },
    F32Load { memory: Memory },
    F64Load { memory: Memory },
    I32Load8S { memory: Memory },
    I32Load8U { memory: Memory },
    I32Load16S { memory: Memory },
    I32Load16U { memory: Memory },
    I64Load8S { memory: Memory },
    I64Load8U { memory: Memory },
    I64Load16S { memory: Memory },
    I64Load16U { memory: Memory },
    I64Load32S { memory: Memory },
    I64Load32U { memory: Memory },

    I32Store { memory: Memory },
    I64Store { memory: Memory },
    F32Store { memory: Memory },
    F64Store { memory: Memory },
    I32Store8 { memory: Memory },
    I32Store16 { memory: Memory },
    I64Store8 { memory: Memory },
    I64Store16 { memory: Memory },
    I64Store32 { memory: Memory },

    I32Const { value: i32 },
    I64Const { value: i64 },
    F32Const { value: Ieee32 },
    F64Const { value: Ieee64 },

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eqz,

    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtU,
    I64GtS,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32Popcnt,

    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64Popcnt,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32ReinterpretF32,
    I64ReinterpretF64,
    TableGet { table: TableId },
    TableSet { table: TableId },
    TableGrow { table: TableId },
    TableSize { table: TableId },
    MemorySize { mem: MemoryId },
    MemoryGrow { mem: MemoryId },
}

impl<'a> std::convert::TryFrom<wasmparser::Operator<'a>> for Operator {
    fn try_from(op: &wasmparser::Operator<'a>) -> Option<Operator> {
        match op {
            &wasmparser::Operator::Unreachable => Some(Operator::Unreachable),
            &wasmparser::Operator::Nop => Some(Operator::Nop),
            &wasmparser::Operator::Call { function_index } => Some(Operator::Call {
                function_index: function_index as usize,
            }),
            &wasmparser::Operator::CallIndirect { index, table_index } => {
                Some(Operator::CallIndirect {
                    index: index as usize,
                    table_index,
                })
            }
            &wasmparser::Operator::Return => Some(Operator::Return),
            &wasmparser::Operator::LocalSet { local_index } => {
                Some(Operator::LocalSet { local_index })
            }
            &wasmparser::Operator::LocalTee { local_index } => {
                Some(Operator::LocalTee { local_index })
            }
            &wasmparser::Operator::LocalGet { local_index } => {
                Some(Operator::LocalGet { local_index })
            }
            &wasmparser::Operator::Select => Some(Operator::Select),
            &wasmparser::Operator::TypedSelect { ty } => Some(Operator::TypedSelect { ty }),
            &wasmparser::Operator::GlobalGet { global_index } => {
                Some(Operator::GlobalGet { global_index })
            }
            &wasmparser::Operator::GlobalSet { global_index } => {
                Some(Operator::GlobalSet { global_index })
            }
            &wasmparser::Operator::I32Load { memarg } => Some(Operator::I32Load {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load { memarg } => Some(Operator::I64Load {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::F32Load { memarg } => Some(Operator::F32Load {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::F64Load { memarg } => Some(Operator::F64Load {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Load8S { memarg } => Some(Operator::I32Load8S {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Load8U { memarg } => Some(Operator::I32Load8U {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Load16S { memarg } => Some(Operator::I32Load16S {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Load16U { memarg } => Some(Operator::I32Load16U {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load8S { memarg } => Some(Operator::I64Load8S {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load8U { memarg } => Some(Operator::I64Load8U {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load16S { memarg } => Some(Operator::I64Load16S {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load16U { memarg } => Some(Operator::I64Load16U {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load32S { memarg } => Some(Operator::I64Load32S {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Load32U { memarg } => Some(Operator::I64Load32U {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Store { memarg } => Some(Operator::I32Store {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Store { memarg } => Some(Operator::I64Store {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::F32Store { memarg } => Some(Operator::F32Store {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::F64Store { memarg } => Some(Operator::F64Store {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Store8 { memarg } => Some(Operator::I32Store8 {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Store16 { memarg } => Some(Operator::I32Store16 {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Store8 { memarg } => Some(Operator::I64Store8 {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Store16 { memarg } => Some(Operator::I64Store16 {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I64Store32 { memarg } => Some(Operator::I64Store32 {
                memory: memarg.into(),
            }),
            &wasmparser::Operator::I32Const { value } => Some(Operator::I32Const { value }),
            &wasmparser::Operator::I64Const { value } => Some(Operator::I64Const { value }),
            &wasmparser::Operator::F32Const { value } => Some(Operator::F32Const { value }),
            &wasmparser::Operator::F64Const { value } => Some(Operator::F64Const { value }),
            &wasmparser::Operator::I32Eqz => Some(Operator::I32Eqz),
            &wasmparser::Operator::I32Eq => Some(Operator::I32Eq),
            &wasmparser::Operator::I32Ne => Some(Operator::I32Ne),
            &wasmparser::Operator::I32LtS => Some(Operator::I32LtS),
            &wasmparser::Operator::I32LtU => Some(Operator::I32LtU),
            &wasmparser::Operator::I32GtS => Some(Operator::I32GtS),
            &wasmparser::Operator::I32GtU => Some(Operator::I32GtU),
            &wasmparser::Operator::I32LeS => Some(Operator::I32LeS),
            &wasmparser::Operator::I32LeU => Some(Operator::I32LeU),
            &wasmparser::Operator::I32GeS => Some(Operator::I32GeS),
            &wasmparser::Operator::I32GeU => Some(Operator::I32GeU),
            &wasmparser::Operator::I64Eqz => Some(Operator::I64Eqz),
            &wasmparser::Operator::I64Eq => Some(Operator::I64Eq),
            &wasmparser::Operator::I64Ne => Some(Operator::I64Ne),
            &wasmparser::Operator::I64LtS => Some(Operator::I64LtS),
            &wasmparser::Operator::I64LtU => Some(Operator::I64LtU),
            &wasmparser::Operator::I64GtU => Some(Operator::I64GtU),
            &wasmparser::Operator::I64GtS => Some(Operator::I64GtS),
            &wasmparser::Operator::I64LeS => Some(Operator::I64LeS),
            &wasmparser::Operator::I64LeU => Some(Operator::I64LeU),
            &wasmparser::Operator::I64GeS => Some(Operator::I64GeS),
            &wasmparser::Operator::I64GeU => Some(Operator::I64GeU),
            &wasmparser::Operator::F32Eq => Some(Operator::F32Eq),
            &wasmparser::Operator::F32Ne => Some(Operator::F32Ne),
            &wasmparser::Operator::F32Lt => Some(Operator::F32Lt),
            &wasmparser::Operator::F32Gt => Some(Operator::F32Gt),
            &wasmparser::Operator::F32Le => Some(Operator::F32Le),
            &wasmparser::Operator::F32Ge => Some(Operator::F32Ge),
            &wasmparser::Operator::F64Eq => Some(Operator::F64Eq),
            &wasmparser::Operator::F64Ne => Some(Operator::F64Ne),
            &wasmparser::Operator::F64Lt => Some(Operator::F64Lt),
            &wasmparser::Operator::F64Gt => Some(Operator::F64Gt),
            &wasmparser::Operator::F64Le => Some(Operator::F64Le),
            &wasmparser::Operator::F64Ge => Some(Operator::F64Ge),
            &wasmparser::Operator::I32Clz => Some(Operator::I32Clz),
            &wasmparser::Operator::I32Ctz => Some(Operator::I32Ctz),
            &wasmparser::Operator::I32Popcnt => Some(Operator::I32Popcnt),
            &wasmparser::Operator::I32Add => Some(Operator::I32Add),
            &wasmparser::Operator::I32Sub => Some(Operator::I32Sub),
            &wasmparser::Operator::I32Mul => Some(Operator::I32Mul),
            &wasmparser::Operator::I32DivS => Some(Operator::I32DivS),
            &wasmparser::Operator::I32DivU => Some(Operator::I32DivU),
            &wasmparser::Operator::I32RemS => Some(Operator::I32RemS),
            &wasmparser::Operator::I32RemU => Some(Operator::I32RemU),
            &wasmparser::Operator::I32And => Some(Operator::I32And),
            &wasmparser::Operator::I32Or => Some(Operator::I32Or),
            &wasmparser::Operator::I32Xor => Some(Operator::I32Xor),
            &wasmparser::Operator::I32Shl => Some(Operator::I32Shl),
            &wasmparser::Operator::I32ShrS => Some(Operator::I32ShrS),
            &wasmparser::Operator::I32ShrU => Some(Operator::I32ShrU),
            &wasmparser::Operator::I32Rotl => Some(Operator::I32Rotl),
            &wasmparser::Operator::I32Rotr => Some(Operator::I32Rotr),
            &wasmparser::Operator::I64Clz => Some(Operator::I64Clz),
            &wasmparser::Operator::I64Ctz => Some(Operator::I64Ctz),
            &wasmparser::Operator::I64Popcnt => Some(Operator::I64Popcnt),
            &wasmparser::Operator::I64Add => Some(Operator::I64Add),
            &wasmparser::Operator::I64Sub => Some(Operator::I64Sub),
            &wasmparser::Operator::I64Mul => Some(Operator::I64Mul),
            &wasmparser::Operator::I64DivS => Some(Operator::I64DivS),
            &wasmparser::Operator::I64DivU => Some(Operator::I64DivU),
            &wasmparser::Operator::I64RemS => Some(Operator::I64RemS),
            &wasmparser::Operator::I64RemU => Some(Operator::I64RemU),
            &wasmparser::Operator::I64And => Some(Operator::I64And),
            &wasmparser::Operator::I64Or => Some(Operator::I64Or),
            &wasmparser::Operator::I64Xor => Some(Operator::I64Xor),
            &wasmparser::Operator::I64Shl => Some(Operator::I64Shl),
            &wasmparser::Operator::I64ShrS => Some(Operator::I64ShrS),
            &wasmparser::Operator::I64ShrU => Some(Operator::I64ShrU),
            &wasmparser::Operator::I64Rotl => Some(Operator::I64Rotl),
            &wasmparser::Operator::I64Rotr => Some(Operator::I64Rotr),
            &wasmparser::Operator::F32Abs => Some(Operator::F32Abs),
            &wasmparser::Operator::F32Neg => Some(Operator::F32Neg),
            &wasmparser::Operator::F32Ceil => Some(Operator::F32Ceil),
            &wasmparser::Operator::F32Floor => Some(Operator::F32Floor),
            &wasmparser::Operator::F32Trunc => Some(Operator::F32Trunc),
            &wasmparser::Operator::F32Nearest => Some(Operator::F32Nearest),
            &wasmparser::Operator::F32Sqrt => Some(Operator::F32Sqrt),
            &wasmparser::Operator::F32Add => Some(Operator::F32Add),
            &wasmparser::Operator::F32Sub => Some(Operator::F32Sub),
            &wasmparser::Operator::F32Mul => Some(Operator::F32Mul),
            &wasmparser::Operator::F32Div => Some(Operator::F32Div),
            &wasmparser::Operator::F32Min => Some(Operator::F32Min),
            &wasmparser::Operator::F32Max => Some(Operator::F32Max),
            &wasmparser::Operator::F32Copysign => Some(Operator::F32Copysign),
            &wasmparser::Operator::F64Abs => Some(Operator::F64Abs),
            &wasmparser::Operator::F64Neg => Some(Operator::F64Neg),
            &wasmparser::Operator::F64Ceil => Some(Operator::F64Ceil),
            &wasmparser::Operator::F64Floor => Some(Operator::F64Floor),
            &wasmparser::Operator::F64Trunc => Some(Operator::F64Trunc),
            &wasmparser::Operator::F64Nearest => Some(Operator::F64Nearest),
            &wasmparser::Operator::F64Sqrt => Some(Operator::F64Sqrt),
            &wasmparser::Operator::F64Add => Some(Operator::F64Add),
            &wasmparser::Operator::F64Sub => Some(Operator::F64Sub),
            &wasmparser::Operator::F64Mul => Some(Operator::F64Mul),
            &wasmparser::Operator::F64Div => Some(Operator::F64Div),
            &wasmparser::Operator::F64Min => Some(Operator::F64Min),
            &wasmparser::Operator::F64Max => Some(Operator::F64Max),
            &wasmparser::Operator::F64Copysign => Some(Operator::F64Copysign),
            &wasmparser::Operator::I32WrapI64 => Some(Operator::I32WrapI64),
            &wasmparser::Operator::I32TruncF32S => Some(Operator::I32TruncF32S),
            &wasmparser::Operator::I32TruncF32U => Some(Operator::I32TruncF32U),
            &wasmparser::Operator::I32TruncF64S => Some(Operator::I32TruncF64S),
            &wasmparser::Operator::I32TruncF64U => Some(Operator::I32TruncF64U),
            &wasmparser::Operator::I64ExtendI32S => Some(Operator::I64ExtendI32S),
            &wasmparser::Operator::I64ExtendI32U => Some(Operator::I64ExtendI32U),
            &wasmparser::Operator::I64TruncF32S => Some(Operator::I64TruncF32S),
            &wasmparser::Operator::I64TruncF32U => Some(Operator::I64TruncF32U),
            &wasmparser::Operator::I64TruncF64S => Some(Operator::I64TruncF64S),
            &wasmparser::Operator::I64TruncF64U => Some(Operator::I64TruncF64U),
            &wasmparser::Operator::F32ConvertI32S => Some(Operator::F32ConvertI32S),
            &wasmparser::Operator::F32ConvertI32U => Some(Operator::F32ConvertI32U),
            &wasmparser::Operator::F32ConvertI64S => Some(Operator::F32ConvertI64S),
            &wasmparser::Operator::F32ConvertI64U => Some(Operator::F32ConvertI64U),
            &wasmparser::Operator::F32DemoteF64 => Some(Operator::F32DemoteF64),
            &wasmparser::Operator::F64ConvertI32S => Some(Operator::F64ConvertI32S),
            &wasmparser::Operator::F64ConvertI32U => Some(Operator::F64ConvertI32U),
            &wasmparser::Operator::F64ConvertI64S => Some(Operator::F64ConvertI64S),
            &wasmparser::Operator::F64ConvertI64U => Some(Operator::F64ConvertI64U),
            &wasmparser::Operator::F64PromoteF32 => Some(Operator::F64PromoteF32),
            &wasmparser::Operator::I32Extend8S => Some(Operator::I32Extend8S),
            &wasmparser::Operator::I32Extend16S => Some(Operator::I32Extend16S),
            &wasmparser::Operator::I64Extend8S => Some(Operator::I64Extend8S),
            &wasmparser::Operator::I64Extend16S => Some(Operator::I64Extend16S),
            &wasmparser::Operator::I64Extend32S => Some(Operator::I64Extend32S),
            &wasmparser::Operator::I32TruncSatF32S => Some(Operator::I32TruncSatF32S),
            &wasmparser::Operator::I32TruncSatF32U => Some(Operator::I32TruncSatF32U),
            &wasmparser::Operator::I32TruncSatF64S => Some(Operator::I32TruncSatF64S),
            &wasmparser::Operator::I32TruncSatF64U => Some(Operator::I32TruncSatF64U),
            &wasmparser::Operator::I64TruncSatF32S => Some(Operator::I64TruncSatF32S),
            &wasmparser::Operator::I64TruncSatF32U => Some(Operator::I64TruncSatF32U),
            &wasmparser::Operator::I64TruncSatF64S => Some(Operator::I64TruncSatF64S),
            &wasmparser::Operator::I64TruncSatF64U => Some(Operator::I64TruncSatF64U),
            &wasmparser::Operator::F32ReinterpretI32 => Some(Operator::F32ReinterpretI32),
            &wasmparser::Operator::F64ReinterpretI64 => Some(Operator::F64ReinterpretI64),
            &wasmparser::Operator::I32ReinterpretF32 => Some(Operator::I32ReinterpretF32),
            &wasmparser::Operator::I64ReinterpretF64 => Some(Operator::I64ReinterpretF64),
            &wasmparser::Operator::TableGet { table } => Some(Operator::TableGet { table }),
            &wasmparser::Operator::TableSet { table } => Some(Operator::TableSet { table }),
            &wasmparser::Operator::TableGrow { table } => Some(Operator::TableGrow { table }),
            &wasmparser::Operator::TableSize { table } => Some(Operator::TableSize { table }),
            &wasmparser::Operator::MemorySize { mem, .. } => Some(Operator::MemorySize { mem }),
            &wasmparser::Operator::MemoryGrow { mem, .. } => Some(Operator::MemoryGrow { mem }),
        }
    }
}

impl<'a> std::convert::Into<wasmparser::Operator<'a>> for Operator {
    fn into(self) -> wasmparser::Operator<'a> {
        match op {
            &Operator::Unreachable => wasmparser::Operator::Unreachable,
            &Operator::Nop => wasmparser::Operator::Nop,
            &Operator::Call { function_index } => wasmparser::Operator::Call {
                function_index: function_index as u32,
            },
            &Operator::CallIndirect { index, table_index } => wasmparser::Operator::CallIndirect {
                index: index as u32,
                table_index: table_index as u32,
            },
            &Operator::Return => wasmparser::Operator::Return,
            &Operator::LocalSet { local_index } => wasmparser::Operator::LocalSet { local_index },
            &Operator::LocalTee { local_index } => wasmparser::Operator::LocalTee { local_index },
            &Operator::LocalGet { local_index } => wasmparser::Operator::LocalGet { local_index },
            &Operator::Select => wasmparser::Operator::Select,
            &Operator::TypedSelect { ty } => wasmparser::Operator::TypedSelect { ty },
            &Operator::GlobalGet { global_index } => {
                wasmparser::Operator::GlobalGet { global_index }
            }
            &Operator::GlobalSet { global_index } => {
                wasmparser::Operator::GlobalSet { global_index }
            }
            &Operator::I32Load { memory } => wasmparser::Operator::I32Load {
                memarg: memory.into(),
            },
            &Operator::I64Load { memory } => wasmparser::Operator::I64Load {
                memarg: memory.into(),
            },
            &Operator::F32Load { memory } => wasmparser::Operator::F32Load {
                memarg: memory.into(),
            },
            &Operator::F64Load { memory } => wasmparser::Operator::F64Load {
                memarg: memory.into(),
            },
            &Operator::I32Load8S { memory } => wasmparser::Operator::I32Load8S {
                memarg: memory.into(),
            },
            &Operator::I32Load8U { memory } => wasmparser::Operator::I32Load8U {
                memarg: memory.into(),
            },
            &Operator::I32Load16S { memory } => wasmparser::Operator::I32Load16S {
                memarg: memory.into(),
            },
            &Operator::I32Load16U { memory } => wasmparser::Operator::I32Load16U {
                memarg: memory.into(),
            },
            &Operator::I64Load8S { memory } => wasmparser::Operator::I64Load8S {
                memarg: memory.into(),
            },
            &Operator::I64Load8U { memory } => wasmparser::Operator::I64Load8U {
                memarg: memory.into(),
            },
            &Operator::I64Load16S { memory } => wasmparser::Operator::I64Load16S {
                memarg: memory.into(),
            },
            &Operator::I64Load16U { memory } => wasmparser::Operator::I64Load16U {
                memarg: memory.into(),
            },
            &Operator::I64Load32S { memory } => wasmparser::Operator::I64Load32S {
                memarg: memory.into(),
            },
            &Operator::I64Load32U { memory } => wasmparser::Operator::I64Load32U {
                memarg: memory.into(),
            },
            &Operator::I32Store { memory } => wasmparser::Operator::I32Store {
                memarg: memory.into(),
            },
            &Operator::I64Store { memory } => wasmparser::Operator::I64Store {
                memarg: memory.into(),
            },
            &Operator::F32Store { memory } => wasmparser::Operator::F32Store {
                memarg: memory.into(),
            },
            &Operator::F64Store { memory } => wasmparser::Operator::F64Store {
                memarg: memory.into(),
            },
            &Operator::I32Store8 { memory } => wasmparser::Operator::I32Store8 {
                memarg: memory.into(),
            },
            &Operator::I32Store16 { memory } => wasmparser::Operator::I32Store16 {
                memarg: memory.into(),
            },
            &Operator::I64Store8 { memory } => wasmparser::Operator::I64Store8 {
                memarg: memory.into(),
            },
            &Operator::I64Store16 { memory } => wasmparser::Operator::I64Store16 {
                memarg: memory.into(),
            },
            &Operator::I64Store32 { memory } => wasmparser::Operator::I64Store32 {
                memarg: memory.into(),
            },
            &Operator::I32Const { value } => wasmparser::Operator::I32Const { value },
            &Operator::I64Const { value } => wasmparser::Operator::I64Const { value },
            &Operator::F32Const { value } => wasmparser::Operator::F32Const { value },
            &Operator::F64Const { value } => wasmparser::Operator::F64Const { value },
            &Operator::I32Eqz => wasmparser::Operator::I32Eqz,
            &Operator::I32Eq => wasmparser::Operator::I32Eq,
            &Operator::I32Ne => wasmparser::Operator::I32Ne,
            &Operator::I32LtS => wasmparser::Operator::I32LtS,
            &Operator::I32LtU => wasmparser::Operator::I32LtU,
            &Operator::I32GtS => wasmparser::Operator::I32GtS,
            &Operator::I32GtU => wasmparser::Operator::I32GtU,
            &Operator::I32LeS => wasmparser::Operator::I32LeS,
            &Operator::I32LeU => wasmparser::Operator::I32LeU,
            &Operator::I32GeS => wasmparser::Operator::I32GeS,
            &Operator::I32GeU => wasmparser::Operator::I32GeU,
            &Operator::I64Eqz => wasmparser::Operator::I64Eqz,
            &Operator::I64Eq => wasmparser::Operator::I64Eq,
            &Operator::I64Ne => wasmparser::Operator::I64Ne,
            &Operator::I64LtS => wasmparser::Operator::I64LtS,
            &Operator::I64LtU => wasmparser::Operator::I64LtU,
            &Operator::I64GtU => wasmparser::Operator::I64GtU,
            &Operator::I64GtS => wasmparser::Operator::I64GtS,
            &Operator::I64LeS => wasmparser::Operator::I64LeS,
            &Operator::I64LeU => wasmparser::Operator::I64LeU,
            &Operator::I64GeS => wasmparser::Operator::I64GeS,
            &Operator::I64GeU => wasmparser::Operator::I64GeU,
            &Operator::F32Eq => wasmparser::Operator::F32Eq,
            &Operator::F32Ne => wasmparser::Operator::F32Ne,
            &Operator::F32Lt => wasmparser::Operator::F32Lt,
            &Operator::F32Gt => wasmparser::Operator::F32Gt,
            &Operator::F32Le => wasmparser::Operator::F32Le,
            &Operator::F32Ge => wasmparser::Operator::F32Ge,
            &Operator::F64Eq => wasmparser::Operator::F64Eq,
            &Operator::F64Ne => wasmparser::Operator::F64Ne,
            &Operator::F64Lt => wasmparser::Operator::F64Lt,
            &Operator::F64Gt => wasmparser::Operator::F64Gt,
            &Operator::F64Le => wasmparser::Operator::F64Le,
            &Operator::F64Ge => wasmparser::Operator::F64Ge,
            &Operator::I32Clz => wasmparser::Operator::I32Clz,
            &Operator::I32Ctz => wasmparser::Operator::I32Ctz,
            &Operator::I32Popcnt => wasmparser::Operator::I32Popcnt,
            &Operator::I32Add => wasmparser::Operator::I32Add,
            &Operator::I32Sub => wasmparser::Operator::I32Sub,
            &Operator::I32Mul => wasmparser::Operator::I32Mul,
            &Operator::I32DivS => wasmparser::Operator::I32DivS,
            &Operator::I32DivU => wasmparser::Operator::I32DivU,
            &Operator::I32RemS => wasmparser::Operator::I32RemS,
            &Operator::I32RemU => wasmparser::Operator::I32RemU,
            &Operator::I32And => wasmparser::Operator::I32And,
            &Operator::I32Or => wasmparser::Operator::I32Or,
            &Operator::I32Xor => wasmparser::Operator::I32Xor,
            &Operator::I32Shl => wasmparser::Operator::I32Shl,
            &Operator::I32ShrS => wasmparser::Operator::I32ShrS,
            &Operator::I32ShrU => wasmparser::Operator::I32ShrU,
            &Operator::I32Rotl => wasmparser::Operator::I32Rotl,
            &Operator::I32Rotr => wasmparser::Operator::I32Rotr,
            &Operator::I64Clz => wasmparser::Operator::I64Clz,
            &Operator::I64Ctz => wasmparser::Operator::I64Ctz,
            &Operator::I64Popcnt => wasmparser::Operator::I64Popcnt,
            &Operator::I64Add => wasmparser::Operator::I64Add,
            &Operator::I64Sub => wasmparser::Operator::I64Sub,
            &Operator::I64Mul => wasmparser::Operator::I64Mul,
            &Operator::I64DivS => wasmparser::Operator::I64DivS,
            &Operator::I64DivU => wasmparser::Operator::I64DivU,
            &Operator::I64RemS => wasmparser::Operator::I64RemS,
            &Operator::I64RemU => wasmparser::Operator::I64RemU,
            &Operator::I64And => wasmparser::Operator::I64And,
            &Operator::I64Or => wasmparser::Operator::I64Or,
            &Operator::I64Xor => wasmparser::Operator::I64Xor,
            &Operator::I64Shl => wasmparser::Operator::I64Shl,
            &Operator::I64ShrS => wasmparser::Operator::I64ShrS,
            &Operator::I64ShrU => wasmparser::Operator::I64ShrU,
            &Operator::I64Rotl => wasmparser::Operator::I64Rotl,
            &Operator::I64Rotr => wasmparser::Operator::I64Rotr,
            &Operator::F32Abs => wasmparser::Operator::F32Abs,
            &Operator::F32Neg => wasmparser::Operator::F32Neg,
            &Operator::F32Ceil => wasmparser::Operator::F32Ceil,
            &Operator::F32Floor => wasmparser::Operator::F32Floor,
            &Operator::F32Trunc => wasmparser::Operator::F32Trunc,
            &Operator::F32Nearest => wasmparser::Operator::F32Nearest,
            &Operator::F32Sqrt => wasmparser::Operator::F32Sqrt,
            &Operator::F32Add => wasmparser::Operator::F32Add,
            &Operator::F32Sub => wasmparser::Operator::F32Sub,
            &Operator::F32Mul => wasmparser::Operator::F32Mul,
            &Operator::F32Div => wasmparser::Operator::F32Div,
            &Operator::F32Min => wasmparser::Operator::F32Min,
            &Operator::F32Max => wasmparser::Operator::F32Max,
            &Operator::F32Copysign => wasmparser::Operator::F32Copysign,
            &Operator::F64Abs => wasmparser::Operator::F64Abs,
            &Operator::F64Neg => wasmparser::Operator::F64Neg,
            &Operator::F64Ceil => wasmparser::Operator::F64Ceil,
            &Operator::F64Floor => wasmparser::Operator::F64Floor,
            &Operator::F64Trunc => wasmparser::Operator::F64Trunc,
            &Operator::F64Nearest => wasmparser::Operator::F64Nearest,
            &Operator::F64Sqrt => wasmparser::Operator::F64Sqrt,
            &Operator::F64Add => wasmparser::Operator::F64Add,
            &Operator::F64Sub => wasmparser::Operator::F64Sub,
            &Operator::F64Mul => wasmparser::Operator::F64Mul,
            &Operator::F64Div => wasmparser::Operator::F64Div,
            &Operator::F64Min => wasmparser::Operator::F64Min,
            &Operator::F64Max => wasmparser::Operator::F64Max,
            &Operator::F64Copysign => wasmparser::Operator::F64Copysign,
            &Operator::I32WrapI64 => wasmparser::Operator::I32WrapI64,
            &Operator::I32TruncF32S => wasmparser::Operator::I32TruncF32S,
            &Operator::I32TruncF32U => wasmparser::Operator::I32TruncF32U,
            &Operator::I32TruncF64S => wasmparser::Operator::I32TruncF64S,
            &Operator::I32TruncF64U => wasmparser::Operator::I32TruncF64U,
            &Operator::I64ExtendI32S => wasmparser::Operator::I64ExtendI32S,
            &Operator::I64ExtendI32U => wasmparser::Operator::I64ExtendI32U,
            &Operator::I64TruncF32S => wasmparser::Operator::I64TruncF32S,
            &Operator::I64TruncF32U => wasmparser::Operator::I64TruncF32U,
            &Operator::I64TruncF64S => wasmparser::Operator::I64TruncF64S,
            &Operator::I64TruncF64U => wasmparser::Operator::I64TruncF64U,
            &Operator::F32ConvertI32S => wasmparser::Operator::F32ConvertI32S,
            &Operator::F32ConvertI32U => wasmparser::Operator::F32ConvertI32U,
            &Operator::F32ConvertI64S => wasmparser::Operator::F32ConvertI64S,
            &Operator::F32ConvertI64U => wasmparser::Operator::F32ConvertI64U,
            &Operator::F32DemoteF64 => wasmparser::Operator::F32DemoteF64,
            &Operator::F64ConvertI32S => wasmparser::Operator::F64ConvertI32S,
            &Operator::F64ConvertI32U => wasmparser::Operator::F64ConvertI32U,
            &Operator::F64ConvertI64S => wasmparser::Operator::F64ConvertI64S,
            &Operator::F64ConvertI64U => wasmparser::Operator::F64ConvertI64U,
            &Operator::F64PromoteF32 => wasmparser::Operator::F64PromoteF32,
            &Operator::I32Extend8S => wasmparser::Operator::I32Extend8S,
            &Operator::I32Extend16S => wasmparser::Operator::I32Extend16S,
            &Operator::I64Extend8S => wasmparser::Operator::I64Extend8S,
            &Operator::I64Extend16S => wasmparser::Operator::I64Extend16S,
            &Operator::I64Extend32S => wasmparser::Operator::I64Extend32S,
            &Operator::I32TruncSatF32S => wasmparser::Operator::I32TruncSatF32S,
            &Operator::I32TruncSatF32U => wasmparser::Operator::I32TruncSatF32U,
            &Operator::I32TruncSatF64S => wasmparser::Operator::I32TruncSatF64S,
            &Operator::I32TruncSatF64U => wasmparser::Operator::I32TruncSatF64U,
            &Operator::I64TruncSatF32S => wasmparser::Operator::I64TruncSatF32S,
            &Operator::I64TruncSatF32U => wasmparser::Operator::I64TruncSatF32U,
            &Operator::I64TruncSatF64S => wasmparser::Operator::I64TruncSatF64S,
            &Operator::I64TruncSatF64U => wasmparser::Operator::I64TruncSatF64U,
            &Operator::F32ReinterpretI32 => wasmparser::Operator::F32ReinterpretI32,
            &Operator::F64ReinterpretI64 => wasmparser::Operator::F64ReinterpretI64,
            &Operator::I32ReinterpretF32 => wasmparser::Operator::I32ReinterpretF32,
            &Operator::I64ReinterpretF64 => wasmparser::Operator::I64ReinterpretF64,
            &Operator::TableGet { table } => wasmparser::Operator::TableGet { table },
            &Operator::TableSet { table } => wasmparser::Operator::TableSet { table },
            &Operator::TableGrow { table } => wasmparser::Operator::TableGrow { table },
            &Operator::TableSize { table } => wasmparser::Operator::TableSize { table },
            &Operator::MemorySize { mem } => wasmparser::Operator::MemorySize {
                mem,
                mem_byte: mem as u8,
            },
            &Operator::MemoryGrow { mem } => wasmparser::Operator::MemoryGrow {
                mem,
                mem_byte: mem as u8,
            },
        }
    }
}

impl std::convert::From<MemoryImmediate> for Memory {
    fn from(value: &MemoryImmediate) -> Memory {
        Memory {
            align: value.align,
            offset: value.offset,
            memory: value.memory as MemoryId,
        }
    }
}

impl std::convert::Into<MemoryImmediate> for Memory {
    fn into(self) -> MemoryImmediate {
        MemoryImmediate {
            align: self.align,
            offset: self.offset,
            memory: self.memory as u32,
        }
    }
}
