#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate easy_http_request;

use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::value::Value;

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
    body:body,
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
 
  if s == true {
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

#[tokio::main]
#[napi]
async fn wtaxios(Configop:Config)->Pet1 {

    let a1: HashMap<String, String>;
    let a2: HashMap<String, String> = HashMap::new();
    let a3= "GET";
    let method = Configop.method.unwrap_or(a3.to_string());
    let url = Configop.url;
    println!("{}",method);
    println!("{}",url);
    if method == "GET"{
      if let Ok(resp) = get(&url).await {
        return Pet1{
          body:resp
        };
        
      }else{
        Pet1{
          body:a2
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
      
        return Pet1{
          body:add
        }
      };
    }
    let a2: HashMap<String, String> = HashMap::new();
    Pet1{
      body:a2
    }
   
}
