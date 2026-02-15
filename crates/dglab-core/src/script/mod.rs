//! 脚本引擎模块（待实现）

pub mod engine;

pub use engine::ScriptError;

/// 脚本引擎（占位符）
pub struct ScriptEngine;

impl ScriptEngine {
    /// 创建新的脚本引擎
    pub fn new() -> Self {
        Self
    }

    /// 执行脚本
    ///
    /// 目前尚未实现，调用时将返回错误。
    pub async fn execute(&self, _script: &str) -> crate::Result<()> {
        Err(crate::error::CoreError::ScriptError(
            "Script engine not implemented yet".to_string(),
        ))
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_returns_error() {
        let engine = ScriptEngine::new();
        let result = engine.execute("some script").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not implemented"));
    }

    #[test]
    fn test_default() {
        let _engine = ScriptEngine::default();
    }
}
