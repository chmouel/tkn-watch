use std::{
    env,
    fs::File,
    io::{Read, Write},
};

mod testenv;

golden!(finished, [""]);
golden!(stuck, [""]);
golden!(running, [""]);
golden!(failed, [""]);
golden!(customrefresh, ["--refresh-seconds", "2000"]);
