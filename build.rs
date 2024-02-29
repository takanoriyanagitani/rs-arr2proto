use std::io;

fn main() -> Result<(), io::Error> {
    prost_build::compile_protos(&["array2d/v1/arr2d.proto"], &["array2d-proto"])?;
    Ok(())
}
