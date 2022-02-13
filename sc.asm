section .text

global _start

_start:
    pusha
    pushf
    mov esi, 0x4B5B4C      ; [[[0x4B5B4C]+0xCD0]+0x40C+E30*0x0～0x3]
    mov esi, [esi]
    sub esi, -0xCD4
    mov edx, [esi]          ; num of chars
    mov esi, [esi-0x4]        ; pointer to chars
    sub esi, -0x40C         ; path of the file
    xor ecx, ecx            ; addr index
    mov edi, 0x4BA010
    mov dword [edi], "lHR"
    mov dword [edi + 0x4], "U@JD"
    mov word [edi + 0x8], "O"
    mov ebx , -0x9
    not ebx
    inc ebx
    inc ebx
    mov byte [edi + ebx], cl
    mov dword [edi + 0xB], ".dll"
    mov byte [edi + 0xF], cl
    xor ebx, ebx
_strcontain:
.loop:
    test edx, edx
    jz .endtoend                 ; can't find the string
    mov al, [esi+ecx]
    mov bl, [edi]
    test al, al
    jz .changechara
    xor bl, 0x21
    cmp al, bl
    je .equal
    inc ecx
    mov edi, 0x4BA010
    jmp .loop
.endtoend:
    jmp _end
.changechara:
    test bl, bl
    jz _setdllpath
    sub esi, -0xE30
    xor ecx, ecx
    dec edx
    jmp .loop
.equal:
    inc ecx
    inc edi
    jmp .loop
_setdllpath:
    mov edx, 0x4BA020
.setpath:
    mov al, [esi]
    test al, al
    jz .dll
    mov byte [edx], al
    inc edx
    inc esi
    jmp .setpath
.dll:
    mov ecx, 0x4BA01B
.loop2:
    mov al, [ecx]
    test al, al
    mov byte [edx], al
    jz .calls
    inc edx
    inc ecx
    jmp .loop2
.calls:
    push dword 0x49F604			; kernel32の文字列
    call dword [0x49F0B8]	    ; GetModuleHandleA
    test eax, eax
    je _end
    push dword 0x4C40E0 		;LoadLibraryAの文字列
    push eax
    call dword [0x49F130]		; GetProcAddress
    test eax, eax
    je _end
    push dword 0x4BA020         ; 自分が作成したDLL
    call eax
_end:
    popf
    popa
    sub esp, 0x8
    mov dword [esp], 0x479B7B
    ret