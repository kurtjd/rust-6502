#![allow(unused_variables)]

use bitflags::bitflags;

type MemReadCallback<'a> = Box<dyn FnMut(usize) -> u8 + 'a>;
type MemWriteCallback<'a> = Box<dyn FnMut(usize, u8) + 'a>;

const STACK_OFFSET: usize = 0x0100;
const RESET_VECTOR: usize = 0xFFFC;
const INTR_VECTOR: usize = 0xFFFE;

enum AddrMode {
    ACM0, // Accumulator
    ABS0, // Absolute
    ABSX, // Absolute Indexed with X
    ABSY, // Absolute Indexed with Y
    IMM0, // Immediate
    IMP0, // Implied
    IND0, // Indirect
    INDX, // Indirect Indexed with X
    INDY, // Indirect Indexed with Y
    REL0, // Relative
    ZPG0, // Zero Page
    ZPGX, // Zero Page Indexed Indirect with X
    ZPGY, // Zero Page Indexed Indirect with Y
}

struct Opcode {
    instr: fn(&mut Cpu6502, &Opcode, &[u8]),
    mode: AddrMode,
    bytes: u8,
}

static OPCODES: [Opcode; 0x100] = [
    // $00-$0F
    Opcode {
        instr: instructions::brk,
        mode: AddrMode::IMP0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::asl,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::php,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::asl,
        mode: AddrMode::ACM0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::anc,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::asl,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $10 - $1F
    Opcode {
        instr: instructions::bpl,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::asl,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::clc,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ora,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::asl,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::slo,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    // $20 - $2F
    Opcode {
        instr: instructions::jsr,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::bit,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rol,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::plp,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rol,
        mode: AddrMode::ACM0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::anc,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::bit,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rol,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $30 - $3F
    Opcode {
        instr: instructions::bmi,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rol,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sec,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::and,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rol,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rla,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    // $40 - $4F
    Opcode {
        instr: instructions::rti,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lsr,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::pha,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lsr,
        mode: AddrMode::ACM0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::alr,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jmp,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lsr,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $50 - $5F
    Opcode {
        instr: instructions::bvc,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lsr,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cli,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::eor,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lsr,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sre,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    // $60 - $6F
    Opcode {
        instr: instructions::rts,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ror,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::pla,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ror,
        mode: AddrMode::ACM0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::arr,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jmp,
        mode: AddrMode::IND0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ror,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $70 - $7F
    Opcode {
        instr: instructions::bvs,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ror,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sei,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::adc,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ror,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::rra,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    // $80 - $8F
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sax,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sty,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::stx,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sax,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dey,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::txa,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::ane,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sty,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::stx,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sax,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $90 - $9F
    Opcode {
        instr: instructions::bcc,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sha,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sty,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::stx,
        mode: AddrMode::ZPGY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sax,
        mode: AddrMode::ZPGY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::tya,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::txs,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::tas,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::shy,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sta,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::shx,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sha,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    // $A0 - $AF
    Opcode {
        instr: instructions::ldy,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldx,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldy,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldx,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::tay,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::tax,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::lxa,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldy,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ldx,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $B0 - $BF
    Opcode {
        instr: instructions::bcs,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldy,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::ldx,
        mode: AddrMode::ZPGY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::ZPGY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::clv,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::tsx,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::las,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ldy,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lda,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::ldx,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::lax,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    // $C0 - $CF
    Opcode {
        instr: instructions::cpy,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cpy,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dec,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::iny,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dex,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sbx,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cpy,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::dec,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $D0 - $DF
    Opcode {
        instr: instructions::bne,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dec,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cld,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::cmp,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::dec,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::dcp,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    // $E0 - $EF
    Opcode {
        instr: instructions::cpx,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::INDX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cpx,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::inc,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::ZPG0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::inx,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::usb,
        mode: AddrMode::IMM0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::cpx,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::inc,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::ABS0,
        bytes: 3,
    },
    // $F0 - $FF
    Opcode {
        instr: instructions::beq,
        mode: AddrMode::REL0,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::jam,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::INDY,
        bytes: 2,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::inc,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::ZPGX,
        bytes: 2,
    },
    Opcode {
        instr: instructions::sed,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::IMP0,
        bytes: 1,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::ABSY,
        bytes: 3,
    },
    Opcode {
        instr: instructions::nop,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::sbc,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::inc,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
    Opcode {
        instr: instructions::isc,
        mode: AddrMode::ABSX,
        bytes: 3,
    },
];

bitflags! {
    pub struct StatusFlags: u8 {
        const N = 1 << 7;   // Negative
        const V = 1 << 6;   // Overflow
        const E = 1 << 5;   // Extension (unused, but initialized to 1)
        const B = 1 << 4;   // Break
        const D = 1 << 3;   // Decimal
        const I = 1 << 2;   // Interrupt Disable
        const Z = 1 << 1;   // Zero
        const C = 1 << 0;   // Carry
    }
}

pub struct Registers {
    pub pc: u16,        // Program counter
    pub s: u8,          // Stack pointer
    pub a: u8,          // Accumulator
    pub x: u8,          // X register
    pub y: u8,          // Y register
    pub p: StatusFlags, // Status
}

pub struct Cpu6502<'a> {
    pub registers: Registers,
    mem_read: MemReadCallback<'a>,
    mem_write: MemWriteCallback<'a>,
    cycles: u8,
    halted: bool,
}

impl<'a> Cpu6502<'a> {
    /* The cpu requires external memory management. It takes as arguments callbacks to memory read
    and memory write functions in the form of closures. Using the CPU would look like:

    // The memory manager
    let mem_man = Rc::new(RefCell::new(MemManager::new()));

    // Create closures for memory manager's read/write methods
    let mem_read = |address: usize| -> u8 {
        mem_man.clone().borrow_mut().mem_read(address)
    };
    let mem_write = |address: usize, value: u8| {
        mem_man.clone().borrow_mut().mem_write(address, value);
    };

    // Pass memory management closures to CPU
    let mut cpu = Cpu6502::new(
        Box::new(mem_read),
        Box::new(mem_write)
    );

     */
    pub fn new(mem_read: MemReadCallback<'a>, mem_write: MemWriteCallback<'a>) -> Self {
        Cpu6502 {
            registers: Registers {
                pc: 0,
                a: 0,
                x: 0,
                y: 0,
                s: 0,
                p: StatusFlags::empty(),
            },

            mem_read,
            mem_write,
            cycles: 0,
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        // Set the PC to point to address stored in reset vector
        let lsb = self.read(RESET_VECTOR);
        let msb = self.read(RESET_VECTOR + 1);
        self.registers.pc = (msb as u16) << 8 | (lsb as u16);

        // Disable interrupts flag and extension bit should be set
        self.registers.p = StatusFlags::E | StatusFlags::I;

        self.halted = false;
    }

    pub fn tick(&mut self) -> u8 {
        self.cycles = 0;

        if self.halted {
            return 0;
        } // Do nothing if halted, typically after encountering a 'jam'

        let fetch = self.read(self.registers.pc as usize) as usize;
        let opcode = &OPCODES[fetch];

        // Find more Rusty way to handle this...
        let mut operands = [0, 0];
        let num_operands = match opcode.bytes > 2 {
            true => opcode.bytes - 1,
            false => 1, // Even for 1 byte opcodes, want to pull a dummy operand
        };

        // JSR ($20) is special so it gets to read its own operands thank you very much
        if fetch != 0x20 {
            for i in 0..num_operands {
                operands[i as usize] =
                    self.read(self.registers.pc.wrapping_add(1 + i as u16) as usize);
            }
        }

        self.registers.pc = self.registers.pc.wrapping_add(opcode.bytes as u16);
        (opcode.instr)(self, opcode, &operands);

        self.cycles
    }

    fn read(&mut self, address: usize) -> u8 {
        self.cycles += 1;
        (self.mem_read)(address)
    }

    fn write(&mut self, address: usize, value: u8) {
        self.cycles += 1;
        (self.mem_write)(address, value)
    }
}

pub mod instructions {
    use super::*;

    /* Needed a way to segregate direct access to memory from the CPU, though didn't
    originally plan for that, so had to introduce this ugly hack :( */
    fn check_read(
        cpu: &mut Cpu6502,
        addr: usize,
        no_read_val: u8,
        pgx: bool,
        to_read: bool,
        cond_read: bool,
    ) -> u8 {
        match to_read {
            true => match cond_read {
                true => match pgx {
                    true => cpu.read(addr as usize),
                    false => no_read_val,
                },
                false => cpu.read(addr as usize),
            },
            false => no_read_val,
        }
    }

    // For easy handling of different address modes
    // This does not seem Rusty at all so need to find better way to avoid casting and wraps everywhere
    fn get_mem(
        cpu: &mut Cpu6502,
        mode: &AddrMode,
        operands: &[u8],
        read: bool,
        cond_read: bool,
    ) -> (usize, u8, bool) {
        match mode {
            AddrMode::ABS0 => {
                let addr = (operands[1] as usize) << 8 | operands[0] as usize;
                let value = check_read(cpu, addr, 0, true, read, cond_read);

                (addr, value, false)
            }
            AddrMode::ABSX => {
                // Have to read unfixed address first
                let value = cpu.read(
                    (operands[1] as usize) << 8
                        | operands[0].wrapping_add(cpu.registers.x) as usize,
                );

                let addr = (operands[1] as u16) << 8 | operands[0] as u16;
                let eff_addr = addr.wrapping_add(cpu.registers.x as u16);
                let pgx = (eff_addr & 0xFF00) != (addr & 0xFF00);

                let ret_val = check_read(cpu, eff_addr as usize, value, pgx, read, cond_read);

                (eff_addr as usize, ret_val, pgx)
            }
            AddrMode::ABSY => {
                // Have to read unfixed address first
                let value = cpu.read(
                    (operands[1] as usize) << 8
                        | operands[0].wrapping_add(cpu.registers.y) as usize,
                );

                let addr = (operands[1] as u16) << 8 | operands[0] as u16;
                let eff_addr = addr.wrapping_add(cpu.registers.y as u16);
                let pgx = (eff_addr & 0xFF00) != (addr & 0xFF00);

                let ret_val = check_read(cpu, eff_addr as usize, value, pgx, read, cond_read);

                (eff_addr as usize, ret_val, pgx)
            }
            AddrMode::IND0 => {
                let lsb_addr = (operands[1] as usize) << 8 | operands[0] as usize;
                let lsb = cpu.read(lsb_addr) as usize;

                // Have to add to lsb only of msb_addr due to CPU bug
                let msb_addr = (operands[1] as usize) << 8 | operands[0].wrapping_add(1) as usize;
                let msb = cpu.read(msb_addr) as usize;
                let eff_addr = msb << 8 | lsb;

                let value = check_read(cpu, eff_addr, 0, true, read, cond_read);

                (eff_addr, value, false)
            }
            AddrMode::INDX => {
                // Dummy read
                cpu.read(operands[0] as usize);

                let addr = (operands[0].wrapping_add(cpu.registers.x)) as u8;
                let lsb = cpu.read(addr as usize) as usize;
                let msb = cpu.read(addr.wrapping_add(1) as usize) as usize;
                let eff_addr = msb << 8 | lsb;

                let value = check_read(cpu, eff_addr, 0, true, read, cond_read);

                (eff_addr, value, false)
            }
            AddrMode::INDY => {
                let zpaddr = operands[0];
                let lsb = cpu.read(zpaddr as usize) as u16;
                let msb = cpu.read(zpaddr.wrapping_add(1) as usize) as u16;
                let addr = msb << 8 | lsb;

                // Dummy read of unfixed address
                let value = cpu
                    .read((msb as usize) << 8 | cpu.registers.y.wrapping_add(lsb as u8) as usize);

                let eff_addr = addr.wrapping_add(cpu.registers.y as u16);
                let pgx = (eff_addr & 0xFF00) != (addr & 0xFF00);

                let ret_val = check_read(cpu, eff_addr as usize, value, pgx, read, cond_read);

                (eff_addr as usize, ret_val, pgx)
            }
            AddrMode::REL0 => {
                let addr = cpu.registers.pc as i32;
                let offset = (operands[0] as i8) as i32;
                let eff_addr = ((addr + offset) as u16) as usize;
                let pgx = (eff_addr & 0xFF00) != (addr as usize & 0xFF00);

                let value = check_read(cpu, eff_addr, 0, pgx, read, cond_read);

                (eff_addr, value, pgx)
            }
            AddrMode::ZPG0 => {
                let value = check_read(cpu, operands[0] as usize, 0, true, read, cond_read);
                (operands[0] as usize, value, false)
            }
            AddrMode::ZPGX => {
                cpu.read(operands[0] as usize); // Read and discard
                let eff_addr = (operands[0].wrapping_add(cpu.registers.x)) as usize;
                let value = check_read(cpu, eff_addr, 0, true, read, cond_read);
                (eff_addr, value, false)
            }
            AddrMode::ZPGY => {
                cpu.read(operands[0] as usize); // Read and discard
                let eff_addr = (operands[0].wrapping_add(cpu.registers.y)) as usize;
                let value = check_read(cpu, eff_addr, 0, true, read, cond_read);
                (eff_addr, value, false)
            }
            AddrMode::ACM0 => (0, cpu.registers.a, false),
            AddrMode::IMM0 => (0, operands[0], false),
            AddrMode::IMP0 => (0, 0, false),
        }
    }

    // Commonly performed by quite a few instructions
    fn update_zn_flags(cpu: &mut Cpu6502, result: u8) {
        cpu.registers.p &= !(StatusFlags::Z | StatusFlags::N);
        if result == 0 {
            cpu.registers.p |= StatusFlags::Z;
        } else if result & (1 << 7) != 0 {
            cpu.registers.p |= StatusFlags::N;
        }
    }

    // For easy stack manipulation
    fn stack_push(cpu: &mut Cpu6502, value: u8) {
        cpu.write(STACK_OFFSET + cpu.registers.s as usize, value);
        cpu.registers.s = cpu.registers.s.wrapping_sub(1);
    }
    fn stack_pop(cpu: &mut Cpu6502) -> u8 {
        cpu.registers.s = cpu.registers.s.wrapping_add(1);
        cpu.read(STACK_OFFSET + cpu.registers.s as usize)
    }
    fn stack_push16(cpu: &mut Cpu6502, value: u16) {
        stack_push(cpu, (value >> 8) as u8);
        stack_push(cpu, (value & 0xFF) as u8);
    }
    fn stack_pop16(cpu: &mut Cpu6502) -> u16 {
        let lsb = stack_pop(cpu) as u16;
        let msb = stack_pop(cpu) as u16;
        msb << 8 | lsb
    }

    // Load/Store Operations
    pub(super) fn lda(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.a = value;

        update_zn_flags(cpu, value);
    }
    pub(super) fn ldx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.x = value;

        update_zn_flags(cpu, value);
    }
    pub(super) fn ldy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.y = value;

        update_zn_flags(cpu, value);
    }

    pub(super) fn sta(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands, false, false);
        cpu.write(addr, cpu.registers.a);
    }
    pub(super) fn stx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands, false, false);
        cpu.write(addr, cpu.registers.x);
    }
    pub(super) fn sty(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands, false, false);
        cpu.write(addr, cpu.registers.y);
    }

    // Register Transfers
    pub(super) fn tax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        update_zn_flags(cpu, cpu.registers.a);
        cpu.registers.x = cpu.registers.a;
    }
    pub(super) fn tay(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        update_zn_flags(cpu, cpu.registers.a);
        cpu.registers.y = cpu.registers.a;
    }
    pub(super) fn txa(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        update_zn_flags(cpu, cpu.registers.x);
        cpu.registers.a = cpu.registers.x;
    }
    pub(super) fn tya(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        update_zn_flags(cpu, cpu.registers.y);
        cpu.registers.a = cpu.registers.y;
    }

    // Stack Operations
    pub(super) fn tsx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        update_zn_flags(cpu, cpu.registers.s);
        cpu.registers.x = cpu.registers.s;
    }
    pub(super) fn txs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.s = cpu.registers.x;
    }
    pub(super) fn pha(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        stack_push(cpu, cpu.registers.a);
    }
    pub(super) fn php(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let psw = StatusFlags::from_bits(cpu.registers.p.bits()).unwrap() | StatusFlags::B;
        stack_push(cpu, psw.bits());
    }
    pub(super) fn pla(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.read(STACK_OFFSET + cpu.registers.s as usize); // Dummy read
        cpu.registers.a = stack_pop(cpu);
        update_zn_flags(cpu, cpu.registers.a);
    }
    pub(super) fn plp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.read(STACK_OFFSET + cpu.registers.s as usize); // Dummy read
        let result = stack_pop(cpu);

        // We should ignore the Break and Extension flags from the pop
        cpu.registers.p &= StatusFlags::B | StatusFlags::E;
        cpu.registers.p |=
            StatusFlags::from_bits(result).unwrap() & !(StatusFlags::B | StatusFlags::E);
    }

    // Logical Operations
    pub(super) fn and(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.a &= value;

        update_zn_flags(cpu, cpu.registers.a);
    }
    pub(super) fn eor(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.a ^= value;

        update_zn_flags(cpu, cpu.registers.a);
    }
    pub(super) fn ora(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        cpu.registers.a |= value;

        update_zn_flags(cpu, cpu.registers.a);
    }
    pub(super) fn bit(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, value, _) = get_mem(cpu, &opcode.mode, operands, true, false);
        let result = cpu.registers.a & value;
        update_zn_flags(cpu, result);

        // Copy the V and N bits from memory into status reg
        cpu.registers.p &= !(StatusFlags::V | StatusFlags::N);
        let m = StatusFlags::from_bits(value).unwrap() & (StatusFlags::V | StatusFlags::N);
        cpu.registers.p |= m;
    }

    // Arithmetic Operations
    fn compare(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8], reg: char) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);
        let reg = match reg {
            'a' => cpu.registers.a,
            'x' => cpu.registers.x,
            'y' => cpu.registers.y,
            _ => 0, // Shouldn't get here
        };

        let result = reg.wrapping_sub(value);

        update_zn_flags(cpu, result);
        cpu.registers.p &= !StatusFlags::C;
        if reg >= value {
            cpu.registers.p |= StatusFlags::C;
        }
    }
    pub(super) fn adc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        let carry = match cpu.registers.p.contains(StatusFlags::C) {
            true => 1,
            false => 0,
        };
        let op1 = cpu.registers.a as u16;
        let op2 = value as u16;

        let bsum = op1 + op2 + carry;
        let mut sum = match cpu.registers.p.contains(StatusFlags::D) {
            true => {
                // Add low nibbles
                let mut res = carry + (op1 & 0xF) + (op2 & 0xF);

                // Perform correction and set the carry bit
                if res > 0x09 {
                    res += 0x06;
                    res = (res & 0x0F) | 0x10;
                }

                // Add high nibbles plus corrected low nibble
                (op1 & 0xF0) + (op2 & 0xF0) + res
            }
            false => bsum,
        };

        // Must set negative and overflow flags before correcting high nibble
        if sum & (1 << 7) != 0 {
            cpu.registers.p |= StatusFlags::N;
        } else {
            cpu.registers.p &= !StatusFlags::N;
        }
        if (!(op1 ^ op2) & (op1 ^ sum) & (1 << 7)) != 0 {
            cpu.registers.p |= StatusFlags::V;
        } else {
            cpu.registers.p &= !StatusFlags::V;
        }

        // Correct high nibble
        if cpu.registers.p.contains(StatusFlags::D) && sum > 0x9F {
            sum += 0x60;
        }

        // Now set carry flag
        if sum > 0xFF {
            cpu.registers.p |= StatusFlags::C;
        } else {
            cpu.registers.p &= !StatusFlags::C;
        }

        // Zero flag is always set based on binary addition
        if (bsum as u8) == 0 {
            cpu.registers.p |= StatusFlags::Z;
        } else {
            cpu.registers.p &= !StatusFlags::Z;
        }

        cpu.registers.a = sum as u8;
    }
    pub(super) fn sbc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, _) = get_mem(cpu, &opcode.mode, operands, true, true);

        // We subtract the inverted carry bit
        let carry = match cpu.registers.p.contains(StatusFlags::C) {
            true => 0,
            false => 1,
        };
        let op1 = cpu.registers.a;
        let op2 = value;

        let bsum = (op1 as u16)
            .wrapping_sub(op2 as u16)
            .wrapping_sub(carry as u16);
        let sum = match cpu.registers.p.contains(StatusFlags::D) {
            true => {
                // Subtract low nibbles and inverted carry
                let mut low = (op1 & 0xF).wrapping_sub(op2 & 0xF).wrapping_sub(carry);

                // Perform correction
                // 'Fix' here represents if the low nibble overflowed
                let mut fix = 0;
                if (low & 0x10) != 0 {
                    low -= 0x06;
                    fix = 1;
                }

                // Subtract high nibbles and 1 if corrected lower nibble overflowed
                let mut high = (op1 >> 4).wrapping_sub(op2 >> 4).wrapping_sub(fix);
                if (high & 0x10) != 0 {
                    high -= 0x6;
                }

                // Merge high and low nibbles
                (high << 4) | (low & 0xF)
            }
            false => bsum as u8,
        };

        // Update flags (SBC always updates flags based on binary result)
        // Thus decimal mode has no affect here
        update_zn_flags(cpu, bsum as u8);

        // We check overflow based on the 1's complement of the operand
        if (!(op1 ^ !op2) & (op1 ^ bsum as u8) & (1 << 7)) != 0 {
            cpu.registers.p |= StatusFlags::V;
        } else {
            cpu.registers.p &= !StatusFlags::V;
        }

        // In SBC case, carry is set if a borrow did NOT occur
        if bsum <= 0xFF {
            cpu.registers.p |= StatusFlags::C;
        } else {
            cpu.registers.p &= !StatusFlags::C;
        }

        cpu.registers.a = sum;
    }
    pub(super) fn cmp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        compare(cpu, opcode, operands, 'a');
    }
    pub(super) fn cpx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        compare(cpu, opcode, operands, 'x');
    }
    pub(super) fn cpy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        compare(cpu, opcode, operands, 'y');
    }

    // Inc/Dec Operations
    pub(super) fn inc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, _) = get_mem(cpu, &opcode.mode, operands, true, false);
        cpu.write(addr, value); // Dummy write
        value = value.wrapping_add(1);
        update_zn_flags(cpu, value);
        cpu.write(addr, value);
    }
    pub(super) fn inx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.x = cpu.registers.x.wrapping_add(1);
        update_zn_flags(cpu, cpu.registers.x);
    }
    pub(super) fn iny(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.y = cpu.registers.y.wrapping_add(1);
        update_zn_flags(cpu, cpu.registers.y);
    }
    pub(super) fn dec(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, _) = get_mem(cpu, &opcode.mode, operands, true, false);
        cpu.write(addr, value);
        value = value.wrapping_sub(1);
        update_zn_flags(cpu, value);
        cpu.write(addr, value);
    }
    pub(super) fn dex(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.x = cpu.registers.x.wrapping_sub(1);
        update_zn_flags(cpu, cpu.registers.x);
    }
    pub(super) fn dey(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.y = cpu.registers.y.wrapping_sub(1);
        update_zn_flags(cpu, cpu.registers.y);
    }

    // Shift Operations
    pub(super) fn asl(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, pgx) = get_mem(cpu, &opcode.mode, operands, true, false);

        match opcode.mode {
            AddrMode::ACM0 => {}
            _ => cpu.write(addr, value),
        };

        let old_bit7 = value & (1 << 7) != 0;
        value <<= 1;
        update_zn_flags(cpu, value);

        cpu.registers.p &= !StatusFlags::C;
        if old_bit7 {
            cpu.registers.p |= StatusFlags::C;
        }

        match opcode.mode {
            AddrMode::ACM0 => cpu.registers.a = value,
            _ => cpu.write(addr, value),
        }
    }
    pub(super) fn lsr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, _) = get_mem(cpu, &opcode.mode, operands, true, false);

        match opcode.mode {
            AddrMode::ACM0 => {}
            _ => cpu.write(addr, value),
        };

        let old_bit0 = value & 1 != 0;
        value >>= 1;
        update_zn_flags(cpu, value);

        cpu.registers.p &= !StatusFlags::C;
        if old_bit0 {
            cpu.registers.p |= StatusFlags::C;
        }

        match opcode.mode {
            AddrMode::ACM0 => cpu.registers.a = value,
            _ => cpu.write(addr, value),
        }
    }
    pub(super) fn rol(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, _) = get_mem(cpu, &opcode.mode, operands, true, false);

        match opcode.mode {
            AddrMode::ACM0 => {}
            _ => cpu.write(addr, value),
        };

        let old_bit7 = value & (1 << 7) != 0;
        value <<= 1;
        if cpu.registers.p.contains(StatusFlags::C) {
            value |= 1;
        }

        cpu.registers.p &= !StatusFlags::C;
        if old_bit7 {
            cpu.registers.p |= StatusFlags::C;
        }

        match opcode.mode {
            AddrMode::ACM0 => cpu.registers.a = value,
            _ => cpu.write(addr, value),
        }

        update_zn_flags(cpu, value);
    }
    pub(super) fn ror(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, mut value, _) = get_mem(cpu, &opcode.mode, operands, true, false);

        match opcode.mode {
            AddrMode::ACM0 => {}
            _ => cpu.write(addr, value),
        };

        let old_bit0 = value & 1 != 0;
        value >>= 1;
        if cpu.registers.p.contains(StatusFlags::C) {
            value |= 1 << 7;
        }

        cpu.registers.p &= !StatusFlags::C;
        if old_bit0 {
            cpu.registers.p |= StatusFlags::C;
        }

        match opcode.mode {
            AddrMode::ACM0 => cpu.registers.a = value,
            _ => cpu.write(addr, value),
        }

        update_zn_flags(cpu, value);
    }

    // Jump/Call Operations
    pub(super) fn jmp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands, false, false);
        cpu.registers.pc = addr as u16;
    }
    pub(super) fn jsr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        /* Because jsr can overwrite its operands during operation, we have to emulate individual
        cycles. That's the reason for this hackish way of doing things. */
        //cpu.cycles -= 2;

        // Reset the PC to just after opcode fetch (which was originally incremented during tick())
        cpu.registers.pc -= (opcode.bytes - 1) as u16;

        // Fetch the low byte of jump address, then increment pc
        //let adl: u16 = cpu.ram[cpu.registers.pc as usize] as u16;
        let adl: u16 = cpu.read(cpu.registers.pc as usize) as u16;
        cpu.registers.pc += 1;

        // Strange dummy read (sometimes things are just magic ya know?)
        cpu.read(STACK_OFFSET + cpu.registers.s as usize);

        // Push the PC to the stack
        stack_push16(cpu, cpu.registers.pc);

        // Fetch the high byte of jump address, and set PC
        //let adh: u16 = cpu.ram[cpu.registers.pc as usize] as u16;
        let adh: u16 = cpu.read(cpu.registers.pc as usize) as u16;
        cpu.registers.pc = (adh << 8) | adl;
    }
    pub(super) fn rts(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.read(STACK_OFFSET + cpu.registers.s as usize); // Dummy read
        cpu.registers.pc = stack_pop16(cpu) + 1;
        cpu.read((cpu.registers.pc - 1) as usize); // Another dummy read
    }

    // Branch Operations
    fn branch(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8], flag: StatusFlags, set: bool) {
        let (addr, _, pgx) = get_mem(cpu, &opcode.mode, operands, false, false);

        // Rust cmon why... I just want to pass in a damn bit flag and you make me do this??
        let branch_set = set && (cpu.registers.p.bits() & flag.bits()) != 0;
        let branch_clr = !set && (cpu.registers.p.bits() & flag.bits()) == 0;

        if branch_set || branch_clr {
            cpu.read(cpu.registers.pc as usize); // Dummy read if branch

            // And do another dummy read of the unfixed eff. addr if page cross
            if pgx {
                let lsb = cpu.registers.pc as u8;
                let msb = (cpu.registers.pc & 0xFF00) as usize;
                cpu.read(msb | lsb.wrapping_add(operands[0]) as usize);
            }

            cpu.registers.pc = addr as u16;
        }
    }
    pub(super) fn bmi(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::N, true);
    }
    pub(super) fn bpl(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::N, false);
    }
    pub(super) fn bvs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::V, true);
    }
    pub(super) fn bvc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::V, false);
    }
    pub(super) fn beq(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::Z, true);
    }
    pub(super) fn bne(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::Z, false);
    }
    pub(super) fn bcs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::C, true);
    }
    pub(super) fn bcc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        branch(cpu, opcode, operands, StatusFlags::C, false);
    }

    // Status Flag Operations
    pub(super) fn clc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p &= !StatusFlags::C;
    }
    pub(super) fn cld(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p &= !StatusFlags::D;
    }
    pub(super) fn cli(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p &= !StatusFlags::I;
    }
    pub(super) fn clv(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p &= !StatusFlags::V;
    }
    pub(super) fn sec(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p |= StatusFlags::C;
    }
    pub(super) fn sed(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p |= StatusFlags::D;
    }
    pub(super) fn sei(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.p |= StatusFlags::I;
    }

    // System Operations
    pub(super) fn brk(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        // Push PC to stack
        stack_push16(cpu, cpu.registers.pc);

        // Push status reg to stack
        php(cpu, opcode, operands);

        let lsb = cpu.read(INTR_VECTOR) as u16;
        let msb = cpu.read(INTR_VECTOR + 1) as u16;
        cpu.registers.pc = msb << 8 | lsb;

        // Set Interrupt Disable flag
        cpu.registers.p |= StatusFlags::I;
    }
    pub(super) fn nop(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        // Intentionally do nothing
    }
    pub(super) fn rti(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        // Pop status reg
        plp(cpu, opcode, operands);

        // Pop PC
        cpu.registers.pc = stack_pop16(cpu);
    }

    // Illegal/Undefined Operations (TODO: Cycle counts will need some work)
    pub(super) fn jam(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.pc = cpu.registers.pc.wrapping_sub(opcode.bytes as u16);
        cpu.halted = true;
    }
    pub(super) fn slo(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        asl(cpu, opcode, operands);
        ora(cpu, opcode, operands);
    }
    pub(super) fn anc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        and(cpu, opcode, operands);
        if cpu.registers.a & (1 << 7) != 0 {
            cpu.registers.p |= StatusFlags::C;
        } else {
            cpu.registers.p &= !StatusFlags::C;
        }
    }
    pub(super) fn rla(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        rol(cpu, opcode, operands);
        and(cpu, opcode, operands);
    }
    pub(super) fn sre(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        lsr(cpu, opcode, operands);
        eor(cpu, opcode, operands);
    }
    pub(super) fn alr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        and(cpu, opcode, operands);
        lsr(cpu, &OPCODES[0x4A], operands); // Always perform lsr on accumulator (opcode $4A)
    }
    pub(super) fn arr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        and(cpu, opcode, operands);
        let and_res = cpu.registers.a;
        ror(cpu, &OPCODES[0x6A], operands); // Always perform ror on accumulator (opcode $6A)
        let ror_res = cpu.registers.a;

        // This instruction used adc circuitry, so if in decimal mode have to perform fixups
        if cpu.registers.p.contains(StatusFlags::D) {
            let mut result = ror_res & 0xF;
            if and_res & 0xF > 4 {
                result += 6;
            }
            result &= 0x0F;
            result |= ror_res & 0xF0;

            if and_res & 0xF0 > 0x40 {
                result = result.wrapping_add(0x60);
                cpu.registers.p |= StatusFlags::C;
            } else {
                cpu.registers.p &= !StatusFlags::C;
            }

            cpu.registers.a = result;
        } else {
            if ror_res & (1 << 6) != 0 {
                cpu.registers.p |= StatusFlags::C;
            } else {
                cpu.registers.p &= !StatusFlags::C;
            }
        }

        // Overflow is set based on XOR of bits 6 and 5 of result
        if ((ror_res >> 6) & 1) ^ ((ror_res >> 5) & 1) != 0 {
            cpu.registers.p |= StatusFlags::V;
        } else {
            cpu.registers.p &= !StatusFlags::V;
        }
    }
    pub(super) fn rra(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        ror(cpu, opcode, operands);
        adc(cpu, opcode, operands);
    }
    pub(super) fn sax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, pgx) = get_mem(cpu, &opcode.mode, operands, false, false);
        //cpu.ram[addr] = cpu.registers.a & cpu.registers.x;
        cpu.write(addr, cpu.registers.a & cpu.registers.x);
    }
    pub(super) fn ane(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        /* This is a highly unstable operation with non-deterministic behavior in reality.
        Things like temperature can affect the value of this 'magic' constant! However, 0xEE
        seems to be the most common result for 'magic' and is the constant used in
        Tom Harte's tests. */
        let magic = 0xEE;

        cpu.registers.a = (cpu.registers.a | magic) & cpu.registers.x;
        and(cpu, opcode, operands);
    }
    fn shr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8], reg: char) {
        let (mut addr, _, pgx) = get_mem(cpu, &opcode.mode, operands, false, false);

        let mut result = match reg {
            'a' => cpu.registers.a & cpu.registers.x,
            'x' => cpu.registers.x,
            'y' => cpu.registers.y,
            _ => 0, // Shouldn't get here
        };

        /* If we have a page crossing, we should NOT increment the high byte, and the result
        of the AND operation should overwrite the high byte of the effective address. */
        if pgx {
            let adh = (addr >> 8) as u8;
            result &= adh;
            addr = ((result as usize) << 8) | (addr & 0xFF);
        } else {
            let adh = ((addr >> 8) + 1) as u8;
            result &= adh;
        }

        cpu.write(addr, result);
    }
    pub(super) fn sha(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        shr(cpu, opcode, operands, 'a');
    }
    pub(super) fn shx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        shr(cpu, opcode, operands, 'x');
    }
    pub(super) fn shy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        shr(cpu, opcode, operands, 'y');
    }
    pub(super) fn tas(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        cpu.registers.s = cpu.registers.a & cpu.registers.x;
        sha(cpu, opcode, operands);
    }
    pub(super) fn lax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        lda(cpu, opcode, operands);
        ldx(cpu, opcode, operands);
    }
    pub(super) fn las(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (_, value, pgx) = get_mem(cpu, &opcode.mode, operands, true, false);
        let result = value & cpu.registers.s;
        update_zn_flags(cpu, result);
        cpu.registers.a = result;
        cpu.registers.x = result;
        cpu.registers.s = result;
    }
    pub(super) fn lxa(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        /* This is a highly unstable operation with non-deterministic behavior in reality.
        Things like temperature can affect the value of this 'magic' constant! However, 0xEE
        seems to be the most common result for 'magic' and is the constant used in
        Tom Harte's tests. */
        let magic = 0xEE;

        cpu.registers.a |= magic;
        and(cpu, opcode, operands);
        cpu.registers.x = cpu.registers.a;
    }
    pub(super) fn dcp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let (addr, _, pgx) = get_mem(cpu, &opcode.mode, operands, false, false);
        dec(cpu, opcode, operands);

        /* In some indirect addressing modes, it's possible for dec to decrement
        the operand which is also used as address. So we can no longer use the operands
        for addressing. Thus we reread from memory before calling dec and call cmp
        immediate directly with the value in RAM at that address. */
        let value = cpu.read(addr); // This won't be cycle accurate, might need mem_peak function
        cmp(cpu, &OPCODES[0xC9], &[value]);
    }
    pub(super) fn sbx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        let reg = cpu.registers.a & cpu.registers.x;
        cpu.registers.x = reg.wrapping_sub(operands[0]);

        // Do a compare, but on (A AND X) instead of normally one
        update_zn_flags(cpu, cpu.registers.x);
        if reg >= operands[0] {
            cpu.registers.p |= StatusFlags::C;
        } else {
            cpu.registers.p &= !StatusFlags::C;
        }
    }
    pub(super) fn isc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        inc(cpu, opcode, operands);
        sbc(cpu, opcode, operands);
    }
    pub(super) fn usb(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) {
        sbc(cpu, opcode, operands);
    }
}
