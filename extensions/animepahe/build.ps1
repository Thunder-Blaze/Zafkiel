#!/usr/bin/env pwsh
# Build the animepahe extension and package it as animepahe.zext

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ScriptDir = $PSScriptRoot

Push-Location $ScriptDir
try {
    Write-Host "Building WASM extension..."
    wasm-pack build --target web --out-dir pkg

    Write-Host "Packaging animepahe.zext..."
    Copy-Item pkg\animepahe_ext.js      extension.js
    Copy-Item pkg\animepahe_ext_bg.wasm extension.wasm

    Compress-Archive -Force `
        -Path manifest.json, extension.js, extension.wasm `
        -DestinationPath animepahe.zext

    Remove-Item extension.js, extension.wasm

    $size = (Get-Item animepahe.zext).Length
    Write-Host "Done: animepahe.zext ($size bytes)"
} finally {
    Pop-Location
}
