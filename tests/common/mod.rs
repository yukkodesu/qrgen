use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn unique_tmp_image_path(prefix: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("qrgen-{prefix}-{nanos}.{extension}"))
}
