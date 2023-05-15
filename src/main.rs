use anyhow::{bail, Result};
use nom::number::streaming::u16;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::prelude::FileExt;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut db_header = [0; 100];
            let mut pageHeader  =   [0 ; 8] ;

            file.read_exact(&mut db_header)?;
            file.read_exact_at( &mut pageHeader , 100 ) ? ;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            #[allow(unused_variables)]
            let page_size = u16::from_be_bytes([db_header[16], db_header[17]]);

            // You can use print statements as follows for debugging, they'll be visible when running tests.
            println!("Logs from your program will appear here!");
            
            let pagesQuantity    =   u16::from_be_bytes( [ db_header[ 28 ] , db_header[ 31 ] ] );
            println!( "Size of the database file in pages:\t{pagesQuantity}" );

            // Cells == rows in a table
            let cellsQuantity   =   u16::from_be_bytes( [ pageHeader[ 3 ] , pageHeader[ 4 ] ] );
            println!( "number of tables:\t{}" , cellsQuantity );
            
            println!( "Page header:\t{:X?}" , pageHeader );

            //let firstFreeListTrunkPageNum =   u16::from_be_bytes( [ db_header[ 32 ] , db_header[ 35 ] ] );
            //println!( "Page number of the first freelist trunk page:\t{}" , firstFreeListTrunkPageNum );

            //let freeListPagesQuantity   =   u16::from_be_bytes( [ db_header[ 36 ] , db_header[ 39 ] ] );
            //println!( "Total number of freelist pages:\t{}" , freeListPagesQuantity );
            
            //let largestRootPage_Num   =   u16::from_be_bytes( [ db_header[ 52 ] , db_header[ 55 ] ] );
            //println!( "The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise:\t{}" , largestRootPage_Num );

            // Uncomment this block to pass the first stage
            println!("database page size: {}", page_size);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
