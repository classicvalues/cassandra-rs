#![feature(collections)]

extern crate collections;
extern crate cql_ffi;

use cql_ffi::*;

const CONTACT_POINTS:&'static str = "127.0.0.1";


#[derive(Debug)]
struct Basic {
    bln:bool,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64,
}

fn insert_into_basic(mut session: CassSession, key:&str, basic:&Basic) -> Result<(CassSession,CassResult),CassError> {
    let query="INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
    let statement = CassStatement::new(query, 6);
    statement.bind_string(0, key).unwrap();
    statement.bind_bool(1, basic.bln).unwrap();
    statement.bind_float(2, basic.flt).unwrap();
    statement.bind_double(3, basic.dbl).unwrap();
    statement.bind_int32(4, basic.i32).unwrap();
    statement.bind_int64(5, basic.i64).unwrap();
    let future = session.execute_statement(&statement).wait().unwrap();
    Ok((session,future))
}

fn select_from_basic(mut session:CassSession, key:&str, basic:&mut Basic) -> Result<(CassSession,CassResult),CassError> {
    let query = "SELECT * FROM examples.basic WHERE key = ?";
    let statement = CassStatement::new(query, 1);
    let statement = statement.bind_string(0, key).unwrap();
    match session.execute_statement(&statement).wait() {
        Ok(result) => {
            for row in result.iter() {
                basic.bln = try!(row.get_column(1).get_bool());
                basic.dbl = try!(row.get_column(2).get_double());
                basic.flt = try!(row.get_column(3).get_float());
                basic.i32 = try!(row.get_column(4).get_int32());
                basic.i64 = try!(row.get_column(5).get_int64());
            }
            Ok((session,result))
        }
        Err(_) => panic!("error")
        
    }
}

fn main() {
    let input = Basic{bln:true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };

    let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let session_future = CassSession::new().connect(&cluster).wait();

    match session_future {
        Ok(mut session) => {
            let mut output = Basic{bln:false,flt:0f32,dbl:0f64,i32:0,i64:0};
            session.execute("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };",0);
            session.execute("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));",0);
            let (session,_) = insert_into_basic(session, "test", &input).unwrap();
            let (mut session,_) = select_from_basic(session, "test", &mut output).unwrap();
            println!("{:?}",input);
            println!("{:?}",output);
            assert!(input.bln == output.bln);
            assert!(input.flt == output.flt);
            assert!(input.dbl == output.dbl);
            assert!(input.i32 == output.i32);
            assert!(input.i64 == output.i64);
            session.close().wait().unwrap();
        },
        _ => {}
    }
}
