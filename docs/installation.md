# 环境配置

## windows

1. 官网安装rust;
2. 安装idea插件rust;
3. 安装VisualCppBuildTools,缺少会提示找不到link.exe.地址:https://download.microsoft.com/download/5/F/7/5F7ACAEB-8363-451F-9425-68A90F98B238/visualcppbuildtools_full.exe
4. 切换国内源,配置路径为~/.cargo/config,内容如下:


    [source.crates-io]
    registry = "https://github.com/rust-lang/crates.io-index"
    replace-with = 'ustc'
    [source.ustc]
    registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    
    [http]
    timeout = 300                # timeout for each HTTP request, in seconds
    check-revoke = false         # check for SSL certificate revocation
    
 