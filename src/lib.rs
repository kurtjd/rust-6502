#![allow(dead_code)]
#![allow(unused_variables)]

use bitflags::bitflags;

const MEM_SIZE: usize = 0x10000;
const STACK_OFFSET: usize = 0x0100;
const RESET_VECTOR: usize = 0xFFFC;

struct Opcode {
    instr: fn(&mut Cpu6502, &Opcode, &[u8]) -> u8,
    mode: AddrMode,
    bytes: u8,
    cycles: u8
}

static OPCODES: [Opcode; 0x100] = [
    // $00-$0F
    Opcode { instr: instructions::brk, mode: AddrMode::IMP0, bytes: 1, cycles: 7 },
    Opcode { instr: instructions::ora, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::slo, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::ora, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::asl, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::slo, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::php, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::ora, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::asl, mode: AddrMode::ACM0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::anc, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ora, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::asl, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::slo, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },

    // $10 - $1F
    Opcode { instr: instructions::bpl, mode: AddrMode::REL0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::ora, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::slo, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::ora, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::asl, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::slo, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::clc, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::ora, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::slo, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ora, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::asl, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::slo, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },

    // $20 - $2F
    Opcode { instr: instructions::jsr, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::and, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::rla, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::bit, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::and, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::rol, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::rla, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::plp, mode: AddrMode::IMP0, bytes: 1, cycles: 4 },
    Opcode { instr: instructions::and, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::rol, mode: AddrMode::ACM0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::anc, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::bit, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::and, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::rol, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::rla, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },

    // $30 - $3F
    Opcode { instr: instructions::bmi, mode: AddrMode::REL0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::and, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::rla, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::and, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::rol, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::rla, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sec, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::and, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::rla, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::and, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::rol, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::rla, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },

    // $40 - $4F
    Opcode { instr: instructions::rti, mode: AddrMode::IMP0, bytes: 1, cycles: 6 },
    Opcode { instr: instructions::eor, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::sre, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::eor, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::lsr, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::sre, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::pha, mode: AddrMode::IMP0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::eor, mode: AddrMode::IMM0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::lsr, mode: AddrMode::ACM0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::alr, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::jmp, mode: AddrMode::ABS0, bytes: 3, cycles: 3 },
    Opcode { instr: instructions::eor, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lsr, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::sre, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    
    // $50 - $5F
    Opcode { instr: instructions::bvc, mode: AddrMode::REL0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::eor, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::sre, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::eor, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::lsr, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sre, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::cli, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::eor, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::sre, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::eor, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lsr, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::sre, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },

    // $60 - $6F
    Opcode { instr: instructions::rts, mode: AddrMode::IMP0, bytes: 1, cycles: 6 },
    Opcode { instr: instructions::adc, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::rra, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::adc, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::ror, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::rra, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::pla, mode: AddrMode::IMP0, bytes: 1, cycles: 4 },
    Opcode { instr: instructions::adc, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::ror, mode: AddrMode::ACM0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::arr, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::jmp, mode: AddrMode::IND0, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::adc, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ror, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::rra, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },

    // $70 - $7F
    Opcode { instr: instructions::bvs, mode: AddrMode::REL0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::adc, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::rra, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::adc, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::ror, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::rra, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sei, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::adc, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::rra, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::adc, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ror, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::rra, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },

    // $80 - $8F
    Opcode { instr: instructions::nop, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::sta, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::sax, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sty, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::sta, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::stx, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::sax, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::dey, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::txa, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::ane, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::sty, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::sta, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::stx, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::sax, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },

    // $90 - $9F
    Opcode { instr: instructions::bcc, mode: AddrMode::REL0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::sta, mode: AddrMode::INDY, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::sha, mode: AddrMode::INDY, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sty, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::sta, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::stx, mode: AddrMode::ZPGY, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::sax, mode: AddrMode::ZPGY, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::tya, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::sta, mode: AddrMode::ABSY, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::txs, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::tas, mode: AddrMode::ABSY, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::shy, mode: AddrMode::ABSX, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::sta, mode: AddrMode::ABSX, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::shx, mode: AddrMode::ABSY, bytes: 3, cycles: 5 },
    Opcode { instr: instructions::sha, mode: AddrMode::ABSY, bytes: 3, cycles: 5 },

    // $A0 - $AF
    Opcode { instr: instructions::ldy, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::lda, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::ldx, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::lax, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::ldy, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::lda, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::ldx, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::lax, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::tay, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::lda, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::tax, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::lxa, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::ldy, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lda, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ldx, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lax, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },

    // $B0 - $BF
    Opcode { instr: instructions::bcs, mode: AddrMode::REL0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::lda, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::lax, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::ldy, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::lda, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::ldx, mode: AddrMode::ZPGY, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::lax, mode: AddrMode::ZPGY, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::clv, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::lda, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::tsx, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::las, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ldy, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lda, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::ldx, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::lax, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },

    // $C0 - $CF
    Opcode { instr: instructions::cpy, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::cmp, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::dcp, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::cpy, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::cmp, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::dec, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::dcp, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::iny, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::cmp, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::dex, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::sbx, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::cpy, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::cmp, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::dec, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::dcp, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },

    // $D0 - $DF
    Opcode { instr: instructions::bne, mode: AddrMode::REL0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::cmp, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::dcp, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::cmp, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::dec, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::dcp, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::cld, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::cmp, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::dcp, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::cmp, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::dec, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::dcp, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },

    // $E0 - $EF
    Opcode { instr: instructions::cpx, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::sbc, mode: AddrMode::INDX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::isc, mode: AddrMode::INDX, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::cpx, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::sbc, mode: AddrMode::ZPG0, bytes: 2, cycles: 3 },
    Opcode { instr: instructions::inc, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::isc, mode: AddrMode::ZPG0, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::inx, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::sbc, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::usb, mode: AddrMode::IMM0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::cpx, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::sbc, mode: AddrMode::ABS0, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::inc, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },
    Opcode { instr: instructions::isc, mode: AddrMode::ABS0, bytes: 3, cycles: 6 },

    // $F0 - $FF
    Opcode { instr: instructions::beq, mode: AddrMode::REL0, bytes: 2, cycles: 2 },
    Opcode { instr: instructions::sbc, mode: AddrMode::INDY, bytes: 2, cycles: 5 },
    Opcode { instr: instructions::jam, mode: AddrMode::IMP0, bytes: 1, cycles: 3 },
    Opcode { instr: instructions::isc, mode: AddrMode::INDY, bytes: 2, cycles: 8 },
    Opcode { instr: instructions::nop, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::sbc, mode: AddrMode::ZPGX, bytes: 2, cycles: 4 },
    Opcode { instr: instructions::inc, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::isc, mode: AddrMode::ZPGX, bytes: 2, cycles: 6 },
    Opcode { instr: instructions::sed, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::sbc, mode: AddrMode::ABSY, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::nop, mode: AddrMode::IMP0, bytes: 1, cycles: 2 },
    Opcode { instr: instructions::isc, mode: AddrMode::ABSY, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::nop, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::sbc, mode: AddrMode::ABSX, bytes: 3, cycles: 4 },
    Opcode { instr: instructions::inc, mode: AddrMode::ABSX, bytes: 3, cycles: 7 },
    Opcode { instr: instructions::isc, mode: AddrMode::ABSX, bytes: 3, cycles: 7 }
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
    pub p: StatusFlags  // Status
}

pub struct Cpu6502 {
    pub registers: Registers,
    pub ram: [u8; MEM_SIZE]
}

impl Cpu6502 {
    pub fn new() -> Self {
        Cpu6502 {
            registers: Registers {
                pc: 0,
                a: 0,
                x: 0,
                y: 0,
                s: 0,
                p: StatusFlags::empty()
            },

            ram: [0; MEM_SIZE]
        }
    }

    pub fn reset(&mut self) {
        // Set the PC to point to address stored in reset vector
        let lsb = self.ram[RESET_VECTOR];
        let msb = self.ram[RESET_VECTOR + 1];
        self.registers.pc = (msb as u16) << 8 | (lsb as u16);

        // Disable interrupts flag and extension bit should be set
        self.registers.p = StatusFlags::E | StatusFlags::I;
    }

    pub fn tick(&mut self) -> u8 {
        let fetch = self.ram[self.registers.pc as usize] as usize;
        let opcode = &OPCODES[fetch];

        // Find more Rusty way to handle this...
        let mut operands = [0, 0];
        for i in 0..opcode.bytes - 1 {
            operands[i as usize] = self.ram[(self.registers.pc.wrapping_add(1 + i as u16)) as usize];
        }
        
        self.registers.pc = self.registers.pc.wrapping_add(opcode.bytes as u16);
        (opcode.instr)(self, opcode, &operands)
    }
}

enum AddrMode {
    ACM0,   // Accumulator
    ABS0,   // Absolute
    ABSX,   // Absolute Indexed with X
    ABSY,   // Absolute Indexed with Y
    IMM0,   // Immediate
    IMP0,   // Implied
    IND0,   // Indirect
    INDX,   // Indirect Indexed with X
    INDY,   // Indirect Indexed with Y
    REL0,   // Relative
    ZPG0,   // Zero Page
    ZPGX,   // Zero Page Indexed Indirect with X
    ZPGY    // Zero Page Indexed Indirect with Y
}

pub mod instructions {
    use super::*;

    // For easy handling of different address modes
    // This does not seem Rusty at all so need to find better way to avoid casting and wraps everywhere
    fn get_mem(cpu: &Cpu6502, mode: &AddrMode, operands: &[u8]) -> (usize, u8, u8) {
        match mode {
            AddrMode::ABS0 => {
                let addr = (operands[1] as usize) << 8 | operands[0] as usize;
                (addr, cpu.ram[addr], 0)
            },
            AddrMode::ABSX => {
                let addr = (operands[1] as u16) << 8 | operands[0] as u16;
                let eff_addr = addr.wrapping_add(cpu.registers.x as u16);
                (eff_addr as usize, cpu.ram[eff_addr as usize], ((eff_addr & 0xFF00) != (addr & 0xFF00)) as u8)
            },
            AddrMode::ABSY => {
                let addr = (operands[1] as u16) << 8 | operands[0] as u16;
                let eff_addr = addr.wrapping_add(cpu.registers.y as u16);
                (eff_addr as usize, cpu.ram[eff_addr as usize], ((eff_addr & 0xFF00) != (addr & 0xFF00)) as u8)
            },
            AddrMode::IND0 => {
                let addr = (operands[1] as u16) << 8 | operands[0] as u16;
                let eff_addr = (cpu.ram[addr.wrapping_add(1) as usize] as usize) << 8 | cpu.ram[addr as usize] as usize;
                (eff_addr, cpu.ram[eff_addr], 0)
            },
            AddrMode::INDX => {
                let addr = (operands[0].wrapping_add(cpu.registers.x)) as u8;
                let eff_addr = (cpu.ram[addr.wrapping_add(1) as usize] as usize) << 8 | cpu.ram[addr as usize] as usize;
                (eff_addr, cpu.ram[eff_addr], 0)
            },
            AddrMode::INDY => {
                let zpaddr = operands[0];
                let addr = (cpu.ram[zpaddr.wrapping_add(1) as usize] as u16) << 8 | cpu.ram[zpaddr as usize] as u16;
                let eff_addr = addr.wrapping_add(cpu.registers.y as u16);
                (eff_addr as usize, cpu.ram[eff_addr as usize], ((eff_addr & 0xFF00) != (addr & 0xFF00)) as u8)
            },
            AddrMode::REL0 => {
                let addr = cpu.registers.pc as usize;
                let eff_addr = (addr as i32 + ((operands[0] as i8) as i32)) as usize;
                (eff_addr, cpu.ram[eff_addr], ((eff_addr & 0xFF00) != (addr & 0xFF00)) as u8)
            },
            AddrMode::ZPG0 => {
                (operands[0] as usize, cpu.ram[operands[0] as usize], 0)
            },
            AddrMode::ZPGX => {
                let eff_addr = (operands[0].wrapping_add(cpu.registers.x)) as usize;
                (eff_addr, cpu.ram[eff_addr], 0)
            },
            AddrMode::ZPGY => {
                let eff_addr = (operands[0].wrapping_add(cpu.registers.y)) as usize;
                (eff_addr, cpu.ram[eff_addr], 0)
            },
            AddrMode::ACM0 => (0, cpu.registers.a, 0),
            AddrMode::IMM0 => (0, operands[0], 0),
            AddrMode::IMP0 => (0, 0, 0)
        }
    }

    // Commonly performed by quite a few instructions
    fn update_zn_flags(cpu: &mut Cpu6502, result: u8) {
        if result == 0 {
            cpu.registers.p |= StatusFlags::Z;
        } else {
            cpu.registers.p &= !StatusFlags::Z;
        }

        if result & (1 << 7) != 0 {
            cpu.registers.p |= StatusFlags::N;
        } else {
            cpu.registers.p &= !StatusFlags::N;
        }
    }

    // Load/Store Operations
    pub (super) fn lda(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (_, value, pgx) = get_mem(cpu, &opcode.mode, operands);
        cpu.registers.a = value;
        update_zn_flags(cpu, value);
        opcode.cycles + pgx
    }
    pub (super) fn ldx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (_, value, pgx) = get_mem(cpu, &opcode.mode, operands);
        cpu.registers.x = value;
        update_zn_flags(cpu, value);
        opcode.cycles + pgx
    }
    pub (super) fn ldy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (_, value, pgx) = get_mem(cpu, &opcode.mode, operands);
        cpu.registers.y = value;
        update_zn_flags(cpu, value);
        opcode.cycles + pgx
    }

    pub (super) fn sta(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands);
        cpu.ram[addr] = cpu.registers.a;
        opcode.cycles
    }
    pub (super) fn stx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands);
        cpu.ram[addr] = cpu.registers.x;
        opcode.cycles
    }
    pub (super) fn sty(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        let (addr, _, _) = get_mem(cpu, &opcode.mode, operands);
        cpu.ram[addr] = cpu.registers.y;
        opcode.cycles
    }

    // Register Transfers
    pub (super) fn tax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        update_zn_flags(cpu, cpu.registers.a);
        cpu.registers.x = cpu.registers.a;
        opcode.cycles
    }
    pub (super) fn tay(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        update_zn_flags(cpu, cpu.registers.a);
        cpu.registers.y = cpu.registers.a;
        opcode.cycles
    }
    pub (super) fn txa(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        update_zn_flags(cpu, cpu.registers.x);
        cpu.registers.a = cpu.registers.x;
        opcode.cycles
    }
    pub (super) fn tya(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        update_zn_flags(cpu, cpu.registers.y);
        cpu.registers.a = cpu.registers.y;
        opcode.cycles
    }

    // Stack Operations
    pub (super) fn tsx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn txs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn pha(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn php(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn pla(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn plp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Logical Operations
    pub (super) fn and(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn eor(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn ora(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bit(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Arithmetic Operations
    pub (super) fn adc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sbc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn cmp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn cpx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn cpy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Inc/Dec Operations
    pub (super) fn inc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn inx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn iny(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn dec(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn dex(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn dey(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Shift Operations
    pub (super) fn asl(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn lsr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn rol(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn ror(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Jump/Call Operations
    pub (super) fn jmp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn jsr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn rts(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Branch Operations
    pub (super) fn bcc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bcs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn beq(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bmi(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bne(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bpl(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bvc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn bvs(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Status Flag Operations
    pub (super) fn clc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn cld(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn cli(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn clv(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sec(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sed(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sei(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // System Operations
    pub (super) fn brk(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn nop(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn rti(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }

    // Illegal/Undefined Operations
    pub (super) fn jam(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn slo(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn anc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn rla(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sre(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn alr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn arr(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn rra(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn ane(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sha(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn shx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn shy(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn tas(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn lax(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn las(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn lxa(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn dcp(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn sbx(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn isc(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
    pub (super) fn usb(cpu: &mut Cpu6502, opcode: &Opcode, operands: &[u8]) -> u8 {
        opcode.cycles
    }
}
