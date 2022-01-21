use crate::processlib::Process;

use winapi::um::{memoryapi, processthreadsapi, winnt};
use winapi::shared::minwindef;

struct OverWrite {
    addr: u32,
    idx0: usize,
    idx1: usize,
}

struct OriginalBytes<T> {
    addr: u32,
    bytes: T,
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
    let rs_ovw_list: [OverWrite; 21] = [
        OverWrite {addr: 0x41DBD4, idx0: 0, idx1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41DF21, idx0: 0, idx1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41F9E7, idx0: 0, idx1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41FBE1, idx0: 0, idx1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x41FC8D, idx0: 1, idx1: 1},    // mov [eax+0xBC30], 0x1 -> mov [0x4C4300], 0x1
        OverWrite {addr: 0x41FD76, idx0: 1, idx1: 1},    // mov [ecx+0xBC30], 0x2 -> mov [0x4C4300], 0x2
        OverWrite {addr: 0x41FDF3, idx0: 1, idx1: 1},    // mov [ecx+0xBC30], 0x3 -> mov [0x4C4300], 0x3 
        OverWrite {addr: 0x41FF01, idx0: 2, idx1: 1},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        OverWrite {addr: 0x42035E, idx0: 1, idx1: 1},    // mov [ecx+0xBC30], 0x4 -> mov [0x4C4300], 0x4
        OverWrite {addr: 0x420399, idx0: 0, idx1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]
        OverWrite {addr: 0x421B93, idx0: 0, idx1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x423EBE, idx0: 3, idx1: 1},    // cmp [ecx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        OverWrite {addr: 0x42E1D4, idx0: 2, idx1: 1},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        OverWrite {addr: 0x42E8CA, idx0: 4, idx1: 1},    // mov ecx, [esi+0xBC30] -> mov ecx, [0x4C4300]
        OverWrite {addr: 0x434A58, idx0: 3, idx1: 1},    // cmp [eax+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x43A762, idx0: 3, idx1: 1},    // cmp [edx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        OverWrite {addr: 0x440BF7, idx0: 0, idx1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x440CAB, idx0: 3, idx1: 1},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x440D95, idx0: 0, idx1: 0},    // mov eax, [eax+0xBC30] -> mov eax, [0x4C4300]; nop
        OverWrite {addr: 0x441274, idx0: 3, idx1: 1},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        OverWrite {addr: 0x47BF1D, idx0: 5, idx1: 1},    // mov edx, [eax+0xBC30] -> mov edx, [0x4C4300]
    ];
    let mut old_bytes: Vec<OverWrite> = Vec::new();

    for i in rs_ovw_list.iter().enumerate() {
        let ovw = &rs_ovw_list[i.0];
        match Process::write(process, ovw.addr, rb1[ovw.idx0]) {
            Ok(_) => {
                old_bytes.push(
                    OverWrite {
                        addr: ovw.addr, 
                        idx0: ovw.idx0, 
                        idx1: ovw.idx1
                    }
                );
            },
            Err(_) => {
                for j in old_bytes.iter().enumerate() {
                    let ovw = &old_bytes[j.0];
                    match Process::write(process, ovw.addr, rb2[ovw.idx1]) {
                        Ok(_) => {},
                        Err(_) => {
                            return Err(format!("Failed to write to {:x}", ovw.addr));
                        }
                    }
                }
                return Err("Error: Failed to OverWrite RoundState section.");
            }
        };
        match Process::write(process, ovw.addr + 4, rb2[ovw.idx1]) {
            Ok(_) => {
                old_bytes.push(
                    OverWrite {
                        addr: ovw.addr, 
                        idx0: ovw.idx0, 
                        idx1: ovw.idx1
                    }
                );
            },
            Err(_) => {
                for j in old_bytes.iter().enumerate() {
                    let ovw = &old_bytes[j.0];
                    match Process::write(process, ovw.addr, rb1[ovw.idx0]) {
                        Ok(_) => {},
                        Err(_) => {
                            return Err(format!("Failed to write to {:x}", ovw.addr));
                        }
                    }
                }
                return Err("Error: Failed to OverWrite RoundState section.");
            }
        };
    };

    // Evacuate the WinFlag storage location
    Process::write(process, 0x41F8CE, rb1[7]).unwrap();    // mov [eax + 0xBC34], 0x0 -> mov [0x4C4304], 0x0
    Process::write(process, 0x41F8D2, rb2[1]).unwrap();
    Process::write(process, 0x41F8D6, 0x0 as u16).unwrap();
    Process::write(process, 0x41F8ED, rb1[6]).unwrap();    // mov [ecx + 0xBC34], esi -> mov [0x4C4304], esi
    Process::write(process, 0x41F8F1, rb2[1]).unwrap();
    Process::write(process, 0x41F90D, rb1[20]).unwrap();   // mov [edx + 0xBC34], eax -> mov [0x4C4304], eax; nop
    Process::write(process, 0x41F911, rb2[0]).unwrap();
    Process::write(process, 0x41F9BD, rb1[7]).unwrap();    // mov [edx + 0xBC34], 0x0 -> mov [0x4C4304], 0x0
    Process::write(process, 0x41F9C1, rb2[1]).unwrap();
    Process::write(process, 0x41F9C5, 0x0 as u16).unwrap();
    Process::write(process, 0x4204A8, rb1[9]).unwrap();    // mov edx, [ecx + 0xBC34] -> mov edx, [0x4C4304]
    Process::write(process, 0x4204AC, rb2[1]).unwrap();
    Process::write(process, 0x420518, rb1[10]).unwrap();   // mov eax, [ecx + 0xBC34] -> mov eax, [0x4C4304]; nop
    Process::write(process, 0x42051C, rb2[0]).unwrap();
    Process::write(process, 0x420535, rb1[11]).unwrap();   // mov ecx, [eax + 0xBC34] -> mov ecx, [0x4C4304]
    Process::write(process, 0x420539, rb2[1]).unwrap();
    Process::write(process, 0x42055F, rb1[8]).unwrap();    // mov esi, [ecx + 0xBC34] -> mov esi, [0x4C4304]
    Process::write(process, 0x420563, rb2[1]).unwrap();
    Process::write(process, 0x42DAB7, rb1[9]).unwrap();    // mov edx, [ecx + 0xBC34] -> mov edx, [0x4C4304]
    Process::write(process, 0x42DABB, rb2[1]).unwrap();
    Process::write(process, 0x42E1DF, rb1[12]).unwrap();   // mov [eax + 0xBC34], edx -> mov [0x4C4304], edx
    Process::write(process, 0x42E1E3, rb2[1]).unwrap();
    Process::write(process, 0x42E8B5, rb1[10]).unwrap();   // mov eax, [eax + 0xBC34] -> mov eax, [0x4C4304]; nop
    Process::write(process, 0x42E8B9, rb2[0]).unwrap();
    Process::write(process, 0x42E90D, rb1[11]).unwrap();   // mov ecx, [esi + 0xBC34] -> mov ecx, [0x4BEA04]
    Process::write(process, 0x42E911, rb2[1]).unwrap();

    // Evacuate the EoG storage location
    Process::write(process, 0x41DD98, rb1[13]).unwrap();   // cmp [ebp + 0xBC38], 0x3 -> cmp [0x4C4308], 0x3
    Process::write(process, 0x41DD9C, rb2[1]).unwrap();
    Process::write(process, 0x41DD9E, 0x3 as u8).unwrap();
    Process::write(process, 0x41DF50, rb1[14]).unwrap();   // mov eax, [ebp + 0xBC38] -> mov eax, [0x4C4308]; nop
    Process::write(process, 0x41DF54, rb2[0]).unwrap();
    Process::write(process, 0x41F8C3, rb1[15]).unwrap();   // mov [ecx + 0xBC38], esi -> mov [0x4C4308], esi
    Process::write(process, 0x41F8C7, rb2[1]).unwrap();
    Process::write(process, 0x41F8DD, rb1[16]).unwrap();   // mov [ecx + 0xBC38], 0x1 -> mov [0x4C4308], 0x1
    Process::write(process, 0x41F8E1, 0x1004C as u32).unwrap();
    Process::write(process, 0x41F8E5, 0x0 as u16).unwrap();
    Process::write(process, 0x41F901, rb1[17]).unwrap();   // mov [ecx + 0xBC38], eax -> mov [0x4C4308], eax; nop
    Process::write(process, 0x41F905, rb2[0]).unwrap();
    Process::write(process, 0x41F92F, rb1[18]).unwrap();   // mov [ecx + 0xBC38], edx -> mov [0x4C4308], edx
    Process::write(process, 0x41F933, rb2[1]).unwrap();
    Process::write(process, 0x41FECC, rb1[17]).unwrap();   // mov [ecx + 0xBC38], eax -> mov [0x4C4308], eax; nop
    Process::write(process, 0x41FED0, rb2[0]).unwrap();
    Process::write(process, 0x41FF1C, rb1[19]).unwrap();   // mov [eax + 0xBC38], edx -> mov [0x4C4308], edx
    Process::write(process, 0x41FF20, rb2[1]).unwrap();
    Process::write(process, 0x41FF86, rb1[14]).unwrap();   // mov eax, [ecx + 0xBC38] -> mov eax, [0x4C4308]
    Process::write(process, 0x41FF8A, rb2[0]).unwrap();
    Process::write(process, 0x41FFEA, rb1[13]).unwrap();   // mov [ecx + 0xBC38], 0x3 -> mov [0x4C4308], 0x3
    Process::write(process, 0x41FFEE, rb2[1]).unwrap();
    Process::write(process, 0x41FFF0, 0x3 as u8).unwrap();
    Process::write(process, 0x42E1EB, rb1[19]).unwrap();   // mov [ecx + 0xBC38], edx -> mov [0x4BEA08], edx
    Process::write(process, 0x42E1EF, rb2[1]).unwrap();

    
    Process::write(process, 0x44152C, 0x90909090 as u32).unwrap();   // -> nop nop nop nop
    Process::write(process, 0x441530, 0xEB90 as u16).unwrap();       // -> nop
    Process::write(process, 0x441532, 0x2C as u8).unwrap();          // -> jmp 0x44155F

    Ok(())
}

unsafe extern "stdcall" fn overwrite_process(ovw_list: &[OverWrite], process: &Process) -> Result<(), &'static str> {

    let rb1: [u32; 21] = [
        0x4C4300A1, 0x430005C7, 0x43001589, 0x43003D83, 
        0x43000D8B, 0x4300158B, 0x43043589, 0x430405C7,
        0x4304358B, 0x4304158B, 0x4C4304A1, 0x43040D8B,
        0x43041589, 0x43083D83, 0x4C4308A1, 0x43083589,
        0x430805C7, 0x4C4308A3, 0x43081589, 0x43081589,
        0x4C4304A3
        ];
    let rb2: [u16; 2] = [0x9000, 0x4C];

    let min_addr = ovw_list.iter().map(|ovw| ovw.addr).min().unwrap();
    let max_addr = ovw_list.iter().map(|ovw| ovw.addr).max().unwrap();
    let addr_range = max_addr - min_addr;
    let mut oldp: minwindef::DWORD = 0;

    match Process::check_protection(process, min_addr) {
        Ok(meminfo) => {
            match meminfo.Protect {
                winnt::PAGE_EXECUTE_READWRITE | winnt::PAGE_READWRITE => {},
                _ => {
                    match Process::change_protection(process, min_addr, winnt::PAGE_EXECUTE_READWRITE | winnt::PAGE_READWRITE, addr_range) {
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
        Process::write(process, ovw.addr, rb1[ovw.idx0]);
        Process::write(process, ovw.addr + 0x4, rb2[ovw.idx1]);
    }

    if oldp != 0 {
        match Process::change_protection(process, min_addr, oldp, addr_range) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
    }
    
    Ok(())
}