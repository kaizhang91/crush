use crate::commands::CompileContext;
use crate::errors::JobResult;
use crate::data::CellType;
use crate::data::Row;
use crate::data::Cell;
use crate::data::ColumnType;
use crate::env::get_cwd;

pub fn compile_and_run(context: CompileContext) -> JobResult<()> {
    context.output.send(Cell::File(get_cwd()?))
}
