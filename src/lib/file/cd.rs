use crate::lib::ExecutionContext;
use crate::errors::{to_job_error, CrushResult, error};
use std::path::Path;
use crate::data::Value;
use crate::scope::{home, cwd};

pub fn perform(context: ExecutionContext) -> CrushResult<()> {
    let dir = match context.arguments.len() {
        0 => home(),
        1 => {
            let dir = &context.arguments[0];
            match &dir.value {
                Value::Text(val) => Ok(Box::from(Path::new(val.as_ref()))),
                Value::File(val) => Ok(val.clone()),
                Value::Glob(val) => val.glob_to_single_file(&cwd()?),
                _ => error(format!("Wrong parameter type, expected text or file, found {}", &dir.value.value_type().to_string()).as_str())
            }
        }
        _ => error("Wrong number of arguments")
    }?;
    to_job_error(std::env::set_current_dir(dir))
}
