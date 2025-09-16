param (
    [string]$ProjectName = "kernel-driver",
    [string]$DriverName = "kernel_driver.inf"
)

$Location   = Get-Location
$DriverPath   = Join-Path $location "$ProjectName\target\debug\$DriverName"

# pnputil.exe /add-driver $DriverPath /install
$infPath = "C:\repos\kernel-driver-rs\kernel-driver\target\debug\kernel_driver_package\kernel_driver.inf"
# Start-Process pnputil.exe -ArgumentList "/add-driver $infPath /install"
pnputil.exe /add-driver $infPath /install