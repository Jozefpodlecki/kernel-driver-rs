# & "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe"
& $signtool verify /kp /v C:\repos\kernel-driver-rs\kernel-driver\target\debug\kernel_driver.sys