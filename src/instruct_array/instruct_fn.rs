use crate::hardware::Cpu;
use crate::hardware::Flag::*;
use std::io::{stdin, stdout, Read, Write};

//0x00
pub fn nop(_cpu: &mut Cpu) {}

/*****************8 bit direct load***********************/
//0x3E
pub fn ld_a_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_a(n);
}
//0x06
pub fn ld_b_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_b(n);
}
//0x0e
pub fn ld_c_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_c(n);
}
//0x16
pub fn ld_d_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_d(n);
}
//0x1e
pub fn ld_e_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_e(n);
}
//0x26
pub fn ld_h_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_h(n);
}
//0x2e
pub fn ld_l_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_l(n);
}
/*****************Load A***********************/
//0x7f
pub fn ld_a_a(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_a());
}
//0x78
pub fn ld_a_b(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_b());
}
//0x79
pub fn ld_a_c(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_c());
}
//0x7a
pub fn ld_a_d(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_d());
}
//0x7b
pub fn ld_a_e(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_e());
}
//0x7C
pub fn ld_a_h(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_h());
}
//0x7D
pub fn ld_a_l(cpu: &mut Cpu) {
    cpu.set_a(cpu.get_l());
}
//0x0A
pub fn ld_a_bcp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[cpu.get_bc() as usize]);
}
//0x1A
pub fn ld_a_dep(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[cpu.get_de() as usize]);
}
//0x7E
pub fn ld_a_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[cpu.get_hl() as usize]);
}
//0xFA
pub fn ld_a_u16p(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[Cpu::get_u16(h, l) as usize]);
}

/*****************Load B***********************/
//0x40
pub fn ld_b_b(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_b());
}
//0x41
pub fn ld_b_c(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_c());
}
//0x42
pub fn ld_b_d(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_d());
}
//0x43
pub fn ld_b_e(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_e());
}
//0x44
pub fn ld_b_h(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_h());
}
//0x45
pub fn ld_b_l(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_l());
}
//0x46
pub fn ld_b_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_b(ram[cpu.get_hl() as usize]);
}
//0x47
pub fn ld_b_a(cpu: &mut Cpu) {
    cpu.set_b(cpu.get_a());
}
/*****************Load C***********************/
//0x48
pub fn ld_c_b(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_b());
}
//0x49
pub fn ld_c_c(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_c());
}
//0x4A
pub fn ld_c_d(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_d());
}
//0x4B
pub fn ld_c_e(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_e());
}
//0x4C
pub fn ld_c_h(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_h());
}
//0x4D
pub fn ld_c_l(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_l());
}
//0x4C
pub fn ld_c_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_c(ram[cpu.get_hl() as usize]);
}
//0x4F
pub fn ld_c_a(cpu: &mut Cpu) {
    cpu.set_c(cpu.get_a());
}
/*****************Load D***********************/
//0x50
pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_b());
}
//0x51
pub fn ld_d_c(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_c());
}
//0x52
pub fn ld_d_d(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_d());
}
//0x53
pub fn ld_d_e(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_e());
}
//0x54
pub fn ld_d_h(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_h());
}
//0x55
pub fn ld_d_l(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_l());
}
//0x56
pub fn ld_d_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_d(ram[cpu.get_hl() as usize]);
}
//0x57
pub fn ld_d_a(cpu: &mut Cpu) {
    cpu.set_d(cpu.get_a());
}
/*****************Load E***********************/
//0x58
pub fn ld_e_b(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_b());
}
//0x59
pub fn ld_e_c(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_c());
}
//0x5A
pub fn ld_e_d(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_d());
}
//0x5B
pub fn ld_e_e(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_e());
}
//0x5C
pub fn ld_e_h(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_h());
}
//0x5D
pub fn ld_e_l(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_l());
}
//0x5E
pub fn ld_e_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_e(ram[cpu.get_hl() as usize]);
}
//0x5F
pub fn ld_e_a(cpu: &mut Cpu) {
    cpu.set_e(cpu.get_a());
}
/*****************Load H***********************/
//0x60
pub fn ld_h_b(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_b());
}
//0x61
pub fn ld_h_c(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_c());
}
//0x62
pub fn ld_h_d(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_d());
}
//0x63
pub fn ld_h_e(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_e());
}
//0x64
pub fn ld_h_h(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_h());
}
//0x65
pub fn ld_h_l(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_l());
}
//0x66
pub fn ld_h_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_h(ram[cpu.get_hl() as usize]);
}
//0x67
pub fn ld_h_a(cpu: &mut Cpu) {
    cpu.set_h(cpu.get_a());
}
/*****************Load L***********************/
//0x68
pub fn ld_l_b(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_b());
}
//0x69
pub fn ld_l_c(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_c());
}
//0x6A
pub fn ld_l_d(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_d());
}
//0x6B
pub fn ld_l_e(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_e());
}
//0x6C
pub fn ld_l_h(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_h());
}
//0x6D
pub fn ld_l_l(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_l());
}
//0x6E
pub fn ld_l_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_l(ram[cpu.get_hl() as usize]);
}
//0x6F
pub fn ld_l_a(cpu: &mut Cpu) {
    cpu.set_l(cpu.get_a());
}
/*****************Load (HL)***********************/
//0x70
pub fn ld_hlp_b(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_b(), cpu.get_hl());
}
//0x71
pub fn ld_hlp_c(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_c(), cpu.get_hl());
}
//0x72
pub fn ld_hlp_d(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_d(), cpu.get_hl());
}
//0x73
pub fn ld_hlp_e(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_e(), cpu.get_hl());
}
//0x74
pub fn ld_hlp_h(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_h(), cpu.get_hl());
}
//0x75
pub fn ld_hlp_l(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_l(), cpu.get_hl());
}
//0x0x36
pub fn ld_hlp_u8(cpu: &mut Cpu, n: u8, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, n, cpu.get_hl());
}
//0x77
pub fn ld_hlp_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), cpu.get_hl());
}

/*****************Load n from A***********************/
//0x02
pub fn ld_bcp_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), cpu.get_bc());
}
//0x12
pub fn ld_dep_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), cpu.get_de());
}
//0xEA
pub fn ld_u16p_a(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), Cpu::get_u16(h, l));
}
/*****************Load A and HRam+C***********************/
//0xF2
pub fn ld_a_hram_c(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[(0xff00 + cpu.get_c() as u16) as usize]);
}
//0xE2
pub fn ld_hram_c_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), 0xff00 + cpu.get_c() as u16);
}
/*****************Load A and decrease/increase HL***********************/
//0x3A //TODO revoir le system de read?
pub fn ldd_a_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[cpu.get_hl() as usize]);
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x32
pub fn ldd_hlp_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), cpu.get_hl());
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x2A
pub fn ldi_a_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[cpu.get_hl() as usize]);
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
//0x22
pub fn ldi_hlp_a(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), cpu.get_hl());
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
/*****************Load A and HRam + n***********************/
//0xF0 //TODO revoir le system de read?
pub fn ldh_a_u8(cpu: &mut Cpu, n: u8, ram: &mut [u8; 0x10000]) {
    cpu.set_a(ram[(0xff00 + n as u16) as usize]);
}
//0xE0
pub fn ldh_u8_a(cpu: &mut Cpu, n: u8, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_a(), 0xff00 + n as u16);
}
/*****************16 bits direct loads***********************/
//0x01
pub fn ld_bc_u16(cpu: &mut Cpu, h: u8, l: u8) {
    cpu.set_b(h);
    cpu.set_c(l);
}
//0x11
pub fn ld_de_u16(cpu: &mut Cpu, h: u8, l: u8) {
    cpu.set_d(h);
    cpu.set_e(l);
}
//0x21
pub fn ld_hl_u16(cpu: &mut Cpu, h: u8, l: u8) {
    cpu.set_h(h);
    cpu.set_l(l);
}
//0x31
pub fn ld_sp_u16(cpu: &mut Cpu, h: u8, l: u8) {
    cpu.set_sp(Cpu::get_u16(h, l));
}
/*****************SP related loads***********************/
//0xF9
pub fn ld_sp_hl(cpu: &mut Cpu) {
    cpu.set_sp(cpu.get_hl());
}
/*0xF8
Complex instruction, the Gameboy use Two's complement signed number
We use smartly rust cast function to avoid having to rotate the bits
 */
pub fn ld_hl_sp_i8(cpu: &mut Cpu, n: u8) {
    match cpu.get_sp().checked_add(n as u16) {
        //checking for an overflow
        None => cpu.set_flag(C),
        Some(_) => cpu.clear_flag(C),
    }
    if (cpu.get_sp() & 0x0f) + (n & 0x0f) as u16 > 0x0f {
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H);
    }

    cpu.clear_flag(Z);
    cpu.clear_flag(N);
    cpu.set_sp((cpu.get_sp() as i16).wrapping_add((n as i8) as i16) as u16);
}
//0x08
pub fn ld_u16_sp(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    cpu.write(ram, cpu.get_sp() as u8, Cpu::get_u16(h, l));
    cpu.write(ram, (cpu.get_sp() >> 8) as u8, Cpu::get_u16(h, l) + 1);
}

//0xF3 (4tics)
pub fn di(cpu: &mut Cpu) {
    cpu.set_mie(false);
}

//0xFB (4tics)
pub fn ei(_cpu: &mut Cpu) {
    //Since you there is a delay of instruction
    //before enabling interrupts it's handle in the Master::step method
}
/*****************XOR***********************/
//0xEE
pub fn xor_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_a(cpu.get_a() ^ n);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.clear_flag(N);
    if cpu.get_a() == 0 {
        cpu.set_flag(Z)
    } else {
        cpu.clear_flag(Z);
    }
}
//0xA8
pub fn xor_b(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_b());
}

//0xA9
pub fn xor_c(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_c());
}
//0xAA
pub fn xor_d(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_d());
}
//0xAB
pub fn xor_e(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_e());
}
//0xAC
pub fn xor_h(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_h());
}
//0xAD
pub fn xor_l(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_l());
}
//0xAE
pub fn xor_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    xor_u8(cpu, ram[cpu.get_hl() as usize]);
}
//0xAF
pub fn xor_a(cpu: &mut Cpu) {
    xor_u8(cpu, cpu.get_a());
}

/*****************JUMP***********************/
//0xC3
pub fn jp_u16(cpu: &mut Cpu, h: u8, l: u8) {
    cpu.set_pc(Cpu::get_u16(h, l).wrapping_sub(3));
}
//0xC2
pub fn jp_nz_u16(cpu: &mut Cpu, h: u8, l: u8) {
    if !cpu.get_flags().z {
        jp_u16(cpu, h, l);
        cpu.set_ticks(4);
    }
}
//0xCA
pub fn jp_z_u16(cpu: &mut Cpu, h: u8, l: u8) {
    if cpu.get_flags().z {
        jp_u16(cpu, h, l);
        cpu.set_ticks(4);
    }
}
//0xD2
pub fn jp_nc_u16(cpu: &mut Cpu, h: u8, l: u8) {
    if !cpu.get_flags().c {
        jp_u16(cpu, h, l);
        cpu.set_ticks(4);
    }
}
//0xDA
pub fn jp_c_u16(cpu: &mut Cpu, h: u8, l: u8) {
    if cpu.get_flags().c {
        jp_u16(cpu, h, l);
        cpu.set_ticks(4);
    }
}
//0xE9
pub fn jp_hl(cpu: &mut Cpu) {
    cpu.set_pc(cpu.get_hl().wrapping_sub(1));
}
//0x18
pub fn jpr(cpu: &mut Cpu, n: u8) {
    cpu.set_pc((cpu.get_pc() as i16).wrapping_add((n as i8) as i16) as u16);
    //TODO: Timing done?
}

//Ox20
pub fn jpr_nz(cpu: &mut Cpu, n: u8) {
    if !cpu.get_flags().z {
        jpr(cpu, n);
        cpu.set_ticks(4);
    }
}
//0x28
pub fn jpr_z(cpu: &mut Cpu, n: u8) {
    if cpu.get_flags().z {
        jpr(cpu, n);
        cpu.set_ticks(4);

    }
}
//0x30
pub fn jpr_nc(cpu: &mut Cpu, n: u8) {
    if !cpu.get_flags().c {
        jpr(cpu, n);
        cpu.set_ticks(4);

    }
}
//0x38
pub fn jpr_c(cpu: &mut Cpu, n: u8) {
    if cpu.get_flags().c {
        jpr(cpu, n);
        cpu.set_ticks(4);

    }
}

/*DEC_____________________________________________________________________*/
//0x3D
pub fn dec_a(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_a() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_a(cpu.get_a().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x05
pub fn dec_b(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_b() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_b(cpu.get_b().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_b() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x0D
pub fn dec_c(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_c() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_c(cpu.get_c().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_c() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x15
pub fn dec_d(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_d() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_d(cpu.get_d().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_d() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x1D
pub fn dec_e(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_e() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_e(cpu.get_e().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_e() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x25
pub fn dec_h(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_h() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_h(cpu.get_h().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_h() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x2D
pub fn dec_l(cpu: &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_l() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_l(cpu.get_l().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_l() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

//0x35
pub fn dec_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    //Half carry flag here used as half borrow :

    if ram[cpu.get_hl() as usize] & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
                           //we have bits in the lower nibble
    } else {
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    ram[cpu.get_hl() as usize] = ram[cpu.get_hl() as usize].wrapping_sub(1);

    cpu.set_flag(N);
    if ram[cpu.get_hl() as usize] == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}
/*****************CP***********************/
//0xFE
pub fn cp_u8(cpu: &mut Cpu, n: u8) {
    cpu.set_flag(N);
    if n > cpu.get_a() {
        cpu.set_flag(C);
    } else {
        cpu.clear_flag(C);
    }
    if cpu.get_a().wrapping_sub(n) == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
    if (n & 0x0f) > (cpu.get_a() & 0x0f) {
        //(like a - n) check for borrow
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H);
    }
}
//0xB8
pub fn cp_b(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_b());
}
//0xB9
pub fn cp_c(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_c());
}
//0xBA
pub fn cp_d(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_d());
}
//0xBB
pub fn cp_e(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_e());
}
//0xBC
pub fn cp_h(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_h());
}
//0xBD
pub fn cp_l(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_l());
}
//0xBE
pub fn cp_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cp_u8(cpu, ram[cpu.get_hl() as usize]);
}
//0xBF
pub fn cp_a(cpu: &mut Cpu) {
    cp_u8(cpu, cpu.get_a());
}

/*INC__________________________________________________________________________________*/

//0x3C
pub fn inc_a(cpu: &mut Cpu) {
    if cpu.get_a() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_a(cpu.get_a().wrapping_add(1));

    if cpu.get_a() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x04
pub fn inc_b(cpu: &mut Cpu) {
    if cpu.get_b() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_b(cpu.get_b().wrapping_add(1));

    if cpu.get_b() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x0C
pub fn inc_c(cpu: &mut Cpu) {
    if cpu.get_c() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_c(cpu.get_c().wrapping_add(1));

    if cpu.get_c() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x14
pub fn inc_d(cpu: &mut Cpu) {
    if cpu.get_d() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_d(cpu.get_d().wrapping_add(1));

    if cpu.get_d() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x1C
pub fn inc_e(cpu: &mut Cpu) {
    if cpu.get_e() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_e(cpu.get_e().wrapping_add(1));

    if cpu.get_e() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x24
pub fn inc_h(cpu: &mut Cpu) {
    if cpu.get_h() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_h(cpu.get_h().wrapping_add(1));

    if cpu.get_h() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x2C
pub fn inc_l(cpu: &mut Cpu) {
    if cpu.get_l() == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    cpu.set_l(cpu.get_l().wrapping_add(1));

    if cpu.get_l() == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x34
pub fn inc_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    if ram[cpu.get_hl() as usize] == 0x0f {
        //Half carry flag
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H)
    }

    ram[cpu.get_hl() as usize] = ram[cpu.get_hl() as usize].wrapping_add(1);

    if ram[cpu.get_hl() as usize] == 0 {
        // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//TODO: timing done ?
/*CALLS____________________________________________________________________________*/
//0xCD
pub fn call_u16(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc().wrapping_add(3), ram);
    cpu.set_pc(Cpu::get_u16(h, l).wrapping_sub(3));
}
//C4
pub fn call_nz_u16(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    if !cpu.get_flags().z {
        call_u16(cpu, h, l, ram);
        cpu.set_ticks(12);
    }
}
//CC
pub fn call_z_u16(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    if cpu.get_flags().z {
        call_u16(cpu, h, l, ram);
        cpu.set_ticks(12);
    }
}
//D4
pub fn call_nc_u16(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    if !cpu.get_flags().c {
        call_u16(cpu, h, l, ram);
        cpu.set_ticks(12);
    }
}
//DC
pub fn call_c_u16(cpu: &mut Cpu, h: u8, l: u8, ram: &mut [u8; 0x10000]) {
    if cpu.get_flags().c {
        call_u16(cpu, h, l, ram);
        cpu.set_ticks(12);
    }
}
/*****************DEC 16 bits***********************/
//0x0B
pub fn dec_bc(cpu: &mut Cpu) {
    cpu.set_bc(cpu.get_bc().wrapping_sub(1));
}
//0x1B
pub fn dec_de(cpu: &mut Cpu) {
    cpu.set_de(cpu.get_de().wrapping_sub(1));
}
//0x2B
pub fn dec_hl(cpu: &mut Cpu) {
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x3B
pub fn dec_sp(cpu: &mut Cpu) {
    cpu.set_sp(cpu.get_sp().wrapping_sub(1));
}
/*****************INC 16 bits***********************/
//0x03
pub fn inc_bc(cpu: &mut Cpu) {
    cpu.set_bc(cpu.get_bc().wrapping_add(1));
}
//0x13
pub fn inc_de(cpu: &mut Cpu) {
    cpu.set_de(cpu.get_de().wrapping_add(1));
}
//0x23
pub fn inc_hl(cpu: &mut Cpu) {
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
//0x33
pub fn inc_sp(cpu: &mut Cpu) {
    cpu.set_sp(cpu.get_sp().wrapping_add(1));
}
/*****************OR***********************/
//0xB7
pub fn or_a(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_a());
}
//0xB0
pub fn or_b(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_b());
}
//0xB1
pub fn or_c(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_c());
}
//0xB2
pub fn or_d(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_d());
}
//0xB3
pub fn or_e(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_e());
}
//0xB4
pub fn or_h(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_h());
}
//0xB5
pub fn or_l(cpu: &mut Cpu) {
    or_u8(cpu, cpu.get_l());
}
//0xB6
pub fn or_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    or_u8(cpu, ram[cpu.get_hl() as usize]);
}
//0xF6
pub fn or_u8(cpu: &mut Cpu, n: u8) {
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | n);
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}
/*****************Returns***********************/
//0xC9 //TODO TIMING
pub fn ret(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let n = cpu.read_u16_from_stack(ram).wrapping_sub(1);
    cpu.set_pc(n);
}
//0xC0
pub fn ret_nz(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    if !cpu.get_flags().z {
        ret(cpu, ram);
        cpu.set_ticks(12);
    }
}
//0xC8
pub fn ret_z(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    if cpu.get_flags().z {
        ret(cpu, ram);
        cpu.set_ticks(12);
    }
}
//0xD0
pub fn ret_nc(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    if !cpu.get_flags().c {
        ret(cpu, ram);
        cpu.set_ticks(12);
    }
}
//0xD8
pub fn ret_c(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    if cpu.get_flags().c {
        ret(cpu, ram);
        cpu.set_ticks(12);
    }
}
//0xD9
pub fn ret_i(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.set_mie(true);
    let pc: u16 = cpu.read_u16_from_stack(&ram).wrapping_sub(1);
    cpu.set_pc(pc);
}
/*****************SP related PUSH***********************/
//0xF5
pub fn push_af(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_af(), ram);
}
//0xC5
pub fn push_bc(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_bc(), ram);
}
//0xD5
pub fn push_de(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_de(), ram);
}
//0xE5
pub fn push_hl(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_hl(), ram);
}
/*****************SP related POP***********************/
//0xF1
pub fn pop_af(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_af(n);
}
//0xC1
pub fn pop_bc(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_bc(n);
}
//0xDE
pub fn pop_de(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_de(n);
}
//0xhl
pub fn pop_hl(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_hl(n);
}
/*****************AND***********************/
//0xA7
pub fn and_a(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_a());
}
//0xA0
pub fn and_b(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_b());
}
//0xA1
pub fn and_c(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_c());
}
//0xA2
pub fn and_d(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_d());
}
//0xA3
pub fn and_e(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_e());
}
//0xA4
pub fn and_h(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_h());
}
//0xA5
pub fn and_l(cpu: &mut Cpu) {
    and_u8(cpu, cpu.get_l());
}
//0xA6
pub fn and_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    and_u8(cpu, ram[cpu.get_hl() as usize]);
}
//0xE6
pub fn and_u8(cpu: &mut Cpu, n: u8) {
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & n);
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}
//MISC

//0x27
pub fn daa(cpu: &mut Cpu) {
    if !cpu.get_flags().n {
        if cpu.get_flags().c || cpu.get_a() > 0x99 {
            cpu.set_a(cpu.get_a().wrapping_add(0x60));
            cpu.set_flag(C);
        }
        if cpu.get_flags().h || (cpu.get_a() & 0x0f) > 0x09 {
            cpu.set_a(cpu.get_a().wrapping_add(0x06));
        }
    } else {
        if cpu.get_flags().c {
            cpu.set_a(cpu.get_a().wrapping_sub(0x60));
        }
        if cpu.get_flags().h {
            cpu.set_a(cpu.get_a().wrapping_sub(0x6));
        }
    }
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
    cpu.clear_flag(H);
}

//0x2F
pub fn cpl(cpu: &mut Cpu) {
    cpu.set_flag(H);
    cpu.set_flag(N);
    cpu.set_a(!(cpu.get_a()));
}

//0xCB
pub fn prefix(cpu: &mut Cpu, n: u8, ram: &mut [u8; 0x10000]) {
    cpu.set_ticks(8);
    match n {
        0x27 => {
            //Generalize Sla?
            //println!("/!\\ SLA A operation occurred");
            cpu.clear_flag(H);
            cpu.clear_flag(N);
            if cpu.get_a() & 0x80 > 0 {
                cpu.set_flag(C);
            } else {
                cpu.clear_flag(C);
            }
            cpu.set_a(cpu.get_a() << 1);
            if cpu.get_a() == 0 {
                cpu.set_flag(Z);
            } else {
                cpu.clear_flag(Z);
            }
        }
        0x33 => {
            //println!("/!\\ SWAP E operation occurred");
            cpu.set_e((cpu.get_e() << 4) | (cpu.get_e() >> 4));
            cpu.set_flag(Z);
            cpu.clear_flag(H);
            cpu.clear_flag(C);
            cpu.clear_flag(N);
        }

        0x37 => {
            //println!("/!\\ SWAP A operation occurred");
            cpu.set_a((cpu.get_a() << 4) | (cpu.get_a() >> 4));
            cpu.set_flag(Z);
            cpu.clear_flag(H);
            cpu.clear_flag(C);
            cpu.clear_flag(N);
        }
        0x3f => {
            //println!("/!\\ SRL A operation occurred");
            cpu.clear_flag(H);
            cpu.clear_flag(N);
            if cpu.get_a() & 0x01 > 0 {
                cpu.set_flag(C);
            } else {
                cpu.clear_flag(C);
            }
            cpu.set_a(cpu.get_a() >> 1);
            if cpu.get_a() == 0 {
                cpu.set_flag(Z);
            } else {
                cpu.clear_flag(Z);
            }
        }

        0x40 => {
            //println!("/!\\ BIT 0 B operation occurred");
            bit(cpu, cpu.get_b(), 0);
        }
        0x41 => {
            //println!("/!\\ BIT 0 C operation occurred");
            bit(cpu, cpu.get_c(), 0);
        }
        0x42 => {
            //println!("/!\\ BIT 0 D operation occurred");
            bit(cpu, cpu.get_d(), 0);
        }
        0x43 => {
            //println!("/!\\ BIT 0 E operation occurred");
            bit(cpu, cpu.get_e(), 0);
        }
        0x44 => {
            //println!("/!\\ BIT 0 H operation occurred");
            bit(cpu, cpu.get_h(), 0);
        }
        0x45 => {
            //println!("/!\\ BIT 0 L operation occurred");
            bit(cpu, cpu.get_l(), 0);
        }
        0x46 => {
            //TODO TIMING
            //println!("/!\\ BIT 0 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 0);
        }
        0x47 => {
            //println!("/!\\ BIT 0 A operation occurred");
            bit(cpu, cpu.get_a(), 0);
        }
        0x48 => {
            //println!("/!\\ BIT 1 B operation occurred");
            bit(cpu, cpu.get_b(), 1);
        }
        0x49 => {
            //println!("/!\\ BIT 1 C operation occurred");
            bit(cpu, cpu.get_c(), 1);
        }
        0x4A => {
            //println!("/!\\ BIT 1 D operation occurred");
            bit(cpu, cpu.get_d(), 1);
        }
        0x4B => {
            //println!("/!\\ BIT 1 E operation occurred");
            bit(cpu, cpu.get_e(), 1);
        }
        0x4C => {
            //println!("/!\\ BIT 1 H operation occurred");
            bit(cpu, cpu.get_h(), 1);
        }
        0x4D => {
            //println!("/!\\ BIT 1 L operation occurred");
            bit(cpu, cpu.get_l(), 1);
        }
        0x4E => {
            //TODO TIMING
            //println!("/!\\ BIT 1 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 1);
        }
        0x4F => {
            //println!("/!\\ BIT 1 A operation occurred");
            bit(cpu, cpu.get_a(), 1);
        }
        0x50 => {
            //println!("/!\\ BIT 2 B operation occurred");
            bit(cpu, cpu.get_b(), 2);
        }
        0x51 => {
            //println!("/!\\ BIT 2 C operation occurred");
            bit(cpu, cpu.get_c(), 2);
        }
        0x52 => {
            //println!("/!\\ BIT 2 D operation occurred");
            bit(cpu, cpu.get_d(), 2);
        }
        0x53 => {
            //println!("/!\\ BIT 2 E operation occurred");
            bit(cpu, cpu.get_e(), 2);
        }
        0x54 => {
            //println!("/!\\ BIT 2 H operation occurred");
            bit(cpu, cpu.get_h(), 2);
        }
        0x55 => {
            //println!("/!\\ BIT 2 L operation occurred");
            bit(cpu, cpu.get_l(), 2);
        }
        0x56 => {
            //TODO Timing
            //println!("/!\\ BIT 2 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 2);
        }
        0x57 => {
            //println!("/!\\ BIT 2 A operation occurred");
            bit(cpu, cpu.get_a(), 2);
        }
        0x58 => {
            //println!("/!\\ BIT 3 B operation occurred");
            bit(cpu, cpu.get_b(), 3);
        }
        0x59 => {
            //println!("/!\\ BIT 3 C operation occurred");
            bit(cpu, cpu.get_c(), 3);
        }
        0x5A => {
            //println!("/!\\ BIT 3 D operation occurred");
            bit(cpu, cpu.get_d(), 3);
        }
        0x5B => {
            //println!("/!\\ BIT 3 E operation occurred");
            bit(cpu, cpu.get_e(), 3);
        }
        0x5C => {
            //println!("/!\\ BIT 3 H operation occurred");
            bit(cpu, cpu.get_h(), 3);
        }
        0x5D => {
            //println!("/!\\ BIT 3 L operation occurred");
            bit(cpu, cpu.get_l(), 3);
        }
        0x5E => {
            //TODO Timing
            //println!("/!\\ BIT 3 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 3);
        }
        0x5F => {
            //println!("/!\\ BIT 3 A operation occurred");
            bit(cpu, cpu.get_a(), 3);
        }
        0x60 => {
            //println!("/!\\ BIT 4 B operation occurred");
            bit(cpu, cpu.get_b(), 4);
        }
        0x61 => {
            //println!("/!\\ BIT 4 C operation occurred");
            bit(cpu, cpu.get_c(), 4);
        }
        0x62 => {
            //println!("/!\\ BIT 4 D operation occurred");
            bit(cpu, cpu.get_d(), 4);
        }
        0x63 => {
            //println!("/!\\ BIT 4 E operation occurred");
            bit(cpu, cpu.get_e(), 4);
        }
        0x64 => {
            //println!("/!\\ BIT 4 H operation occurred");
            bit(cpu, cpu.get_h(), 4);
        }
        0x65 => {
            //println!("/!\\ BIT 4 L operation occurred");
            bit(cpu, cpu.get_l(), 4);
        }
        0x66 => {
            //TODO timing
            //println!("/!\\ BIT 4 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 4);
        }
        0x67 => {
            //println!("/!\\ BIT 4 A operation occurred");
            bit(cpu, cpu.get_a(), 4);
        }
        0x68 => {
            //println!("/!\\ BIT 5 B operation occurred");
            bit(cpu, cpu.get_b(), 5);
        }
        0x69 => {
            //println!("/!\\ BIT 5 C operation occurred");
            bit(cpu, cpu.get_c(), 5);
        }
        0x6A => {
            //println!("/!\\ BIT 5 D operation occurred");
            bit(cpu, cpu.get_d(), 5);
        }
        0x6B => {
            //println!("/!\\ BIT 5 E operation occurred");
            bit(cpu, cpu.get_e(), 5);
        }
        0x6C => {
            //println!("/!\\ BIT 5 H operation occurred");
            bit(cpu, cpu.get_h(), 5);
        }
        0x6D => {
            //println!("/!\\ BIT 5 L operation occurred");
            bit(cpu, cpu.get_l(), 5);
        }
        0x6E => {
            //TODO Timing
            //println!("/!\\ BIT 5 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 5);
        }
        0x6F => {
            //println!("/!\\ BIT 5 A operation occurred");
            bit(cpu, cpu.get_a(), 5);
        }
        0x70 => {
            //println!("/!\\ BIT 6 B operation occurred");
            bit(cpu, cpu.get_b(), 6);
        }
        0x71 => {
            //println!("/!\\ BIT 6 C operation occurred");
            bit(cpu, cpu.get_c(), 6);
        }
        0x72 => {
            //println!("/!\\ BIT 6 D operation occurred");
            bit(cpu, cpu.get_d(), 6);
        }
        0x73 => {
            //println!("/!\\ BIT 6 E operation occurred");
            bit(cpu, cpu.get_e(), 6);
        }
        0x74 => {
            //println!("/!\\ BIT 6 H operation occurred");
            bit(cpu, cpu.get_h(), 6);
        }
        0x75 => {
            //println!("/!\\ BIT 6 L operation occurred");
            bit(cpu, cpu.get_l(), 6);
        }
        0x76 => {
            ////println!("/!\\ BIT 6 (HL) operation occurred");
            bit(cpu, ram[cpu.get_hl() as usize], 6);
            cpu.set_ticks(8); //16bits register, take more time
        }
        0x77 => {
            //println!("/!\\ BIT 6 A operation occurred");
            bit(cpu, cpu.get_a(), 6);
        }
        0x78 => {
            //println!("/!\\ BIT 7 B operation occurred");
            bit(cpu, cpu.get_b(), 7);
        }
        0x79 => {
            //println!("/!\\ BIT 7 C operation occurred");
            bit(cpu, cpu.get_c(), 7);
        }
        0x7A => {
            //println!("/!\\ BIT 7 D operation occurred");
            bit(cpu, cpu.get_d(), 7);
        }
        0x7B => {
            //println!("/!\\ BIT 7 E operation occurred");
            bit(cpu, cpu.get_e(), 7);
        }
        0x7C => {
            //println!("/!\\ BIT 7 H operation occurred");
            bit(cpu, cpu.get_h(), 7);
        }
        0x7D => {
            //println!("/!\\ BIT 7 L operation occurred");
            bit(cpu, cpu.get_l(), 7);
        }
        0x7E => {
            //println!("/!\\ BIT 7 (HL) operation occurred");
            cpu.set_ticks(8); //16bits register, take more time
            bit(cpu, ram[cpu.get_hl() as usize], 7);
        }
        0x7F => {
            //println!("/!\\ BIT 7 L operation occurred");
            bit(cpu, cpu.get_a(), 7);
        }
        0x86 => {
            //println!("/!\\ Res 0 (HL) operation occurred");
            cpu.write(ram, ram[cpu.get_hl() as usize] & 0xfe, cpu.get_hl());
        }
        0x87 => {
            //println!("/!\\ Res 0 A operation occurred");
            cpu.set_a(cpu.get_a() & 0xfe);
        }
        0x9e => {
            //println!("/!\\ Res 3 (HL) operation occurred");
            cpu.write(ram, ram[cpu.get_hl() as usize] & 0b11110111, cpu.get_hl());
        }
        0xBE => {
            //println!("/!\\ Res 7 (hl) operation occurred");
            cpu.write(ram, ram[cpu.get_hl() as usize] & 0xef, cpu.get_hl());
        }
        0xDE => {
            //println!("/!\\ SET 3 (HL) operation occurred");
            cpu.write(ram, ram[cpu.get_hl() as usize] | 0b1000, cpu.get_hl());
        }
        0xFE => {
            //println!("/!\\ SET 7 (HL) operation occurred");
            cpu.write(ram, ram[cpu.get_hl() as usize] | 0b10000000, cpu.get_hl());
        }
        _ => {
            println!("/!\\ {:#04x} Not done yet", n);
            wait();
        }
    }
}

fn bit(cpu: &mut Cpu, n: u8, bit: u8) {

    cpu.set_flag(H);
    cpu.clear_flag(N);
    if n & (1 << bit) == 0 {
        //check 7th bit
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z);
    }
}

/*RESET_____________________________________________________________________*/
//C7
pub fn rst_0(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0xFFFF);
}
//CF
pub fn rst_8(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x0007);
}

//D7
pub fn rst_10(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x000f);
}

//DF
pub fn rst_18(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x0017);
}
//E7
pub fn rst_20(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x001f);
}

//EF
pub fn rst_28(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x0027);
}

//F7
pub fn rst_30(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x002f);
}

//FF
pub fn rst_38(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.write_u16_to_stack(cpu.get_pc() + 1, ram);
    cpu.set_pc(0x0037);
}
/*****************Add n to A***********************/
//0x87
pub fn add_a(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_a());
}
//0x80
pub fn add_b(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_b());
}
//0x81
pub fn add_c(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_c());
}
//0x82
pub fn add_d(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_d());
}
//0x83
pub fn add_e(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_e());
}
//0x84
pub fn add_h(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_h());
}
//0x85
pub fn add_l(cpu: &mut Cpu) {
    add_u8(cpu, cpu.get_l());
}
//0x86
pub fn add_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    add_u8(cpu, ram[cpu.get_hl() as usize]);
}
//0xC6
pub fn add_u8(cpu: &mut Cpu, n: u8) {
    match cpu.get_a().checked_add(n) {
        None => {
            cpu.set_flag(C);
        }
        Some(_) => {
            cpu.clear_flag(C);
        }
    }
    if (cpu.get_a() & 0x0f) + (n & 0x0f) > 0x0f {
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H);
    }

    cpu.set_a(cpu.get_a().wrapping_add(n));
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    }
    cpu.clear_flag(N);
}
/*****************ADD 16 bits***********************/
//Add_u16
fn add_hl_u16(cpu: &mut Cpu, n: u16) {
    match cpu.get_hl().checked_add(n) {
        None => {
            cpu.set_flag(C);
        }
        Some(_) => {
            cpu.clear_flag(C);
        }
    }
    if cpu.get_h() & 0x0f + ((n >> 8) as u8 & 0x0f) > 0x0f {
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H);
    }
    cpu.set_hl(cpu.get_hl().wrapping_add(n));
    cpu.clear_flag(N);
}
//0x09
pub fn add_hl_bc(cpu: &mut Cpu) {
    add_hl_u16(cpu, cpu.get_bc());
}
//0x19
pub fn add_hl_de(cpu: &mut Cpu) {
    add_hl_u16(cpu, cpu.get_de());
}
//0x29
pub fn add_hl_hl(cpu: &mut Cpu) {
    add_hl_u16(cpu, cpu.get_hl());
}
//0x39
pub fn add_hl_sp(cpu: &mut Cpu) {
    add_hl_u16(cpu, cpu.get_sp());
}

pub fn wait() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    print!("{esc}c", esc = 27 as char);
}

//0xD6
pub fn sub_u8(cpu: &mut Cpu, n: u8) {
    match cpu.get_a().checked_sub(n) {
        None => {
            cpu.set_flag(C);
        }
        Some(_) => {
            cpu.clear_flag(C);
        }
    }
    if cpu.get_a() & 0x0f < n & 0x0f {
        cpu.set_flag(H);
    } else {
        cpu.clear_flag(H);
    }

    cpu.set_a(cpu.get_a().wrapping_sub(n));
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    }
    cpu.set_flag(N);
}
//0x97
pub fn sub_a(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_a());
}
//0x90
pub fn sub_b(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_b());
}
//0x91
pub fn sub_c(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_c());
}
//0x92
pub fn sub_d(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_d());
}
//0x93
pub fn sub_e(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_e());
}
//0x94
pub fn sub_h(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_h());
}
//0x95
pub fn sub_l(cpu: &mut Cpu) {
    sub_u8(cpu, cpu.get_l());
}
//0x96
pub fn sub_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    sub_u8(cpu, ram[cpu.get_hl() as usize]);
}

pub fn rlca(cpu: &mut Cpu) {
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(Z);

    if cpu.get_a() >> 7 > 0 {
        cpu.set_flag(C);
    } else {
        cpu.clear_flag(C);
    }

    cpu.set_a((cpu.get_a() << 1) | (cpu.get_a() >> 7));
}
//0xCE
pub fn adc_u8(cpu: &mut Cpu, n: u8) {
    if cpu.get_flags().c {
        add_u8(cpu, n.wrapping_add(1));
    } else {
        add_u8(cpu, n);
    }
}
//0x8F
pub fn adc_a(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_a());
}
//0x88
pub fn adc_b(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_b());
}
//0x89
pub fn adc_c(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_c());
}
//0x8A
pub fn adc_d(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_d());
}
//0x8B
pub fn adc_e(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_e());
}
//0x8C
pub fn adc_h(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_h());
}
//0x8D
pub fn adc_l(cpu: &mut Cpu) {
    adc_u8(cpu, cpu.get_l());
}
//0x8E
pub fn adc_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    adc_u8(cpu, ram[cpu.get_hl() as usize]);
}

//0xDE
pub fn sbc_u8(cpu: &mut Cpu, n: u8) {
    if cpu.get_flags().c {
        sub_u8(cpu, n.wrapping_add(1));
    } else {
        sub_u8(cpu, n);
    }
}
//0x9F
pub fn sbc_a(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_a());
}
//0x98
pub fn sbc_b(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_b());
}
//0x99
pub fn sbc_c(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_c());
}
//0x9A
pub fn sbc_d(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_d());
}
//0x9B
pub fn sbc_e(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_e());
}
//0x9C
pub fn sbc_h(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_h());
}
//0x9D
pub fn sbc_l(cpu: &mut Cpu) {
    sbc_u8(cpu, cpu.get_l());
}
//0x9E
pub fn sbc_hlp(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    sbc_u8(cpu, ram[cpu.get_hl() as usize]);
}
