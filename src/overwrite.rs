use crate::processlib::Process;

struct Overwrite {
    address: u32,
    value_index0: usize,
    value_index1: usize,
}

pub unsafe extern "stdcall" fn overwrite(process: &Process) -> Result<(), &'static str> {

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
    let rs_ovw_list: [Overwrite; 21] = [
        Overwrite {address: 0x41DBD4, value_index0: 0, value_index1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x41DF21, value_index0: 0, value_index1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x41F9E7, value_index0: 0, value_index1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x41FBE1, value_index0: 0, value_index1: 0},    // mov eax, [ebp+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x41FC8D, value_index0: 1, value_index1: 1},    // mov [eax+0xBC30], 0x1 -> mov [0x4C4300], 0x1
        Overwrite {address: 0x41FD76, value_index0: 1, value_index1: 1},    // mov [ecx+0xBC30], 0x2 -> mov [0x4C4300], 0x2
        Overwrite {address: 0x41FDF3, value_index0: 1, value_index1: 1},    // mov [ecx+0xBC30], 0x3 -> mov [0x4C4300], 0x3 
        Overwrite {address: 0x41FF01, value_index0: 2, value_index1: 1},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        Overwrite {address: 0x42035E, value_index0: 1, value_index1: 1},    // mov [ecx+0xBC30], 0x4 -> mov [0x4C4300], 0x4
        Overwrite {address: 0x420399, value_index0: 0, value_index1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]
        Overwrite {address: 0x421B93, value_index0: 0, value_index1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x423EBE, value_index0: 3, value_index1: 1},    // cmp [ecx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        Overwrite {address: 0x42E1D4, value_index0: 2, value_index1: 1},    // mov [ecx+0xBC30], edx -> mov [0x4C4300], edx
        Overwrite {address: 0x42E8CA, value_index0: 4, value_index1: 1},    // mov ecx, [esi+0xBC30] -> mov ecx, [0x4C4300]
        Overwrite {address: 0x434A58, value_index0: 3, value_index1: 1},    // cmp [eax+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        Overwrite {address: 0x43A762, value_index0: 3, value_index1: 1},    // cmp [edx+0xBC30], 0x3 -> cmp [0x4C4300], 0x3
        Overwrite {address: 0x440BF7, value_index0: 0, value_index1: 0},    // mov eax, [ecx+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x440CAB, value_index0: 3, value_index1: 1},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        Overwrite {address: 0x440D95, value_index0: 0, value_index1: 0},    // mov eax, [eax+0xBC30] -> mov eax, [0x4C4300]; nop
        Overwrite {address: 0x441274, value_index0: 3, value_index1: 1},    // cmp [ecx+0xBC30], 0x2 -> cmp [0x4C4300], 0x2
        Overwrite {address: 0x47BF1D, value_index0: 5, value_index1: 1},    // mov edx, [eax+0xBC30] -> mov edx, [0x4C4300]
    ];

    for i in rs_ovw_list.iter().enumerate() {
        let ovw = &rs_ovw_list[i.0];
        match Process::write(process, ovw.address, rb1[ovw.value_index0]) {
            Ok(_) => {},
            Err(_) => {
                return Err("[!] Failed to overwrite RoundState section.");
            }
        };
        match Process::write(process, ovw.address + 4, rb2[ovw.value_index1]) {
            Ok(_) => {},
            Err(_) => {
                return Err("[!] Failed to overwrite RoundState section.");
            }
        };
    }

    Process::write(process, 0x47BF1D, rb1[5]).unwrap();     // mov edx, [eax + 0xBC30] -> mov edx, [0x4BEA00]
    Process::write(process, 0x47BF21, rb2[1]).unwrap();

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