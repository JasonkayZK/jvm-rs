use std::result::Result;

use crate::instructions::base::Instruction;
use crate::instructions::constants::ldc::{LDC, LDC2_W, LDC_W};
use crate::instructions::references::{
    CHECK_CAST, GET_FIELD, GET_STATIC, INSTANCE_OF, INVOKE_SPECIAL, INVOKE_VIRTUAL, NEW, PUT_FIELD,
    PUT_STATIC,
};

use super::comparisons::*;
use super::constants::*;
use super::control::*;
use super::conversions::*;
use super::extended::*;
use super::loads::*;
use super::math::*;
use super::stack::*;
use super::stores::*;

pub fn new_instruction(opcode: u8) -> Result<Box<dyn Instruction>, String> {
    let inst: Box<dyn Instruction> = match opcode {
        0x00 => Box::<NOP>::default(),
        0x01 => Box::<ACONST_NULL>::default(),
        0x02 => Box::<ICONST_M1>::default(),
        0x03 => Box::<ICONST_0>::default(),
        0x04 => Box::<ICONST_1>::default(),
        0x05 => Box::<ICONST_2>::default(),
        0x06 => Box::<ICONST_3>::default(),
        0x07 => Box::<ICONST_4>::default(),
        0x08 => Box::<ICONST_5>::default(),
        0x09 => Box::<LCONST_0>::default(),
        0x0a => Box::<LCONST_1>::default(),
        0x0b => Box::<FCONST_0>::default(),
        0x0c => Box::<FCONST_1>::default(),
        0x0d => Box::<FCONST_2>::default(),
        0x0e => Box::<DCONST_0>::default(),
        0x0f => Box::<DCONST_1>::default(),
        0x10 => Box::<BIPUSH>::default(),
        0x11 => Box::<SIPUSH>::default(),
        0x12 => Box::<LDC>::default(),
        0x13 => Box::<LDC_W>::default(),
        0x14 => Box::<LDC2_W>::default(),
        0x15 => Box::<ILOAD>::default(),
        0x16 => Box::<LLOAD>::default(),
        0x17 => Box::<FLOAD>::default(),
        0x18 => Box::<DLOAD>::default(),
        0x19 => Box::<ALOAD>::default(),
        0x1a => Box::<ILOAD_0>::default(),
        0x1b => Box::<ILOAD_1>::default(),
        0x1c => Box::<ILOAD_2>::default(),
        0x1d => Box::<ILOAD_3>::default(),
        0x1e => Box::<LLOAD_0>::default(),
        0x1f => Box::<LLOAD_1>::default(),
        0x20 => Box::<LLOAD_2>::default(),
        0x21 => Box::<LLOAD_3>::default(),
        0x22 => Box::<FLOAD_0>::default(),
        0x23 => Box::<FLOAD_1>::default(),
        0x24 => Box::<FLOAD_2>::default(),
        0x25 => Box::<FLOAD_3>::default(),
        0x26 => Box::<DLOAD_0>::default(),
        0x27 => Box::<DLOAD_1>::default(),
        0x28 => Box::<DLOAD_2>::default(),
        0x29 => Box::<DLOAD_3>::default(),
        0x2a => Box::<ALOAD_0>::default(),
        0x2b => Box::<ALOAD_1>::default(),
        0x2c => Box::<ALOAD_2>::default(),
        0x2d => Box::<ALOAD_3>::default(),
        // 0x2e => {
        //     Box::new(IALOAD::default())
        // },
        // 0x2f => {
        //     Box::new(LALOAD::default())
        // },
        // 0x30 => {
        //     Box::new(FALOAD::default())
        // },
        // 0x31 => {
        //     Box::new(DALOAD::default())
        // },
        // 0x32 => {
        //     Box::new(AALOAD::default())
        // },
        // 0x33 => {
        //     Box::new(BALOAD::default())
        // },
        // 0x34 => {
        //     Box::new(CALOAD::default())
        // },
        // 0x35 => {
        //     Box::new(SALOAD::default())
        // },
        0x36 => Box::<ISTORE>::default(),
        0x37 => Box::<LSTORE>::default(),
        0x38 => Box::<FSTORE>::default(),
        0x39 => Box::<DSTORE>::default(),
        0x3a => Box::<ASTORE>::default(),
        0x3b => Box::<ISTORE_0>::default(),
        0x3c => Box::<ISTORE_1>::default(),
        0x3d => Box::<ISTORE_2>::default(),
        0x3e => Box::<ISTORE_3>::default(),
        0x3f => Box::<LSTORE_0>::default(),
        0x40 => Box::<LSTORE_1>::default(),
        0x41 => Box::<LSTORE_2>::default(),
        0x42 => Box::<LSTORE_3>::default(),
        0x43 => Box::<FSTORE_0>::default(),
        0x44 => Box::<FSTORE_1>::default(),
        0x45 => Box::<FSTORE_2>::default(),
        0x46 => Box::<FSTORE_3>::default(),
        0x47 => Box::<DSTORE_0>::default(),
        0x48 => Box::<DSTORE_1>::default(),
        0x49 => Box::<DSTORE_2>::default(),
        0x4a => Box::<DSTORE_3>::default(),
        0x4b => Box::<ASTORE_0>::default(),
        0x4c => Box::<ASTORE_1>::default(),
        0x4d => Box::<ASTORE_2>::default(),
        0x4e => Box::<ASTORE_3>::default(),
        // 0x4f => {
        //     Box::new(IASTORE::default())
        // },
        // 0x50 => {
        //     Box::new(LASTORE::default())
        // },
        // 0x51 => {
        //     Box::new(FASTORE::default())
        // },
        // 0x52 => {
        //     Box::new(DASTORE::default())
        // },
        // 0x53 => {
        //     Box::new(AASTORE::default())
        // },
        // 0x54 => {
        //     Box::new(BASTORE::default())
        // },
        // 0x55 => {
        //     Box::new(CASTORE::default())
        // },
        // 0x56 => {
        //     Box::new(SASTORE::default())
        // },
        0x57 => Box::<POP>::default(),
        0x58 => Box::<POP2>::default(),
        0x59 => Box::<DUP>::default(),
        0x5a => Box::<DUP_X1>::default(),
        0x5b => Box::<DUP_X2>::default(),
        0x5c => Box::<DUP2>::default(),
        0x5d => Box::<DUP2_X1>::default(),
        0x5e => Box::<DUP2_X2>::default(),
        0x5f => Box::<SWAP>::default(),
        0x60 => Box::<IADD>::default(),
        0x61 => Box::<LADD>::default(),
        0x62 => Box::<FADD>::default(),
        0x63 => Box::<DADD>::default(),
        0x64 => Box::<ISUB>::default(),
        0x65 => Box::<LSUB>::default(),
        0x66 => Box::<FSUB>::default(),
        0x67 => Box::<DSUB>::default(),
        0x68 => Box::<IMUL>::default(),
        0x69 => Box::<LMUL>::default(),
        0x6a => Box::<FMUL>::default(),
        0x6b => Box::<DMUL>::default(),
        0x6c => Box::<IDIV>::default(),
        0x6d => Box::<LDIV>::default(),
        0x6e => Box::<FDIV>::default(),
        0x6f => Box::<DDIV>::default(),
        0x70 => Box::<IREM>::default(),
        0x71 => Box::<LREM>::default(),
        0x72 => Box::<FREM>::default(),
        0x73 => Box::<DREM>::default(),
        0x74 => Box::<INEG>::default(),
        0x75 => Box::<LNEG>::default(),
        0x76 => Box::<FNEG>::default(),
        0x77 => Box::<DNEG>::default(),
        0x78 => Box::<ISHL>::default(),
        0x79 => Box::<LSHL>::default(),
        0x7a => Box::<ISHR>::default(),
        0x7b => Box::<LSHR>::default(),
        0x7c => Box::<IUSHR>::default(),
        0x7d => Box::<LUSHR>::default(),
        0x7e => Box::<IAND>::default(),
        0x7f => Box::<LAND>::default(),
        0x80 => Box::<IOR>::default(),
        0x81 => Box::<LOR>::default(),
        0x82 => Box::<IXOR>::default(),
        0x83 => Box::<LXOR>::default(),
        0x84 => Box::<IINC>::default(),
        0x85 => Box::<I2L>::default(),
        0x86 => Box::<I2F>::default(),
        0x87 => Box::<I2D>::default(),
        0x88 => Box::<L2I>::default(),
        0x89 => Box::<L2F>::default(),
        0x8a => Box::<L2D>::default(),
        0x8b => Box::<F2I>::default(),
        0x8c => Box::<F2L>::default(),
        0x8d => Box::<F2D>::default(),
        0x8e => Box::<D2I>::default(),
        0x8f => Box::<D2L>::default(),
        0x90 => Box::<D2F>::default(),
        0x91 => Box::<I2B>::default(),
        0x92 => Box::<I2C>::default(),
        0x93 => Box::<I2S>::default(),
        0x94 => Box::<LCMP>::default(),
        0x95 => Box::<FCMPL>::default(),
        0x96 => Box::<FCMPG>::default(),
        0x97 => Box::<DCMPL>::default(),
        0x98 => Box::<DCMPG>::default(),
        0x99 => Box::<IFEQ>::default(),
        0x9a => Box::<IFNE>::default(),
        0x9b => Box::<IFLT>::default(),
        0x9c => Box::<IFGE>::default(),
        0x9d => Box::<IFGT>::default(),
        0x9e => Box::<IFLE>::default(),
        0x9f => Box::<IF_ICMPEQ>::default(),
        0xa0 => Box::<IF_ICMPNE>::default(),
        0xa1 => Box::<IF_ICMPLT>::default(),
        0xa2 => Box::<IF_ICMPGE>::default(),
        0xa3 => Box::<IF_ICMPGT>::default(),
        0xa4 => Box::<IF_ICMPLE>::default(),
        0xa5 => Box::<IF_ACMPEQ>::default(),
        0xa6 => Box::<IF_ACMPNE>::default(),
        0xa7 => Box::<GOTO>::default(),
        // 0xa8 => {
        //     Box::new(JSR::default())
        // },
        // 0xa9 => {
        //     Box::new(RET::default())
        // },
        0xaa => Box::<TABLE_SWITCH>::default(),
        0xab => Box::<LOOKUP_SWITCH>::default(),
        // 0xac => {
        //     Box::new(IRETURN::default())
        // },
        // 0xad => {
        //     Box::new(LRETURN::default())
        // },
        // 0xae => {
        //     Box::new(FRETURN::default())
        // },
        // 0xaf => {
        //     Box::new(DRETURN::default())
        // },
        // 0xb0 => {
        //     Box::new(ARETURN::default())
        // },
        // 0xb1 => {
        //     Box::new(_RETURN::default())
        // },
        0xb2 => Box::<GET_STATIC>::default(),
        0xb3 => Box::<PUT_STATIC>::default(),
        0xb4 => Box::<GET_FIELD>::default(),
        0xb5 => Box::<PUT_FIELD>::default(),
        0xb6 => Box::<INVOKE_VIRTUAL>::default(),
        0xb7 => Box::<INVOKE_SPECIAL>::default(),
        // 0xb8 => {
        //     Box::new(INVOKE_STATIC::default())
        // },
        // 0xb9 => {
        //     Box::new(INVOKE_INTERFACE::default())
        // },
        // 0xba => {
        //     Box::new(INVOKE_DYNAMIC::default())
        // },
        0xbb => Box::<NEW>::default(),
        // 0xbc => {
        //     Box::new(NEW_ARRAY::default())
        // },
        // 0xbd => {
        //     Box::new(ANEW_ARRAY::default())
        // },
        // 0xbe => {
        //     Box::new(ARRAYLENG::default())
        // },
        // 0xbf => {
        //     Box::new(ATHROW::default())
        // },
        0xc0 => Box::<CHECK_CAST>::default(),
        0xc1 => Box::<INSTANCE_OF>::default(),
        // 0xc2 => {
        //     Box::new(MONITORENTER::default())
        // },
        // 0xc3 => {
        //     Box::new(MONITOREXIT::default())
        // },
        0xc4 => Box::<WIDE>::default(),
        // 0xc5 => {
        //     Box::new(MULTI_ANEW_ARRAY::default())
        // },
        0xc6 => Box::<IFNULL>::default(),
        0xc7 => Box::<IFNONNULL>::default(),
        0xc8 => Box::<GOTO_W>::default(),
        // 0xc9 => {
        //     Box::new(JSR_W::default())
        // },
        // 0xca => {
        //     breakpoint
        // },
        // 0xfe => {
        //     impdep1
        // },
        // 0xff => {
        //     impdep2
        // },
        _ => {
            return Err(format!("Unsupported opcode: 0x{:x}!", opcode));
        }
    };

    Ok(inst)
}
