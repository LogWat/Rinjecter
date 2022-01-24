use crate::processlib::Process;

use winapi::um::{winnt};
use winapi::shared::minwindef;

use winapi::um::winuser::{MB_OK, MessageBoxW};

#[derive(Copy, Clone)]
pub enum AddrSize {
    Byte(u8),
    Word(u16),
    Dword(u32),
    Qword(u64),
}

pub struct OverWrite {
    pub addr: u32,
    pub byte: AddrSize,
}

pub unsafe extern "stdcall" fn OverWrite(process: &Process) -> Result<(), &'static str> {

    let rb1: [u32; 21] = [
        0x4C4300A1, 0x430005C7, 0x43001589, 0x43003D83, 
        0x43000D8B, 0x4300158B, 0x43043589, 0x430405C7,
        0x4304358B, 0x4304158B, 0x4C4304A1, 0x43040D8B,
        0x43041589, 0x43083D83, 0x4C4308A1, 0x43083589,
        0x430805C7, 0x4C4308A3, 0x43081589, 0x43081589,
        0x4C4304A3
        ];
    let rb2: [u16; 2] = [0x9000, 0x4C];
        
    // rewrite program

    // Evacuate the RoundState stroage location -> to [0x4C4300]
    let rs_ovw_list: Vec<OverWrite> = vec![
        OverWrite {addr: 0x41DBD4, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x41DBD8, byte: AddrSize::Word(rb2[0])},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41DF21, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x41DF25, byte: AddrSize::Word(rb2[0])},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41F9E7, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x41F9EB, byte: AddrSize::Word(rb2[0])},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41FBE1, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x41FBE5, byte: AddrSize::Word(rb2[0])},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41FC8D, byte: AddrSize::Dword(rb1[1])}, OverWrite {addr: 0x41FC91, byte: AddrSize::Word(rb2[1])},    // mov [eax+0xBC30], 0x1 -> mov [0x4C4300], 0x1
        OverWrite {addr: 0x41FD76, byte: AddrSize::Dword(rb1[1])}, OverWrite {addr: 0x41FD7A, byte: AddrSize::Word(rb2[1])},    // mov [ecx+0xBC30], 0x2 -> mov [0x4C4300], 0x2
        OverWrite {addr: 0x41FDF3, byte: AddrSize::Dword(rb1[1])}, OverWrite {addr: 0x41FDF7, byte: AddrSize::Word(rb2[1])},    // mov [ecx+0xBC30], 0x3 -> mov [0x4C4300], 0x3
        OverWrite {addr: 0x41FF01, byte: AddrSize::Dword(rb1[2])}, OverWrite {addr: 0x41FF05, byte: AddrSize::Word(rb2[1])},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        OverWrite {addr: 0x42035E, byte: AddrSize::Dword(rb1[1])}, OverWrite {addr: 0x420362, byte: AddrSize::Word(rb2[1])},    // mov [ecx+0xBC30], 0x4 -> mov [0x4C4300], 0x4
        OverWrite {addr: 0x420399, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x42039D, byte: AddrSize::Word(rb2[0])},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x421B93, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x421B97, byte: AddrSize::Word(rb2[0])},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x423EBE, byte: AddrSize::Dword(rb1[3])}, OverWrite {addr: 0x423EC2, byte: AddrSize::Word(rb2[1])},    // cmp [ecx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        OverWrite {addr: 0x42E1D4, byte: AddrSize::Dword(rb1[2])}, OverWrite {addr: 0x42E1D8, byte: AddrSize::Word(rb2[1])},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        OverWrite {addr: 0x42E8CA, byte: AddrSize::Dword(rb1[4])}, OverWrite {addr: 0x42E8CE, byte: AddrSize::Word(rb2[1])},    // mov ecx, [esi+0xBC30] -> mov ecx, [0x4C4300]
        OverWrite {addr: 0x434A58, byte: AddrSize::Dword(rb1[3])}, OverWrite {addr: 0x434A5C, byte: AddrSize::Word(rb2[1])},    // cmp [eax+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x43A762, byte: AddrSize::Dword(rb1[3])}, OverWrite {addr: 0x43A766, byte: AddrSize::Word(rb2[1])},    // cmp [edx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        OverWrite {addr: 0x440BF7, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x440BFB, byte: AddrSize::Word(rb2[0])},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x440CAB, byte: AddrSize::Dword(rb1[3])}, OverWrite {addr: 0x440CAF, byte: AddrSize::Word(rb2[1])},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x440D95, byte: AddrSize::Dword(rb1[0])}, OverWrite {addr: 0x440D99, byte: AddrSize::Word(rb2[0])},    // mov eax, [eax+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x441274, byte: AddrSize::Dword(rb1[3])}, OverWrite {addr: 0x441278, byte: AddrSize::Word(rb2[1])},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x47BF1D, byte: AddrSize::Dword(rb1[5])}, OverWrite {addr: 0x47BF21, byte: AddrSize::Word(rb2[1])},    // mov edx, [eax+0xBC30] -> mov edx, [0x4C4300]
    ];

    match overwrite_process_list(&rs_ovw_list, process) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    // Evacuate the WinFlag storage location
    let wf_ovw_list: Vec<OverWrite> = vec![
        OverWrite {addr: 0x41F8CE, byte: AddrSize::Dword(rb1[7])}, OverWrite {addr: 0x41F8D2, byte: AddrSize::Word(rb2[1])},
        OverWrite {addr: 0x41F8D6, byte: AddrSize::Word(0x0 as u16)},                                                           // mov [eax + 0xBC34], 0x0 -> mov [0x4C4304], 0x0
        OverWrite {addr: 0x41F8ED, byte: AddrSize::Dword(rb1[6])}, OverWrite {addr: 0x41F8F1, byte: AddrSize::Word(rb2[1])},    // mov [ecx + 0xBC34], esi -> mov [0x4C4304], esi
        OverWrite {addr: 0x41F90D, byte: AddrSize::Dword(rb1[20])}, OverWrite {addr: 0x41F911, byte: AddrSize::Word(rb2[0])},   // mov [edx + 0xBC34], eax -> mov [0x4C4304], eax; nop
        OverWrite {addr: 0x41F9BD, byte: AddrSize::Dword(rb1[7])}, OverWrite {addr: 0x41F9C1, byte: AddrSize::Word(rb2[1])},
        OverWrite {addr: 0x41F9C5, byte: AddrSize::Word(0x0 as u16)},                                                           // mov [edx + 0xBC34], 0x0 -> mov [0x4C4304], 0x0
        OverWrite {addr: 0x4204A8, byte: AddrSize::Dword(rb1[9])}, OverWrite {addr: 0x4204AC, byte: AddrSize::Word(rb2[1])},    // mov edx, [ecx + 0xBC34] -> mov edx, [0x4C4304]
        OverWrite {addr: 0x420518, byte: AddrSize::Dword(rb1[10])}, OverWrite {addr: 0x42051C, byte: AddrSize::Word(rb2[0])},   // mov eax, [ecx + 0xBC34] -> mov eax, [0x4C4304]; nop
        OverWrite {addr: 0x420535, byte: AddrSize::Dword(rb1[11])}, OverWrite {addr: 0x420539, byte: AddrSize::Word(rb2[1])},   // mov ecx, [eax + 0xBC34] -> mov ecx, [0x4C4304]
        OverWrite {addr: 0x42055F, byte: AddrSize::Dword(rb1[8])}, OverWrite {addr: 0x420563, byte: AddrSize::Word(rb2[1])},    // mov esi, [ecx + 0xBC34] -> mov esi, [0x4C4304]
        OverWrite {addr: 0x42DAB7, byte: AddrSize::Dword(rb1[9])}, OverWrite {addr: 0x42DABB, byte: AddrSize::Word(rb2[1])},    // mov edx, [ecx + 0xBC34] -> mov edx, [0x4C4304]
        OverWrite {addr: 0x42E1DF, byte: AddrSize::Dword(rb1[12])}, OverWrite {addr: 0x42E1E3, byte: AddrSize::Word(rb2[1])},   // mov [eax + 0xBC34], edx -> mov [0x4C4304], edx
        OverWrite {addr: 0x42E8B5, byte: AddrSize::Dword(rb1[10])}, OverWrite {addr: 0x42E8B9, byte: AddrSize::Word(rb2[0])},   // mov eax, [eax + 0xBC34] -> mov eax, [0x4C4304]; nop
        OverWrite {addr: 0x42E90D, byte: AddrSize::Dword(rb1[11])}, OverWrite {addr: 0x42E911, byte: AddrSize::Word(rb2[1])},   // mov ecx, [esi + 0xBC34] -> mov ecx, [0x4C4304]
    ];

    match overwrite_process_list(&wf_ovw_list, process) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }


    // Evacuate the EoG storage location
    let eog_ovw_list: Vec<OverWrite> = vec![
        OverWrite {addr: 0x41DD98, byte: AddrSize::Dword(rb1[13])}, OverWrite {addr: 0x41DD9C, byte: AddrSize::Word(rb2[1])},   
        OverWrite {addr: 0x41DD9E, byte: AddrSize::Byte(0x3 as u8)},                                                             // mov [ebp + 0xBC38], 0x3 -> mov [0x4C4308], 0x3
        OverWrite {addr: 0x41DF50, byte: AddrSize::Dword(rb1[14])}, OverWrite {addr: 0x41DF54, byte: AddrSize::Word(rb2[0])},    // mov eax, [ebp + 0xBC38] -> mov eax, [0x4C4308]; nop
        OverWrite {addr: 0x41F8C3, byte: AddrSize::Dword(rb1[15])}, OverWrite {addr: 0x41F8C7, byte: AddrSize::Word(rb2[1])},    // mov [ecx + 0xBC38], esi -> mov [0x4C4308], esi
        OverWrite {addr: 0x41F8DD, byte: AddrSize::Dword(rb1[16])}, OverWrite {addr: 0x41F8E1, byte: AddrSize::Dword(0x1004C as u32)},
        OverWrite {addr: 0x41F8E5, byte: AddrSize::Word(0x0 as u16)},                                                            // mov [ecx + 0xBC38], 0x1 -> mov [0x4C4308], 0x1
        OverWrite {addr: 0x41F901, byte: AddrSize::Dword(rb1[17])}, OverWrite {addr: 0x41F905, byte: AddrSize::Word(rb2[0])},    // mov [ecx + 0xBC38], eax -> mov [0x4C4308], eax; nop
        OverWrite {addr: 0x41F92F, byte: AddrSize::Dword(rb1[18])}, OverWrite {addr: 0x41F933, byte: AddrSize::Word(rb2[1])},    // mov [ecx + 0xBC38], edx -> mov [0x4C4308], edx
        OverWrite {addr: 0x41FECC, byte: AddrSize::Dword(rb1[17])}, OverWrite {addr: 0x41FED0, byte: AddrSize::Word(rb2[0])},    // mov [ecx + 0xBC38], eax -> mov [0x4C4308], eax; nop
        OverWrite {addr: 0x41FF1C, byte: AddrSize::Dword(rb1[19])}, OverWrite {addr: 0x41FF20, byte: AddrSize::Word(rb2[1])},    // mov [eax + 0xBC38], edx -> mov [0x4C4308], edx
        OverWrite {addr: 0x41FF86, byte: AddrSize::Dword(rb1[14])}, OverWrite {addr: 0x41FF8A, byte: AddrSize::Word(rb2[0])},    // mov eax, [ecx + 0xBC38] -> mov eax, [0x4C4308]; nop
        OverWrite {addr: 0x41FFEA, byte: AddrSize::Dword(rb1[13])}, OverWrite {addr: 0x41FFEE, byte: AddrSize::Word(rb2[1])},
        OverWrite {addr: 0x41FFF0, byte: AddrSize::Byte(0x3 as u8)},                                                             // mov [ecx + 0xBC38], 0x3 -> mov [0x4C4308], 0x3
        OverWrite {addr: 0x42E1EB, byte: AddrSize::Dword(rb1[19])}, OverWrite {addr: 0x42E1EF, byte: AddrSize::Word(rb2[1])},    // mov [ecx + 0xBC38], edx -> mov [0x4C4308], edx
    ];

    match overwrite_process_list(&eog_ovw_list, process) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    // Other overwrites
    let other_ovw_list: Vec<OverWrite> = vec![
        OverWrite {addr: 0x44152C, byte: AddrSize::Dword(0x90909090 as u32)},   // -> nop nop nop nop
        OverWrite {addr: 0x441530, byte: AddrSize::Word(0xEB90 as u16)},        // -> nop
        OverWrite {addr: 0x441532, byte: AddrSize::Byte(0x2C as u8)},           // -> jmp 0x44155F
    ];

    match overwrite_process_list(&other_ovw_list, process) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

pub unsafe extern "stdcall" fn overwrite_process_list(ovw_list: &Vec<OverWrite>, process: &Process) -> Result<(), &'static str> {

    let min_addr = ovw_list.iter().map(|ovw| ovw.addr).min().unwrap();
    let max_addr = ovw_list.iter().map(|ovw| ovw.addr).max().unwrap();
    let addr_range = max_addr - min_addr;
    let mut oldp: minwindef::DWORD = 0;

    match Process::check_protection(process, min_addr) {
        Ok(meminfo) => {
            match meminfo.Protect {
                winnt::PAGE_EXECUTE_READWRITE | winnt::PAGE_READWRITE => {
                    if meminfo.RegionSize < addr_range as usize {
                        match Process::change_protection(process, min_addr, winnt::PAGE_READWRITE, addr_range) {
                            Ok(o) => oldp = o,
                            Err(err) => return Err(err)
                        }
                    }
                },
                _ => {
                    match Process::change_protection(process, min_addr, winnt::PAGE_READWRITE, addr_range) {
                        Ok(o) => oldp = o,
                        Err(err) => return Err(err)
                    }
                }
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    for ovw in ovw_list {
        Process::write(process, ovw.addr, ovw.byte);
    }

    if oldp != 0 {
        match Process::change_protection(process, min_addr, oldp, addr_range) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
    }
    
    Ok(())
}