# 打包脚本 - 为多平台构建JiLang并备份

# 获取JiLang版本
$version = (Get-Content -Path "Cargo.toml" | Select-String -Pattern 'version\s*=\s*"(.*?)"').Matches.Groups[1].Value
if (-not $version) {
    $version = "0.3.0"  # 默认版本号
}

# 获取UTC+8时间
$time = [System.TimeZoneInfo]::ConvertTimeBySystemTimeZoneId((Get-Date), "China Standard Time").ToString("yyyy-MM-dd-HH-mm-ss")

# 创建备份目录
$backupDir = "target\WOWBackup\$version-$time"
New-Item -ItemType Directory -Path $backupDir -Force

# 平台配置
$platforms = @(
    @{target="x86_64-pc-windows-msvc"; name="win64"; ext="exe"},
    @{target="i686-pc-windows-msvc"; name="win32"; ext="exe"},
    @{target="x86_64-unknown-linux-gnu"; name="linux-x64"; ext="bin"},
    @{target="x86_64-unknown-linux-musl"; name="linux-musl"; ext="bin"},
    @{target="aarch64-unknown-linux-gnu"; name="linux-arm64"; ext="bin"}
)

# 为每个平台构建
foreach ($platform in $platforms) {
    Write-Host "正在为 $($platform.name) 构建..."
    
    # 确保安装了目标
    cargo install cross 2>$null
    
    # 构建
    if ($platform.target -like "*windows*") {
        cargo build --release --target $platform.target
        $buildPath = "target\$($platform.target)\release\JiLang.$($platform.ext)"
    } else {
        cross build --release --target $platform.target
        $buildPath = "target\$($platform.target)\release\JiLang"
    }
    
    # 检查构建是否成功
    if (Test-Path $buildPath) {
        # 复制到备份目录
        $destName = "JiLang-$($platform.name).$($platform.ext)"
        $destPath = Join-Path $backupDir $destName
        Copy-Item $buildPath $destPath
        Write-Host "已生成: $destPath" -ForegroundColor Green
    } else {
        Write-Host "构建失败: $($platform.name)" -ForegroundColor Red
    }
}

Write-Host "打包完成！文件位于: $backupDir" -ForegroundColor Cyan