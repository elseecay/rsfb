#![allow(warnings)]

mod fbapi;
mod detail;

use fbapi as fb;
use fb::ibase as ib;
use detail as dt;

pub struct Error
{
    // TODO
}

type Result<T> = std::result::Result<T, Error>;

pub struct Connection
{

}

pub struct ConnectionParamsBuilder
{

}

pub struct ConnectionParams
{

}

impl Connection
{
    pub fn create_database(filename: &str, page_size: u32)
    {

    }

    pub fn connect()
    {

    }
}


#[cfg(test)]
mod test;