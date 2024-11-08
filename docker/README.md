下面是一个Dockerfile，它将初始化一个Rust环境，并从指定的GitHub仓库下载并编译Rust代码。

这个Dockerfile基于官方Rust镜像，安装了Git工具，然后克隆了您指定的GitHub仓库，并使用`cargo build --release`命令编译项目。编译完成后，默认运行编译后的二进制文件。 

您可以使用以下命令来构建和运行Docker镜像：
```sh
docker build -t rustscan_env .
docker run rustscan_env
```

如果需要进一步调整或者添加其他功能，请告诉我。