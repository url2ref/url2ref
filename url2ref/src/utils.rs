use std::string::FromUtf8Error;

use curl::easy::Easy;

pub fn get_response_as_string(request_url: &str) -> Result<String, FromUtf8Error> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&request_url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    String::from_utf8(data)
}