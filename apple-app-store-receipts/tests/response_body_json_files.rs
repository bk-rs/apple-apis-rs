use std::{error, fs, path::PathBuf};

use apple_app_store_receipts::objects::response_body::ResponseBody;

#[test]
fn de_all() -> Result<(), Box<dyn error::Error>> {
    let dir = PathBuf::new().join("tests/response_body_json_files");
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && Some(Some("json")) == path.extension().map(|x| x.to_str()) {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<ResponseBody>(&content) {
                Ok(response_body) => match response_body {
                    ResponseBody::Success(_) => {
                        assert!(content.contains(r#""status":0"#));

                        println!("path {path:?} de successful");
                    }
                    ResponseBody::Error(body) => {
                        println!("path {path:?} de successful, body: {body:?}");
                    }
                },
                Err(err) => {
                    eprintln!("path {path:?} de failed, err: {err:?}");
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}
