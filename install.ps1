# Installs Gauntlet on Windows from its GitHub Releases.
#
#   irm https://github.com/BenjaminLucier/gauntlet/releases/latest/download/install.ps1 | iex
#
# Set GAUNTLET_VERSION (such as "0.2.0") for a particular release instead of
# the latest, and GAUNTLET_INSTALL_DIR to install somewhere other than
# %LOCALAPPDATA%\Programs\gauntlet. That folder is added to your user PATH.
# To uninstall, delete the folder and remove it from PATH.
#
# Gauntlet compiles your solutions with Rust, which this doesn't install:
# get it from https://rustup.rs, with the Visual Studio Build Tools it asks for.

& {
    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'  # Invoke-WebRequest is far slower with it
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

    $repo = 'BenjaminLucier/gauntlet'

    # A 32-bit or emulated shell reports its own architecture; the real one is in W6432.
    $arch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
    $target = switch ($arch) {
        'AMD64' { 'x86_64-pc-windows-msvc' }
        'ARM64' { 'aarch64-pc-windows-msvc' }
        default { throw "Gauntlet has no Windows build for the $arch architecture." }
    }

    $base = if ($env:GAUNTLET_VERSION) {
        "https://github.com/$repo/releases/download/v$($env:GAUNTLET_VERSION.TrimStart('v'))"
    } else {
        "https://github.com/$repo/releases/latest/download"
    }
    $dir = if ($env:GAUNTLET_INSTALL_DIR) { $env:GAUNTLET_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'Programs\gauntlet' }

    $archive = "gauntlet-$target.zip"
    $temp = Join-Path ([IO.Path]::GetTempPath()) ([Guid]::NewGuid().ToString())
    New-Item -ItemType Directory -Path $temp | Out-Null
    try {
        Write-Host "Downloading $base/$archive"
        $zip = Join-Path $temp $archive
        Invoke-WebRequest -UseBasicParsing -Uri "$base/$archive" -OutFile $zip
        # Holds "<hash> *gauntlet-<target>.zip".
        $sums = Join-Path $temp "gauntlet-$target.sha256"
        Invoke-WebRequest -UseBasicParsing -Uri "$base/gauntlet-$target.sha256" -OutFile $sums

        $expected = ((Get-Content $sums -Raw).Trim() -split '\s+')[0]
        $actual = (Get-FileHash -Algorithm SHA256 $zip).Hash
        if ($actual -ne $expected) {
            throw "Checksum mismatch for ${archive}: expected $expected, got $actual"
        }

        Expand-Archive -Path $zip -DestinationPath (Join-Path $temp 'x')
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
        Copy-Item -Force (Join-Path $temp 'x\gauntlet.exe') (Join-Path $dir 'gauntlet.exe')
    } finally {
        Remove-Item -Recurse -Force $temp -ErrorAction SilentlyContinue
    }

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $entries = if ($userPath) { $userPath -split ';' | Where-Object { $_ } } else { @() }
    if ($entries -notcontains $dir) {
        [Environment]::SetEnvironmentVariable('Path', (@($entries) + $dir) -join ';', 'User')
        $env:Path = "$env:Path;$dir"
        Write-Host "Added $dir to your PATH. Open a new terminal for other windows to see it."
    }

    $version = & (Join-Path $dir 'gauntlet.exe') --version
    Write-Host "Installed $version to $dir. Run 'gauntlet' to start."
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Host "Gauntlet compiles your solutions with Rust, which isn't on your PATH yet: install it from https://rustup.rs (with the Visual Studio Build Tools it asks for), then run 'rustup component add clippy'."
    }
}
