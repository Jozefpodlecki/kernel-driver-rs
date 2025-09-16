$catalogPath = "C:\repos\kernel-driver-rs\kernel-driver\target\debug\kernel_driver_package\kernel_driver.cat"
$infPath = "C:\repos\kernel-driver-rs\kernel-driver\target\debug\kernel_driver_package\kernel_driver.inf"
$certPath = "Cert:\LocalMachine\TrustedPublisher"

$signature = Get-AuthenticodeSignature $catalogPath

$store = Get-Item -Path $certPath

$store.Open("ReadWrite")

$store.Add($signature.SignerCertificate)

$store.Close()

Start-Process pnputil.exe -ArgumentList "/add-driver $infPath /install"