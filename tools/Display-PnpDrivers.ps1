pnputil /enum-drivers |
    Select-String "Published Name|Driver Package Provider|Class Name" |
    ForEach-Object {
        if ($_ -match "Published Name") {
            $name = ($_ -split ":\s+")[1].Trim()
        }
        elseif ($_ -match "Driver Package Provider") {
            $provider = ($_ -split ":\s+")[1].Trim()
        }
        elseif ($_ -match "Class Name") {
            $class = ($_ -split ":\s+")[1].Trim()
            [PSCustomObject]@{
                PublishedName = $name
                Provider      = $provider
                Class         = $class
            }
        }
    } |
    Sort-Object PublishedName |
    Format-Table -AutoSize

# Get-WmiObject Win32_PnPSignedDriver |
# Select-Object DeviceName, InfName, DriverVersion, DriverDate, InstallDate |
# Format-Table -AutoSize