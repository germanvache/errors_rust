use std::fs::File;
use std::io::ErrorKind;

fn main() {
   let archivo = File::open("hello.txt");

   let _archivo = match archivo {
    Ok(a) =>a,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(a) => a,
            Err(error) => panic!("Problem creating the file: {error:?}"),
        },
        ErrorKind::PermissionDenied => panic!("Permission denied: {error:?}"),
        _ => panic!("Problem opening the file: {error:?}"),
    },
};
}

