import wtaxios from '@wtapi_get/wtapi-win32-x64-msvc'

// var a1 = wtapi("https://www.baidu.com");
console.log(wtaxios)
var a2 = wtaxios({method:"GETTEXT",url:"https://api.hbqgame.com/game/sggzc/comm/experienceUrl"})
console.log('From native', a2)