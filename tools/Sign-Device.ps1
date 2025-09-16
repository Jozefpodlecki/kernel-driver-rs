# Paths
$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe"
$driverPath = "C:\repos\kernel-driver-rs\kernel-driver\target\debug\kernel_driver.sys"
$certPath = "C:\repos\kernel-driver-rs\kernel-driver\target\debug\WDRLocalTestCert.cer"
$certStore = "WDRTestCertStore"
$certName = "WDRLocalTestCert"
$timestampUrl = "http://timestamp.digicert.com"
$fdAlg = "SHA256"

# /s $certStore `
# /n $certName `
# /f $certPath `

& $signtool sign `
    /v `
    /fd $fdAlg `
    /s $certStore `
    /n $certName `
    /t $timestampUrl `
    $driverPath