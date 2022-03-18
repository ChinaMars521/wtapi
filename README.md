# `@wtapi_get/wtapi`
## 使用Rust开发的基础APi
目前接口  
    `wxaxios`  Get Post使用  
    `wtDownload` 下载文件使用  
    `wtExtractZip` 解压缩ZIP

``` JS

import {wxaxios, wtDownload,wtExtractZip} from './index.js'

wxaxios({method:"GETTEXT",url:"http://www.baidu.com"})
wtDownload({url:"https://dl1.qwp365.cn/buding/mod/3DMGAME-ERquanditucifucd.zip",taskNum:20,path:"./test/",fileName:"1.zip"});
wtExtractZip("J:\\123123\\wtapi\\test\\1.zip","./test/")
```
git地址 https://github.com/ChinaMars521/wtapi  
欢迎提交bug和一些建议  
linux 版本目前只兼容Centos,alpine 未兼容其他  
未来版本兼容ubuntu,有些版本linux需要更新opensll按需要安装
