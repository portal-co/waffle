use std::{collections::{BTreeMap, BTreeSet}, ops::{Deref, DerefMut}};

use crate::{
    cfg::CFGInfo, pool::ListRef, Block, BlockTarget, FrontendOptions, Func, FunctionBody, Memory,
    MemoryArg, Module, Operator, Signature, SignatureData, Terminator, Type, Value, ValueDef,
};
pub fn sop_i32(f: &mut FunctionBody, b: Block, x: Value, y: u32, o: Operator) -> Value {
    let t = f.single_type_list(Type::I32);
    let vi = f.add_value(ValueDef::Operator(
        Operator::I32Const { value: y },
        ListRef::default(),
        t,
    ));
    f.append_to_block(b, vi);
    let v = f.arg_pool.double(x, vi);
    let w = f.add_value(ValueDef::Operator(o, v, t));
    f.append_to_block(b, w);
    return w;
}
pub fn make_memcpy(m: &mut Module, _1: Memory, _2: Memory, swizzle: bool) -> Func {
    let mut b = FunctionBody::default();
    let e = b.entry.clone();
    let k = make_memcpy_body(&mut b, e, _1, _2, swizzle);
    b.set_terminator(k, Terminator::Return { values: vec![] });
    let s = m.signatures.push(SignatureData {
        params: vec![Type::I32, Type::I32, Type::I32],
        returns: vec![],
    });
    return m
        .funcs
        .push(crate::FuncDecl::Body(s,format!("memcpy_{_1}_{_2}_swizzle_{swizzle}"), b));
}
pub fn make_memcpy_body(
    f: &mut FunctionBody,
    b: Block,
    _1: Memory,
    _2: Memory,
    swizzle: bool,
) -> Block {
    let k = f.add_block();
    let a = f.add_blockparam(b, Type::I32);
    let c = f.add_blockparam(b, Type::I32);
    let d = f.add_blockparam(b, Type::I32);

    let ra = f.arg_pool.single(a);
    let rc = f.arg_pool.single(c);
    let t = f.single_type_list(Type::I32);
    let l = f.add_value(ValueDef::Operator(
        crate::Operator::I32Load8U {
            memory: MemoryArg {
                align: 1,
                offset: 0,
                memory: _1,
            },
        },
        ra,
        t,
    ));
    f.append_to_block(b, l);
    let m = if swizzle {
        let l = f.add_value(ValueDef::Operator(
            crate::Operator::I32Load8U {
                memory: MemoryArg {
                    align: 1,
                    offset: 0,
                    memory: _2,
                },
            },
            rc,
            t,
        ));
        f.append_to_block(b, l);
        Some(l)
    } else {
        None
    };
    let rc = f.arg_pool.double(c, l);
    let ra = m.map(|l| f.arg_pool.double(a, l));
    // let u = f.type_pool.allocate(0, Type::I32);
    let m = f.add_value(ValueDef::Operator(
        crate::Operator::I32Store8 {
            memory: MemoryArg {
                align: 1,
                offset: 0,
                memory: _2,
            },
        },
        rc,
        ListRef::default(),
    ));
    f.append_to_block(b, m);
    if let Some(ra) = ra {
        let m = f.add_value(ValueDef::Operator(
            crate::Operator::I32Store8 {
                memory: MemoryArg {
                    align: 1,
                    offset: 0,
                    memory: _1,
                },
            },
            ra,
            ListRef::default(),
        ));
        f.append_to_block(b, m);
    }
    let mut r = vec![];
    r.push(sop_i32(f, b, a, 1, Operator::I32Add));
    r.push(sop_i32(f, b, c, 1, Operator::I32Add));
    r.push(sop_i32(f, b, d, 1, Operator::I32Sub));
    f.set_terminator(
        b,
        Terminator::Select {
            value: d,
            targets: vec![BlockTarget {
                block: k,
                args: vec![],
            }],
            default: BlockTarget { block: b, args: r },
        },
    );
    return k;
}
