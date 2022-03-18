#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate easy_http_request;

use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::value::Value;
use anyhow::{Error, Result};
use futures::future::join_all;
use futures::StreamExt;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::IntoUrl;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::fs::{File, remove_file};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::fs;
use std::io::{copy, Read, Seek, Write};
use std::str;
use encoding::{DecoderTrap, Encoding};
use encoding::all::GBK;


use easy_http_request::{DefaultHttpRequest};
#[napi(object)]
pub struct Pet {
  pub body: String,
  pub status_code: u32,
  pub headers:HashMap<String, String>
}



#[napi]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}
#[napi]
fn wtapi(url: String) -> Pet {
  let response = DefaultHttpRequest::get_from_url_str(url).unwrap().send().unwrap();
  println!("{:?}", response.headers);
  // println!("{}", String::from_utf8(response.body).unwrap());
  let body = String::from_utf8(response.body).unwrap();
  let status_code = response.status_code;
  Pet{
    body,
    status_code:status_code as u32,
    headers:response.headers
  }
}


async fn get(url:&str) -> Result<HashMap<String, String>, reqwest::Error>{
  Ok(reqwest::get(url).await?.json::<HashMap<String, String>>().await?)
}

// 输出 text 格式
async fn get_calss_list_out_text(url:&str)->Result<(HashMap<String, String>),reqwest::Error>{
  let res=reqwest::get(url).await?;
  let s = res.status().is_success();
  let mut a2221: HashMap<String, String> = HashMap::new();
  let status_code:String = "status_code".to_string();
  let code:String = "200".to_string();
  let code404:String = "404".to_string();
 
  if s {
    a2221.insert(status_code,code);
  }else{
    a2221.insert(status_code,code404);
  }
  let crshi = res.text().await?;
  let data:String = "data".to_string();
  a2221.insert(data,crshi);
  Ok(a2221)
  
}

async fn post(url:String,body:HashMap<String, String>) -> Result<HashMap<String, String>, reqwest::Error>{
  // post 请求要创建client
  let client = reqwest::Client::new();

  // 组装header
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());

  // 组装要提交的数据
  let mut data = HashMap::new();
  data.insert("user", "zhangsan");
  data.insert("password", "https://docs.rs/serde_json/1.0.59/serde_json/");

  // 发起post请求并返回
  Ok(client.post("https://httpbin.org/post").headers(headers).json(&data).send().await?.json::<HashMap<String, String>>().await?)
}

#[napi(object)]
pub struct Config {
  pub method:Option<String>,
  pub url:String,
  pub body:Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct Pet1 {
  pub body: HashMap<String, String>,
}

#[napi(object)]
pub struct DonConfig {
  pub task_num:u32,
  pub url: String,
  pub path: String,
  pub file_name: String,
}
impl DonConfig{
  pub fn read_config(dm:DonConfig) -> Result<Self, &'static str> {
    let task_num = dm.task_num;
    let url = dm.url;
    let path = dm.path;
    let file_name = dm.file_name;
    Ok(Self { task_num, url, path, file_name })
  }
}
pub async fn check_request_range<U: IntoUrl>(url: U) -> Result<(bool, u64)> {
  let mut range = false;
  let req = reqwest::Client::new().head(url);
  let rep = req.send().await?;
  if !rep.status().is_success() {
      return Err(Error::msg("request fail"));
  }
  let headers = rep.headers();
  if headers
      .get(ACCEPT_RANGES)
      .map(|val| (val.to_str().ok()?.eq("bytes")).then(|| ()))
      .flatten()
      .is_some()
  {
      range = true;
  }
  let length = headers
      .get(CONTENT_LENGTH)
      .map(|val| val.to_str().ok())
      .flatten()
      .map(|val| val.parse().ok())
      .flatten()
      .ok_or(Error::msg("get length fail"))?;
  Ok((range, length))
}

async fn download<U: IntoUrl>(url: U, (mut start, end): (u64, u64), is_partial: bool,
    file: Arc<Mutex<File>>) -> Result<()> {
    let req = reqwest::Client::new().get(url);

    let req = if is_partial {
        if end == u64::MAX {
            req.header(RANGE, format!("bytes={}-{}", start, ""))
        } else {
            req.header(RANGE, format!("bytes={}-{}", start, end))
        }
    } else {
        req
    };
    let rep = req.send().await?;
    if !rep.status().is_success() {
        return Err(Error::msg("request fail"));
    }
    let mut stream = rep.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let mut chunk = chunk?;
        let mut file = file.lock().await;
        file.seek(SeekFrom::Start(start)).await?;
        start += chunk.len() as u64;
        file.write_all_buf(&mut chunk).await?;
    }
    Ok(())
}

pub async fn new_run<U: IntoUrl, P: AsRef<Path>>(url: U, path: P, task_num: u32) -> Result<()> {
  let url = url.into_url()?;
  let mut handles = vec![];
  let (range, length) = check_request_range(url.clone()).await?;
  let file = Arc::new(Mutex::new(File::create(&path).await?));
  let is_error = if range {
    let task_num1 = task_num as u64;
      let task_length = length / task_num1;
      for i in 0..(task_num - 1) {        // 线程数必须大于等于1
        let i1 = i as u64;
          let file = Arc::clone(&file);
          handles.push(tokio::spawn(download(
              url.clone(),
              (task_length * i1, task_length * (i1 + 1) - 1),
              true,
              file,
          )));
      }
      { 
          let file = Arc::clone(&file);
          handles.push(tokio::spawn(
              download(url.clone(), (task_length * (task_num1 - 1), u64::MAX), true, file)
          ));
      }
      
      let ret = join_all(handles).await;
      drop(file);
      ret.into_iter().flatten().any(|n| n.is_err())
  } else {
      download(url.clone(), (0, length - 1), false, file)
          .await
          .is_err()
  };
  if is_error {
      remove_file(&path).await?;
      Err(Error::msg("download file error"))
  } else {
      Ok(())
  }
}
#[tokio::main]
#[napi]
pub async fn wtDownload(dm:DonConfig) {
  let config = DonConfig::read_config(dm).unwrap();
  let file_path = Path::new(&config.path).join(&config.file_name);
  let now = Instant::now();
  fs::create_dir_all( Path::new(&config.path));
  new_run(&config.url, file_path, config.task_num).await.unwrap();
  println!("elasped time: {}", now.elapsed().as_secs_f32());
}

#[tokio::main]
#[napi]
async fn wtaxios(Configop:Config)->Pet1 {

    let a1: HashMap<String, String>;
    let a2: HashMap<String, String> = HashMap::new();
    let a3= "GET";
    let method = Configop.method.unwrap_or_else(||a3.to_string());
    let url = Configop.url;
    println!("{}",method);
    println!("{}",url);
    if method == "GET"{
      if let Ok(resp) = get(&url).await {
        return Pet1{
          body:resp
        };
      }
    }else if method == "POST"{
      let body = Configop.body.unwrap_or(a2);
      if let Ok(res) = post(url,body).await {
        println!("{:#?}", res);
        let res = res;
        a1 = res;
        return Pet1{body:a1};
      }
    }else if method == "GETTEXT"{
      if let Ok(resp) = get_calss_list_out_text(&url).await{
        println!("{:#?}",resp);
        let add = resp;
        return Pet1{body:add}
      };
    }
    let a2: HashMap<String, String> = HashMap::new();
    Pet1{body:a2}
   
}

//创建文件夹
fn create_dir(path: &Path) -> Result<(), std::io::Error> {
  fs::create_dir_all(path)
}

///解压
/// test.zip文件解压到d:/test文件夹下
///
fn extract(test: &Path, mut target: &Path) {

  let zipfile = std::fs::File::open(&test).unwrap();
  let mut zip = zip::ZipArchive::new(zipfile).unwrap();
  
 
  if !target.exists() {
      fs::create_dir_all(target).map_err(|e| {
          println!("{}", e);
      });
  }
 
  for i in 0..zip.len() {
      let mut file = zip.by_index(i).unwrap();
      println!("{}",GBK.decode(file.name_raw(), DecoderTrap::Strict).unwrap());
      let name = GBK.decode(file.name_raw(), DecoderTrap::Strict).unwrap();
      let nameinfo:&str = &name.to_string()[..];
      if file.is_dir() {
        
          println!("file utf8 path {:?}", file.name_raw());//文件名编码,在windows下用winrar压缩的文件夹，中文文夹件会码(发现文件名是用操作系统本地编码编码的，我的电脑就是GBK),本例子中的压缩的文件再解压不会出现乱码
          let target = target.join(Path::new(&file.name().replace("\\", "")));
          fs::create_dir_all(target).unwrap();
      } else {
          let file_path = target.join(Path::new(nameinfo));
          let mut target_file = if !file_path.exists() {
              println!("file path {}", file_path.to_str().unwrap());
              fs::File::create(file_path).unwrap()
          } else {
              fs::File::open(file_path).unwrap()
          };
          copy(&mut file, &mut target_file);
          // target_file.write_all(file.read_bytes().into());
      }
  }
}

#[napi]
pub fn wtExtractZip(test: String,path:String) {
  let testinfo: &str = &test.to_string()[..];
  let mut my_str = test;
  my_str.pop();
  my_str.pop();
  my_str.pop();
  my_str.pop();
  println!("{:?}",my_str);
  let mut my_str1 = my_str;
  println!("{:?}",my_str1);
  extract(Path::new(testinfo), Path::new(&my_str1));
}



