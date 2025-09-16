param (
    [string]$ProjectName = "kernel-driver",
    [string]$DriverName = "kernel_driver.inf"
)

$Location   = Get-Location
$DriverPath   = Join-Path $location "$ProjectName\target\debug\$DriverName"

pnputil.exe /add-driver $DriverPath /install