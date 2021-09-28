#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>

BOOL WINAPI DllMain(HINSTANCE hinstDLL,DWORD fdwReason,LPVOID lpvReserved){

    switch (fdwReason) {
        case DLL_PROCESS_ATTACH:
            MessageBoxW(NULL, L"Oh, boy. You've made huge mistake.", L"⚠ Warning ⚠", MB_OK);
            break;
        case DLL_THREAD_ATTACH:
        case DLL_THREAD_DETACH:
        case DLL_PROCESS_DETACH:
            break;
    }

	return TRUE;
}