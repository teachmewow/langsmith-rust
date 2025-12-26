use crate::models::run::Run;
use crate::error::Result;

/// Validates a Run before sending to LangSmith
pub fn validate_run(run: &Run) -> Result<()> {
    if run.name.is_empty() {
        return Err(crate::error::LangSmithError::Config(
            "Run name cannot be empty".to_string()
        ));
    }

    if !run.inputs.is_object() {
        return Err(crate::error::LangSmithError::Config(
            "Run inputs must be an object".to_string()
        ));
    }

    Ok(())
}

