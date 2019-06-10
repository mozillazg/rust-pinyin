开发文档
==========


发布新版本
----------

发布新版本步骤如下：

1. 首先确保 master 分支是最新的准备发布的代码。
2. 确保 master 分支的单元测试和 examples 正常通过（可以通过把 master 代码推送到远程然后触发 CI 服务确认也可以在本地跑测试确认）。
3. 通过 [bumpversion](https://pypi.org/project/bumpversion/) 更新本地版本号以及增加 git tag（三选一）（如果不想使用 bumpversion 命令的话，需要手动编辑 `Cargo.toml` 文件更新版本号以及通过 git tag 命令增加一个新 tag）:
   * 如果是大版本（1.x.y -> 2.0.0）更新，执行 `bumpversion --verbose major` 命令。
   * 如果是不兼容/新功能版本（1.2.y -> 1.3.0），执行 `bumpversion --verbose minor` 命令。
   * 如果是 bugfix 之类的小版本（1.2.3 -> 1.2.4），执行 `bumpversion --verbose patch` 命令。
4. 准备发布新版本:
   * `cargo publish`
5. 检查发布结果是否符合预期：
   * https://crates.io/crates/pinyin
   * https://docs.rs/pinyin/
