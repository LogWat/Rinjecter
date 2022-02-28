# test_rust

### RustによるDLLインジェクション及びプロセスデバッグの練習・実践用プログラム
- dll_injection.asmをシェルコードとして実行させ，dllを対象のプログラム(a.exeとする)にinject
- 注入したdllでcalc.exeをa.exeの子プロセスとして生成
- calc.exeにリモートスレッド(2.dll)をinject
- 2.dllからa.exeに対してデバッグアタッチ

---
#### ※注 
このプログラムは私環境での実験用です．
使用は**自己責任**でお願いいたします．