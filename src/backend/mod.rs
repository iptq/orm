#[cfg(feature = "mysql")]
pub mod mysql;

use r2d2::ManageConnection;
use Compiler;

pub trait Backend
where
    Self: Sized,
{
    type Compiler: Compiler;
    type ConnectionManager: ManageConnection;
}
