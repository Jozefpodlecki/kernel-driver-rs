param (
    [string]$ProjectName = "kernel-driver",
    [string]$CertName = "WDRLocalTestCert.cer"
)

$location = Get-Location
$certPath = Join-Path $location "$ProjectName\target\debug\$CertName"

# Load the certificate to get its thumbprint
$certToRemove = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($certPath)

function Remove-CertFromStore {
    param(
        [System.Security.Cryptography.X509Certificates.X509Certificate2]$Cert,
        [string]$StoreName
    )

    $store = New-Object System.Security.Cryptography.X509Certificates.X509Store($StoreName, "LocalMachine")
    $store.Open("ReadWrite")

    $existingCert = $store.Certificates | Where-Object { $_.Thumbprint -eq $Cert.Thumbprint }

    if ($existingCert) {
        Write-Host "Removing certificate from $StoreName..."
        $store.Remove($existingCert)
    } else {
        Write-Host "Certificate not found in $StoreName."
    }

    $store.Close()
}

Remove-CertFromStore -Cert $certToRemove -StoreName "Root"

Remove-CertFromStore -Cert $certToRemove -StoreName "TrustedPublisher"

Write-Host "Done."
