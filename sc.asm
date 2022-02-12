section .text

global _start

_start:
    pusha
    pushf
    mov esi, -0x4B5B4C      ; [[[0x4B5B4C]+0xCD0]+0x40C+E30*0x0～0x3]
    not esi
    mov esi, [esi]
    sub esi, -0xCD0
    mov esi, [esi]
    mov edx, esi            ; num of chars
    sub esi, -0x40C         ; path of the file
    xor ecx, ecx            ; addr index
    mov edi, -0x4BA010
    not edi
    mov dword [edi], "X|fa"
    mov dword [edi + 0x4], "t~p{"
    mov byte [edi + 0x8], cl
    mov dword [edi + 0x9], "dll"
    mov byte [edi + 0xC], cl
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
    mov edi, -0x4BA010
    not edi
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
    mov edx, -0x4BA020
    not edx
.setpath:
    mov al, [esi]
    test al, al
    jz .dll
    mov byte [edx], al
    inc edx
    inc esi
    jmp .setpath
.dll:
    mov ecx, -0x4BA019
    not ecx
.loop2:
    mov al, [ecx]
    test al, al
    mov byte [edx], al
    jz .calls
    inc edx
    inc ecx
    jmp .loop2
.calls:
    mov ebx, -0x49F604
    not ebx
    push ebx			        ; kernel32の文字列
    mov ebx, -0x49F0B8
    not ebx
    call dword [ebx]	        ; GetModuleHandleA
    test eax, eax
    je _end
    mov ebx, -0x4C40E0
    not ebx
    push ebx 		            ;LoadLibraryAの文字列
    push eax
    mov ebx, -0x49F130
    not ebx
    call ebx		            ; GetProcAddress
    test eax, eax
    je _end
    mov ebx, -0x4BA020
    not ebx
    push ebx                    ; 自分が作成したDLL
    call eax
_end:
    popf
    popa
    sub esp, 0x8
    mov dword [esp], -0x479B7B
    not dword [esp]
    ret