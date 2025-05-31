# Git钩子安装脚本
# 配置FurinaOCR项目的Git钩子

param(
    [switch]$Force  # 强制覆盖现有钩子
)

function Write-Success { Write-Host "✅ $args" -ForegroundColor Green }
function Write-Warning { Write-Host "⚠️ $args" -ForegroundColor Yellow }
function Write-Error { Write-Host "❌ $args" -ForegroundColor Red }
function Write-Info { Write-Host "ℹ️ $args" -ForegroundColor Cyan }

Write-Host @"
🎯 FurinaOCR Git钩子安装程序
==============================
"@ -ForegroundColor Cyan

# 检查是否在Git仓库中
if (-not (Test-Path ".git")) {
    Write-Error "当前目录不是Git仓库"
    exit 1
}

# 确保.git/hooks目录存在
$hooksDir = ".git/hooks"
if (-not (Test-Path $hooksDir)) {
    New-Item -ItemType Directory -Path $hooksDir -Force | Out-Null
    Write-Info "创建Git钩子目录: $hooksDir"
}

# 复制预提交钩子
$sourceHook = ".githooks/pre-commit"
$targetHook = "$hooksDir/pre-commit"

if (Test-Path $sourceHook) {
    if ((Test-Path $targetHook) -and (-not $Force)) {
        Write-Warning "预提交钩子已存在"
        Write-Info "使用 -Force 参数强制覆盖"
        exit 1
    }
    
    Copy-Item $sourceHook $targetHook -Force
    Write-Success "已安装预提交钩子"
} else {
    Write-Error "源钩子文件不存在: $sourceHook"
    exit 1
}

# 配置Git钩子路径
try {
    git config core.hooksPath .githooks
    Write-Success "已配置Git钩子路径"
}
catch {
    Write-Warning "配置Git钩子路径失败，使用复制的钩子文件"
}

# 验证安装
if (Test-Path $targetHook) {
    Write-Success "🎉 Git钩子安装成功！"
    Write-Info "现在每次提交前都会自动进行代码质量检查"
    Write-Info ""
    Write-Info "钩子功能:"
    Write-Info "  • 代码格式检查 (rustfmt)"
    Write-Info "  • 静态分析 (clippy)"
    Write-Info "  • 编译检查"
    Write-Info "  • 核心测试"
    Write-Info "  • 敏感信息检测"
    Write-Info "  • 提交消息格式检查"
    Write-Info ""
    Write-Info "测试钩子: git commit --dry-run"
} else {
    Write-Error "钩子安装失败"
    exit 1
} 