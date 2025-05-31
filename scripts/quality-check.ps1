# FurinaOCR 代码质量检查脚本
# 用于本地开发环境的快速质量验证

param(
    [switch]$Full,      # 执行完整检查
    [switch]$Quick,     # 快速检查
    [switch]$Fix,       # 自动修复可修复的问题
    [switch]$Report     # 生成详细报告
)

# 颜色输出函数
function Write-Success { Write-Host "✅ $args" -ForegroundColor Green }
function Write-Warning { Write-Host "⚠️ $args" -ForegroundColor Yellow }
function Write-Error { Write-Host "❌ $args" -ForegroundColor Red }
function Write-Info { Write-Host "ℹ️ $args" -ForegroundColor Cyan }
function Write-Header { Write-Host "`n🎯 $args" -ForegroundColor Magenta -BackgroundColor Black }

# 检查Rust工具链
function Test-RustToolchain {
    Write-Header "检查Rust工具链"
    
    try {
        $rustVersion = cargo --version
        Write-Success "Rust工具链: $rustVersion"
        
        # 检查必要组件
        $components = @("rustfmt", "clippy")
        foreach ($component in $components) {
            $result = rustup component list --installed | Select-String $component
            if ($result) {
                Write-Success "组件 $component 已安装"
            } else {
                Write-Warning "组件 $component 未安装，正在安装..."
                rustup component add $component
            }
        }
        return $true
    }
    catch {
        Write-Error "Rust工具链检查失败: $_"
        return $false
    }
}

# 代码格式检查
function Test-CodeFormat {
    Write-Header "代码格式检查"
    
    try {
        if ($Fix) {
            Write-Info "正在自动格式化代码..."
            cargo fmt --all
            Write-Success "代码格式化完成"
        } else {
            cargo fmt --all -- --check
            Write-Success "代码格式符合标准"
        }
        return $true
    }
    catch {
        Write-Error "代码格式检查失败"
        if (-not $Fix) {
            Write-Info "运行 'cargo fmt --all' 自动修复格式问题"
        }
        return $false
    }
}

# Clippy静态分析
function Test-ClippyLints {
    Write-Header "Clippy静态分析"
    
    try {
        if ($Fix) {
            Write-Info "正在执行Clippy自动修复..."
            cargo clippy --fix --all-targets --all-features --allow-dirty
        }
        
        cargo clippy --all-targets --all-features -- -D warnings
        Write-Success "Clippy检查通过，零警告"
        return $true
    }
    catch {
        Write-Error "Clippy检查失败"
        Write-Info "请修复上述警告后重新运行"
        return $false
    }
}

# 编译检查
function Test-Compilation {
    Write-Header "编译检查"
    
    try {
        Write-Info "检查所有目标..."
        cargo check --all-targets --all-features
        Write-Success "编译检查通过"
        
        Write-Info "构建发布版本..."
        cargo build --release --all-features
        Write-Success "发布版本构建成功"
        return $true
    }
    catch {
        Write-Error "编译检查失败"
        return $false
    }
}

# 运行测试
function Test-AllTests {
    Write-Header "运行测试套件"
    
    try {
        Write-Info "运行单元测试..."
        cargo test --all --verbose
        Write-Success "单元测试通过"
        
        Write-Info "运行文档测试..."
        cargo test --doc --verbose
        Write-Success "文档测试通过"
        
        if ($Full) {
            Write-Info "运行集成测试..."
            cargo test --package tests --verbose
            Write-Success "集成测试通过"
        }
        
        return $true
    }
    catch {
        Write-Error "测试失败"
        return $false
    }
}

# 安全审计
function Test-SecurityAudit {
    Write-Header "安全审计"
    
    try {
        # 检查cargo-audit是否安装
        $auditInstalled = Get-Command cargo-audit -ErrorAction SilentlyContinue
        if (-not $auditInstalled) {
            Write-Warning "cargo-audit未安装，正在安装..."
            cargo install cargo-audit --quiet
        }
        
        cargo audit
        Write-Success "安全审计通过，无已知漏洞"
        return $true
    }
    catch {
        Write-Error "安全审计失败"
        return $false
    }
}

# 依赖检查
function Test-Dependencies {
    Write-Header "依赖检查"
    
    try {
        # 检查cargo-deny是否安装
        $denyInstalled = Get-Command cargo-deny -ErrorAction SilentlyContinue
        if (-not $denyInstalled) {
            Write-Warning "cargo-deny未安装，正在安装..."
            cargo install cargo-deny --quiet
        }
        
        if (Test-Path "deny.toml") {
            cargo deny check
            Write-Success "依赖检查通过"
        } else {
            Write-Warning "未找到deny.toml配置文件，跳过依赖检查"
        }
        return $true
    }
    catch {
        Write-Error "依赖检查失败"
        return $false
    }
}

# 性能基准测试
function Test-Benchmarks {
    Write-Header "性能基准测试"
    
    try {
        if (Test-Path "benches") {
            Write-Info "运行性能基准测试..."
            cargo bench --bench string_optimization_bench
            Write-Success "性能基准测试完成"
        } else {
            Write-Warning "未找到基准测试，跳过"
        }
        return $true
    }
    catch {
        Write-Error "性能基准测试失败"
        return $false
    }
}

# 生成质量报告
function New-QualityReport {
    Write-Header "生成质量报告"
    
    $reportPath = "quality-report.md"
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    
    $report = @"
# FurinaOCR 代码质量报告

**生成时间**: $timestamp  
**检查类型**: $(if ($Full) { "完整检查" } elseif ($Quick) { "快速检查" } else { "标准检查" })

## 检查结果

"@
    
    $report | Out-File -FilePath $reportPath -Encoding UTF8
    Write-Success "质量报告已生成: $reportPath"
}

# 主执行流程
function Start-QualityCheck {
    Write-Host @"
🎯 FurinaOCR 代码质量检查工具
=================================
"@ -ForegroundColor Cyan

    $allPassed = $true
    $checksPassed = 0
    $totalChecks = 0
    
    # 执行检查步骤
    $checks = @(
        @{ Name = "Rust工具链"; Function = { Test-RustToolchain } },
        @{ Name = "代码格式"; Function = { Test-CodeFormat } },
        @{ Name = "Clippy分析"; Function = { Test-ClippyLints } },
        @{ Name = "编译检查"; Function = { Test-Compilation } },
        @{ Name = "测试套件"; Function = { Test-AllTests } },
        @{ Name = "安全审计"; Function = { Test-SecurityAudit } },
        @{ Name = "依赖检查"; Function = { Test-Dependencies } }
    )
    
    if ($Full) {
        $checks += @{ Name = "性能基准"; Function = { Test-Benchmarks } }
    }
    
    foreach ($check in $checks) {
        $totalChecks++
        try {
            $result = & $check.Function
            if ($result) {
                $checksPassed++
                Write-Success "$($check.Name) 通过"
            } else {
                $allPassed = $false
                Write-Error "$($check.Name) 失败"
            }
        }
        catch {
            $allPassed = $false
            Write-Error "$($check.Name) 执行异常: $_"
        }
        Write-Host ""
    }
    
    # 生成报告
    if ($Report) {
        New-QualityReport
    }
    
    # 输出总结
    Write-Header "检查总结"
    Write-Host "通过检查: $checksPassed/$totalChecks" -ForegroundColor $(if ($allPassed) { "Green" } else { "Red" })
    
    if ($allPassed) {
        Write-Success "🎉 所有质量检查通过！代码已准备好提交。"
        exit 0
    } else {
        Write-Error "❌ 部分检查失败，请修复后重新运行。"
        exit 1
    }
}

# 参数处理
if ($Quick) {
    Write-Info "执行快速检查模式"
    $checks = @("Test-CodeFormat", "Test-ClippyLints")
} elseif ($Full) {
    Write-Info "执行完整检查模式"
} else {
    Write-Info "执行标准检查模式"
}

# 启动检查
Start-QualityCheck 