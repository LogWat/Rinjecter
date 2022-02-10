global _start

section .text

_start:
    pusha
    pushf
_strcmp:
.loop:
    mov al, [esi]
    mov ah, [edi]
    cmp al, ah
    jne .end
    inc esi
    inc edi
    cmp al, 0
    jne .loop
.end:
    popf
    popa
    ret