# Rinject

### RustによるDLLインジェクション及びプロセスデバッグの練習・実践用プログラム
- dll_injection.asmをシェルコードとして実行させ，dllを対象のプログラム(a.exeとする)にinject
- 注入したdllでcalc.exeをa.exeの子プロセスとして生成
- calc.exeにリモートスレッド(2.dll)をinject
- 2.dllからa.exeに対してデバッグアタッチ

---
#### ※注 
このプログラムは私環境での実験用です．
使用は**自己責任**でお願いいたします．

#### メモ
- 対象のプログラム(a.exeとする)に対してshellcodeを利用し，dllを注入する(dll1とする)
    - dll1からだとa.exeに対してDebugActiveProcessを呼び出すとAccess denied(GetLastError 5)を吐く
- dll1からcalc.exeを子プロセスとして生成し，dllを注入する(dll2とする)
    - dll2から**a.exe**にDebugActiveProcessを呼び出すとプロセスアタッチに成功する