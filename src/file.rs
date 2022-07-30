use tokio::{fs::File as FileAsync, io::AsyncReadExt};
pub(crate) struct File {}

impl File {
  pub async fn to_buf(path: &str) -> Vec<u8> {
    let f = FileAsync::open(path).await;
    let mut contents = vec![];
    let _f = match f {
      Ok(mut file) => file.read_to_end(&mut contents).await,
      Err(error) => panic!("Opening File Fail: {:?}", error),
    };
    return contents;
  }
  pub async fn to_string(path: &str) -> String {
    let contents = File::to_buf(path).await;
    match std::str::from_utf8(&contents[..]) {
      Ok(v) => v.to_string(),
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
  }
}
