@echo off

%OHOS_NDK_HOME%\native\llvm\bin\clang.exe ^
  -target aarch64-linux-ohos ^
  --sysroot=%OHOS_NDK_HOME%\native\sysroot ^
  -D__MUSL__ ^
  %*
