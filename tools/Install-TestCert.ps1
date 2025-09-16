param (
    [string]$ProjectName = "kernel-driver",
    [string]$CertName = "WDRLocalTestCert.cer"
)

$location   = Get-Location
$certPath   = Join-Path $location "$ProjectName\target\debug\$CertName"

$certToInstall = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($certPath)

function Test-CertInStore {
    param(
        [System.Security.Cryptography.X509Certificates.X509Certificate2]$Cert,
        [string]$StoreLocation
    )

    $store = New-Object System.Security.Cryptography.X509Certificates.X509Store($StoreLocation, "LocalMachine")
    $store.Open("ReadOnly")

    $exists = $store.Certificates | Where-Object { $_.Thumbprint -eq $Cert.Thumbprint }
    $store.Close()

    return $exists -ne $null
}

if (-not (Test-CertInStore -Cert $certToInstall -StoreLocation "Root")) {
    Write-Host "Installing certificate to Trusted Root Certification Authorities..."
    Import-Certificate -FilePath $certPath -CertStoreLocation Cert:\LocalMachine\Root | Out-Null
} else {
    Write-Host "Certificate already exists in Trusted Root Certification Authorities."
}

if (-not (Test-CertInStore -Cert $certToInstall -StoreLocation "TrustedPublisher")) {
    Write-Host "Installing certificate to Trusted Publishers..."
    Import-Certificate -FilePath $certPath -CertStoreLocation Cert:\LocalMachine\TrustedPublisher | Out-Null
} else {
    Write-Host "Certificate already exists in Trusted Publishers."
}

Write-Host "Done."