pub mod stdimport;
pub mod types;
pub mod share;

use super::fbapi::*;



pub fn create_status_wrapper() -> StatusWrapper
{
    let m = Master::get();
    let s = m.get_status();
    return StatusWrapper::new(s);
}