import { wtDownload,wtExtractZip} from './index.js'

wtDownload({url:"https://dl1.qwp365.cn/buding/mod/3DMGAME-ERquanditucifucd.zip",taskNum:20,path:"./test/",fileName:"1.zip"});
console.log("123")
wtExtractZip("J:\\123123\\wtapi\\test\\1.zip","./test/")