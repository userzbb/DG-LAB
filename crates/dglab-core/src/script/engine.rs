//! 脚本引擎错误类型

/// 脚本执行错误
#[derive(Debug, thiserror::Error)]
pub enum ScriptError {
    /// 解析错误
    #[error("Parse error: {0}")]
    ParseError(String),
    /// 运行时错误
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}
