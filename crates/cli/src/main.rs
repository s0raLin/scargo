use scargo::Scargo;

// 从插件 crate 导入插件
// 注意：插件 crate 依赖主项目，但主项目不依赖插件 crate
// 这样可以避免循环依赖
use plugins::jsp_plugin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 函数式Builder模式 - 链式注册插件
    Scargo::new()
        .plugin(jsp_plugin())
        .run()
        .await
}