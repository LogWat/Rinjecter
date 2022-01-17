use crate::processlib::Process;

pub unsafe extern "stdcall" fn overwrite(process: &Process) -> Result<(), &'static str> {

    let rb1: [u32; 20] = [
        0x4BEA00A1, 0xEA0005C7, 0xEA001589, 0xEA003D83, 
        0xEA000D8B, 0xEA00158B, 0xEA043589, 0xEA0405C7,
        0xEA04358B, 0xEA04158B, 0x4BEA04A1, 0xEA040D8B,
        0xEA041589, 0xEA083D83, 0x4BEA08A1, 0xEA083589,
        0xEA0805C7, 0x4BEA08A3, 0xEA081589, 0xEA081589
        ];
    let rb2: [u16; 2] = [0x9000, 0x4B];
        
    // rewrite program
    Process::write(process, 0x41DBD4, rb1[0]).unwrap();     // -> mov eax, [0x4BEA00] ([0x4BEA00] = 0x0)
    Process::write(process, 0x41DBD8, rb2[0]).unwrap();     // -> nop
    Process::write(process, 0x41DF21, rb1[0]).unwrap();
    Process::write(process, 0x41DF25, rb2[0]).unwrap();
    Process::write(process, 0x41F9E7, rb1[0]).unwrap();
    Process::write(process, 0x41F9EB, rb2[0]).unwrap();
    Process::write(process, 0x41FC8D, rb1[1]).unwrap();
    Process::write(process, 0x41FC91, rb2[1]).unwrap();     // -> mov [0x4BEA00], 0x1
    Process::write(process, 0x41DF76, rb1[1]).unwrap();
    Process::write(process, 0x41DF7A, rb2[1]).unwrap();
    Process::write(process, 0x41FDF3, rb1[1]).unwrap();
    Process::write(process, 0x41FDF7, rb2[1]).unwrap();
    Process::write(process, 0x41FF01, rb1[2]).unwrap();
    Process::write(process, 0x41FF05, rb2[1]).unwrap();     // -> mov [0x4BEA00], edx
    Process::write(process, 0x42035E, rb1[2]).unwrap();
    Process::write(process, 0x420362, rb2[1]).unwrap();
    Process::write(process, 0x420399, rb1[0]).unwrap();
    Process::write(process, 0x42039D, rb2[0]).unwrap();
    Process::write(process, 0x421B93, rb1[0]).unwrap();
    Process::write(process, 0x421B97, rb2[0]).unwrap();
    Process::write(process, 0x423EBE, rb1[3]).unwrap();     // -> cmp [0x4BEA00], 0x3
    Process::write(process, 0x423EC2, rb2[1]).unwrap();
    Process::write(process, 0x42E1D4, rb1[2]).unwrap();
    Process::write(process, 0x42E1D8, rb2[1]).unwrap();
    Process::write(process, 0x42E8CA, rb1[4]).unwrap();     // -> mov ecx, [0x4BEA00]
    Process::write(process, 0x42E8CE, rb2[1]).unwrap();
    Process::write(process, 0x434A58, rb1[3]).unwrap();
    Process::write(process, 0x434A5C, rb2[1]).unwrap();
    Process::write(process, 0x43A762, rb1[3]).unwrap();
    Process::write(process, 0x43A766, rb2[1]).unwrap();
    Process::write(process, 0x440BF7, rb1[0]).unwrap();
    Process::write(process, 0x440BFB, rb2[0]).unwrap();
    Process::write(process, 0x440CAB, rb1[3]).unwrap();
    Process::write(process, 0x440CB1, rb2[1]).unwrap();
    Process::write(process, 0x440D95, rb1[0]).unwrap();
    Process::write(process, 0x440D99, rb2[0]).unwrap();
    Process::write(process, 0x441274, rb1[3]).unwrap();
    Process::write(process, 0x441278, rb2[1]).unwrap();
    Process::write(process, 0x47BF1D, rb1[5]).unwrap();    // -> mov edx, [0x4BEA00]
    Process::write(process, 0x47BF21, rb2[1]).unwrap();
    Process::write(process, 0x41F8CE, rb1[7]).unwrap();    // -> mov [0x4BEA00], 0x0
    Process::write(process, 0x41F8D2, rb2[1]).unwrap();
    Process::write(process, 0x41F8D6, 0x0 as u16).unwrap();
    Process::write(process, 0x41F8ED, rb1[6]).unwrap();    // -> mov [0x4BEA00], esi
    Process::write(process, 0x41F8F1, rb2[1]).unwrap();
    Process::write(process, 0x41F90D, rb1[8]).unwrap();    // -> mov [0x4BEA00], eax
    Process::write(process, 0x41F911, rb2[0]).unwrap();    // -> nop
    Process::write(process, 0x41F9BD, rb1[7]).unwrap();
    Process::write(process, 0x41F9C1, rb2[1]).unwrap();
    Process::write(process, 0x41F9C5, 0x0 as u16).unwrap();
    Process::write(process, 0x4204A8, rb1[9]).unwrap();    // -> mov edx, [0x4BEA00]
    Process::write(process, 0x4204AC, rb2[1]).unwrap();
    Process::write(process, 0x420518, rb1[10]).unwrap();   // -> mov eax, [0x4BEA00]
    Process::write(process, 0x420524, rb2[0]).unwrap();    // -> nop
    Process::write(process, 0x420535, rb1[11]).unwrap();   // -> mov ecx, [0x4BEA00]
    Process::write(process, 0x420539, rb2[1]).unwrap();
    Process::write(process, 0x42055F, rb1[8]).unwrap();    // -> mov esi, [0x4BEA00]
    Process::write(process, 0x420563, rb2[1]).unwrap();
    Process::write(process, 0x42DAB7, rb1[9]).unwrap();    // -> mov eax, [0x4BEA00]
    Process::write(process, 0x42DABB, rb2[1]).unwrap();
    Process::write(process, 0x42E1DF, rb1[12]).unwrap();   // -> mov [0x4BEA00], edx
    Process::write(process, 0x42E1E3, rb2[1]).unwrap();
    Process::write(process, 0x42E8B5, rb1[10]).unwrap();
    Process::write(process, 0x42E8B9, rb2[0]).unwrap();
    Process::write(process, 0x42E90D, rb1[11]).unwrap();
    Process::write(process, 0x42E911, rb2[1]).unwrap();
    Process::write(process, 0x41DD98, rb1[13]).unwrap();   // -> cmp [0x4BEA00], 0x3
    Process::write(process, 0x41DD9C, rb2[1]).unwrap();
    Process::write(process, 0x41DD9E, 0x3 as u8).unwrap();
    Process::write(process, 0x41DF50, rb1[14]).unwrap();   // -> mov eax, [0x4BEA00]
    Process::write(process, 0x41DF54, rb2[0]).unwrap();    // -> nop
    Process::write(process, 0x41F8C3, rb1[15]).unwrap();   // -> mov [0x4BEA00], esi
    Process::write(process, 0x41F8C7, rb2[1]).unwrap();
    Process::write(process, 0x41F8DD, rb1[16]).unwrap();   // -> mov [0x4BEA00], 0x1
    Process::write(process, 0x41F8E1, 0x1004B as u32).unwrap();
    Process::write(process, 0x41F8E5, 0x0 as u16).unwrap();
    Process::write(process, 0x41F901, rb1[17]).unwrap();   // -> mov [0x4BEA00], eax
    Process::write(process, 0x41F905, rb2[0]).unwrap();    // -> nop
    Process::write(process, 0x41F92F, rb1[18]).unwrap();   // -> mov [0x4BEA00], edx
    Process::write(process, 0x41F933, rb2[1]).unwrap();
    Process::write(process, 0x41FECC, rb1[17]).unwrap();
    Process::write(process, 0x41FED0, rb2[0]).unwrap();
    Process::write(process, 0x41FF1C, rb1[19]).unwrap();   // -> cmp [0x4BEA00], edx
    Process::write(process, 0x41FF20, rb2[1]).unwrap();
    Process::write(process, 0x41FF86, rb1[14]).unwrap();
    Process::write(process, 0x41FF8A, rb2[0]).unwrap();
    Process::write(process, 0x41FFEA, rb1[13]).unwrap();
    Process::write(process, 0x41FFEE, rb2[1]).unwrap();
    Process::write(process, 0x41FFF0, 0x3 as u8).unwrap();
    Process::write(process, 0x42E1EB, rb1[19]).unwrap();
    Process::write(process, 0x42E1EF, rb2[1]).unwrap();
    Process::write(process, 0x44152C, 0x90909090 as u32).unwrap();     // -> nop nop nop nop
    Process::write(process, 0x44152C + 0x4, 0xEB90 as u16).unwrap();   // -> nop
    Process::write(process, 0x44152C + 0x2, 0x2C as u8).unwrap();     // -> jmp 0x44155F

    Ok(())
}