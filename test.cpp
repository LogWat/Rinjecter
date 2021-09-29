#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <iostream>

const unsigned int d_path = 0x4B5B4C;

VOID error_box(LPCWSTR lptext, LPCWSTR title) {
    MessageBoxW(NULL, lptext, title, MB_OK);
    return;
}

void displayname_change() {
    void *start1 = (void *)0x400000;
    DWORD oldp = 0;
    if (!VirtualProtect(start1, 0xA1000, PAGE_READWRITE, &oldp)) {
        error_box(L"Well, it looks like the memory protection setting failed", L"⚠ ERROR ⚠");
        return;
    }
    *((int *)0x401000) = 0x21212121;
    if (!VirtualProtect(start1, 0xA1000, PAGE_READONLY, &oldp)) {
        error_box(L"Well, it looks like the memory protection setting failed", L"⚠ ERROR ⚠");
        return;
    }
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL,DWORD fdwReason,LPVOID lpvReserved){

    switch (fdwReason) {
        case DLL_PROCESS_ATTACH:
            MessageBoxW(NULL, L":v", L":D", MB_OK);
            displayname_change();
            break;
        case DLL_THREAD_ATTACH:
        case DLL_THREAD_DETACH:
        case DLL_PROCESS_DETACH:
            break;
    }

	return TRUE;
}
