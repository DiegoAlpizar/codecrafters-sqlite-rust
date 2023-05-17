use anyhow::{bail, Result};
use nom::number::streaming::u16;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
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
            let mut page_header  =   [0 ; 8] ;

            file.read_exact(&mut db_header)?;
            file.read_exact_at( &mut page_header , 100 ) ? ;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            #[allow(unused_variables)]
            let page_size = u16::from_be_bytes([db_header[16], db_header[17]]);

            // You can use print statements as follows for debugging, they'll be visible when running tests.
            println!("Logs from your program will appear here!");
            
            let pages_quantity    =   u16::from_be_bytes( [ db_header[ 28 ] , db_header[ 31 ] ] );
            println!( "Size of the database file in pages:\t{pages_quantity}" );

            // Cells == rows in a table
            let cells_quantity   =   u16::from_be_bytes( [ page_header[ 3 ] , page_header[ 4 ] ] );

            let cell_content_area_start_offset  =   u16::from_be_bytes( [ page_header[ 5 ] , page_header[ 6 ] ] );

            let mut cell_ptr_arr  =   vec![ 0 ; cells_quantity as usize * 2 ] ;

            file.read_exact_at( &mut cell_ptr_arr , 108 ) ? ;

            let mut file_buf    =   BufReader::new( &file );

            for i in ( 0 .. cell_ptr_arr.len() ).step_by( 2 )
            {
                //let mut cell_ptr_iter    =   cell_ptr_arr.chunks_exact( 2 );
    
                //let next_cell_ptr   =   cell_ptr_iter.next().unwrap() ;
                let next_cell    =  u16::from_be_bytes( [ cell_ptr_arr[ i ] , cell_ptr_arr[ i+1 ] ] ) as u64 ;
                
                file_buf.seek( SeekFrom::Start( next_cell ) ) ? ;
                
                //let algo    =   file_buf..bytes().next().unwrap() ;
                let algo    =   file_buf.read(buf)

                let mut cell_header   =   [ 0 , 2 ] ;
                file.read_at( &mut cell_header , next_cell ) ? ;
    
                let cell_size   =   cell_header[ 0 ] ;
                let row_id  =   cell_header[ 1 ] ;
    
    
                let mut record  =   vec![ 0 ; cell_size as usize ] ;
    
                file.read_exact_at( &mut record , next_cell + 2 ) ? ;
    
                let record_header_size  =   record[ 0 ] ;

                //record_header_size.leading_ones()
    
                let record_header   =   &record[ .. record_header_size as usize ] ;
                
                let serial_type_1   =   record[ 1 ] ;
                let serial_type_2   =   record[ 2 ] ;
                let serial_type_3   =   record[ 3 ] ;
                let serial_type_4   =   record[ 4 ] ;
                let serial_type_5   =   record[ 5 ] ;
    
                let column_1_length    =   { (serial_type_1 - 13) / 2 } as usize ;
                let column_2_length    =   { (serial_type_2 - 13) / 2 } as usize ;
                let column_3_lenght    =   { (serial_type_3 - 13) / 2 } as usize ;
                let column_4_length =   serial_type_4 as usize ;
                let column_5_length    =   { (serial_type_5 - 13) / 2 } as usize ;
    
                let column_3_start_offset   =   column_1_length + column_2_length ;
                let column_4_start_offset   =   column_3_start_offset + column_3_lenght ;
                let column_5_start_offset   =   column_4_start_offset + column_4_length ;
                
                let record_data =   &record[ record_header_size as usize .. ] ;
                
                let db_object_type  =   &record_data[ .. column_1_length ] ;
                let db_object_name  =   &record_data[ column_1_length .. column_3_start_offset ] ;
                let table_name  =   &record_data[ column_3_start_offset .. column_4_start_offset ] ;
                let root_page   =   &record_data[ column_4_start_offset .. column_5_start_offset ] ;
                let sql_statement   =   &record_data[ column_5_start_offset .. ] ;
                                                                
                //file.
                //let mut first_page  =   file.take( 4096 );
                //first_page.
    
                println!( "number of tables:\t{}" , cells_quantity );
    
                println!( "Page header:\t{:X?}" , page_header );
                println!( "Start-offset of cell content area:\t{:X?}" , cell_content_area_start_offset );
    
                println!( "Cell ptr array:\t{:X? }" , cell_ptr_arr );
    
    
                //println!( "{:X?}" , cell_ptr_iter );
                println!( "cpa:\t{}" , u16::from_be_bytes( [ cell_ptr_arr[ 0 ] , cell_ptr_arr[ 1 ] ] ) );
                println!( "next cell:\t{:?}" , next_cell );
                println!( "Cell header:\t{:X?}" , cell_header );
                println!( "Cell size:\t{:X?}\t{}" , cell_size , cell_size );
                println!( "Row ID:\t{}" , row_id );
                println!( "Record header size:\t{}" , record_header_size );
    
                use to_binary::BinaryString ;
    
                println!( "Record header:\t{}" , BinaryString::from( record_header ).add_spaces().unwrap() );
    
                println!( "1st Serial type:\t{}\t{}" , serial_type_1 , column_1_length );
                println!( "2nd Serial type:\t{}\t{}" , serial_type_2 , column_2_length );
                println!( "3rd Serial type:\t{}\t{}" , serial_type_3 , column_3_lenght );
                println!( "4th Serial type:\t{}\t{}" , serial_type_4 , serial_type_4 );
                println!( "5th Serial type:\t{}\t{}" , serial_type_5 , column_5_length );
    
                println!( "Record data:\t{}" , String::from_utf8( record_data.to_vec() ).unwrap() );
    
                println!( "DB object Type:\t{}" , String::from_utf8( db_object_type.to_vec() ).unwrap() );
                println!( "DB object Name:\t{}" , String::from_utf8( db_object_name.to_vec() ).unwrap() );
                println!( "Table Name:\t{}" , String::from_utf8( table_name.to_vec() ).unwrap() );
                println!( "Root page:\t{:?}" , root_page );
                println!( "SQL statement:\t{}" , String::from_utf8( sql_statement.to_vec() ).unwrap() );

                println!( "Blah:\t{:b}\t{:08b}\t{}" , 0b_11000010_u8 , serial_type_5 >> 2 , 0b_01100010_u8.leading_zeros() );
                println!( "Algo:\t{}" , algo.unwrap() );
            }



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
