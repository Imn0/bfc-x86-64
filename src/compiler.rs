use crate::{Ins, Op};
use bincode::serialize;
use goblin::elf::{
    header::{ELFCLASS64, ELFDATA2LSB, ET_EXEC, EV_CURRENT},
    program_header::{PF_R, PF_X, PT_LOAD},
};
use serde::Serialize;
use std::io::Write;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Debug, Serialize)]
pub struct _Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Serialize)]
#[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith, Serialize))]
pub struct _ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub fn elf_header(program: Vec<u8>, file_name: &str) {
    let entry: u64 = 0x400000
        + std::mem::size_of::<_Elf64Header>() as u64
        + std::mem::size_of::<_ProgramHeader>() as u64;

    let ehdr = _Elf64Header {
        e_ident: [
            0x7f,
            b'E',
            b'L',
            b'F',
            ELFCLASS64,
            ELFDATA2LSB,
            EV_CURRENT,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        e_type: ET_EXEC as u16,
        e_machine: 0x3e, // EM_X86_64
        e_version: EV_CURRENT as u32,
        e_entry: entry,
        e_phoff: std::mem::size_of::<_Elf64Header>() as u64,
        e_ehsize: std::mem::size_of::<_Elf64Header>() as u16,
        e_phentsize: std::mem::size_of::<_ProgramHeader>() as u16,
        e_phnum: 1,
        ..Default::default()
    };

    let phdr = _ProgramHeader {
        p_type: PT_LOAD,
        p_flags: PF_X | PF_R,
        p_offset: std::mem::size_of::<_Elf64Header>() as u64
            + std::mem::size_of::<_ProgramHeader>() as u64,
        p_vaddr: entry,
        p_filesz: program.len() as u64,
        p_memsz: program.len() as u64,
        p_align: 0,
        ..Default::default()
    };

    let ehdr_bytes = serialize(&ehdr).unwrap();
    let phdr_bytes = serialize(&phdr).unwrap();
    let mut file = std::fs::File::create(file_name).unwrap();

    file.write_all(&ehdr_bytes).unwrap();
    file.write_all(&phdr_bytes).unwrap();
    file.write_all(&program).unwrap();
}

pub fn compile(program: Vec<Ins>, file_name: &str) {
    // zeroing out the memory, and pointer
    let mut output: Vec<Vec<u8>> = Vec::new();
    let header: Vec<u8> = vec![
        0x55, // push    rbp
        0x48, 0x89, 0xE5, // mov     rbp, rsp
        0x48, 0x8D, 0x95, 0xC8, 0x8A, 0xFF, 0xFF, // lea     rdx, [rbp-30008]
        0xB8, 0x00, 0x00, 0x00, 0x00, // mov     eax, 0x0
        0xB9, 0xA6, 0x0E, 0x00, 0x00, // mov     ecx, 3750 (30000 / 8)
        0x48, 0x89, 0xD7, // mov     rdi, rdx
        0xF3, 0x48, 0xAB, // rep stosq
        0x48, 0x31, 0xDB, // xor     rbx, rbx
    ];

    let mut byte_count: u64 = 0;
    let mut program_table: Vec<u64> = Vec::new();
    // let program_table: Vec<(u64, u64)> = Vec::new();

    for ins in program {
        match ins.op {
            Op::Lft => {
                program_table.push(byte_count);
                let i = vec![
                    0x48, 0x83, 0xEB, 0x01, // sub rbx, 1
                ];
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Rit => {
                program_table.push(byte_count);
                let i = vec![
                    0x48, 0x83, 0xC3, 0x01, // add rbx, 1
                ];
                byte_count += i.len() as u64;

                output.push(i);
            }
            Op::Inc => {
                let i = vec![
                    0xFE, 0x84, 0x1D, 0xC8, 0x8A, 0xFF, 0xFF, // inc byte ptr [rbp-30008+rbx]
                ];
                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Dec => {
                let i = vec![
                    0xFE, 0x8C, 0x1D, 0xC8, 0x8A, 0xFF, 0xFF, // dec byte ptr [rbp-30008+rbx]
                ];
                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Prt => {
                let i = vec![
                    0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
                    0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00, // mov rdi, 1
                    0x48, 0x8D, 0xB4, 0x1D, 0xC8, 0x8A, 0xFF, // lea rsi, [rbp-30008+rbx]
                    0xFF, //
                    0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 1
                    0x0F, 0x05, // syscall
                ];
                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Inp => {
                let i = vec![
                    0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // mov rax, 1
                    0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00, // mov rdi, 1
                    0x48, 0x8D, 0xB4, 0x1D, 0xC8, 0x8A, 0xFF, // lea rsi, [rbp-30008+rbx]
                    0xFF, //
                    0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 1
                    0x0F, 0x05, // syscall
                ];
                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Lst => {
                let i = vec![
                    0x80, 0xBC, 0x1D, 0xC8, 0x8A, 0xFF, 0xFF,
                    0x00, // cmp byte ptr [rbp-30008+rbx], 0
                    0x0F, 0x84, 0x00, 0x00, 0x00, 0x00, // je xxxxx
                ];
                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
            Op::Led => {
                let where_to_jump = program_table[ins.arg];
                let distance = byte_count as i32 - where_to_jump as i32;

                let backward = (-(distance)).to_le_bytes();

                let i = vec![
                    0x80,
                    0xBC,
                    0x1D,
                    0xC8,
                    0x8A,
                    0xFF,
                    0xFF,
                    0x00, // cmp byte ptr [rbp-30008+rbx], 0
                    0x0F,
                    0x85,
                    backward[0],
                    backward[1],
                    backward[2],
                    backward[3], // jne xxxxx
                ];

                let forward = (distance).to_le_bytes();

                let j = vec![
                    0x80, 0xBC, 0x1D, 0xC8, 0x8A, 0xFF, 0xFF,
                    0x00, // cmp byte ptr [rbp-30008+rbx], 0
                    0x0F, 0x84, forward[0], forward[1], forward[2], forward[3], // je xxxxx
                ];

                output[ins.arg] = j;

                program_table.push(byte_count);
                byte_count += i.len() as u64;
                output.push(i);
            }
        }
    }
    output.insert(0, header);

    output.push(vec![
        0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00, 0x48, 0x31, 0xFF, 0x0F, 0x05,
    ]);

    elf_header(output.concat(), file_name);
}
