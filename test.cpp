#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <iostream>

const unsigned int d_path = 0x4B5B4C;

VOID error_box(LPCWSTR lptext, LPCWSTR title) {
    MessageBoxW(NULL, lptext, title, MB_OK);
    return;
}

// Obtain the self address
DWORD WINAPI Thread2Proc() {
    int self_addr = 0;
    int tmp = *((int *)d_path) + 0x2DD5;
    int *test = *(int **)d_path;
    while (self_addr == 0) {
        for (int i = 0; i < 4; i++) {
            if (*(int *)(test + 0x2DD5 + i) > 0x4C0000) {
                *(int *)(test + 0x2DD2) = 0x21212121;
                if (**(int **)(test + 0x2DD5 + i) == 0x313D) {
                    *(*(int **)(test + 0x2DD5 + i) + 0x4) = 0x21212121;
                    self_addr = *((int *)(test + 0x2DD5 + i));
                }
            }
        }
    }
    // *((int *)(self_addr + 0x389)) = -210384;
    return 0;
}

void displayname_change() {
    DWORD oldp = 0;
    DWORD oldp2 = 0;
    int *start1 = *((int **)d_path);
    int *start2 = *(*((int ***)d_path) + 0x64B);
    if (!VirtualProtect((void *)start1, sizeof(int) * 3, PAGE_READWRITE, &oldp)) {
        error_box(L"DisplayName_Change() <= Failed!", L"⚠ ERROR ⚠");
        return;
    }
    *(int *)(start1 + 0x4) = 0x7473694D; // Mistaken
    *(int *)(start1 + 0x5) = 0x6E656B61;

    int num_of_char = *(*((int **)d_path) + 0x335); // [d_path]+3284
    int counter = 0;
    for (int *i = start2; counter <= num_of_char; i += 0x10E) {
        if (*((int *)(i + 0x1)) == 0x7473694D && *((int *)(i + 0x2)) == 0x6E656B61) {
            if (!VirtualProtect((void *)i, sizeof(int) * 4, PAGE_READWRITE, &oldp)) {
                error_box(L"DisplayName_Change() <= Failed!", L"⚠ ERROR ⚠");
                return;
            }
            TCHAR UserName[256];
            DWORD dwUserSize = sizeof(UserName)/sizeof(UserName[0]);
            if (!GetUserName(UserName, &dwUserSize)) {
                *((int *)(i + 0x1)) = 0x5E6F5E28;
                *((int *)(i + 0x2)) = 0x2F29;
                return;
            }
            char *addr = (char *)i;
            int j = 0;
            for (j = 0; UserName[j] != '\0'; ++j) {
                *((char *)(addr + 0x4 + j)) = UserName[j];
            }
            *((char *)(addr + 0x4 + j)) = 0x0;
            break;
        }
        counter++;
    }
}

bool change_memprotect() {
    void *start1 = (void *)0x410000;
    DWORD oldp1 = 0;
    //if (!VirtualProtect(start1, 0xC2000, PAGE_READWRITE, &oldp1)) 
    //    return false;
    if (!VirtualProtect(start1, 0xB5000, PAGE_READWRITE, &oldp1)) 
        return false;

    return true;
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL,DWORD fdwReason,LPVOID lpvReserved){

    switch (fdwReason) {
        case DLL_PROCESS_ATTACH:
            displayname_change();
            if (!change_memprotect()) {
                error_box(L"change_memprotect() <= Failed!", L"⚠ ERROR ⚠");
            }
            int param;
            CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)Thread2Proc, (LPVOID)&param, 0, NULL);

            break;
        case DLL_THREAD_ATTACH:
        case DLL_THREAD_DETACH:
        case DLL_PROCESS_DETACH:
            break;
    }

	return TRUE;
}
