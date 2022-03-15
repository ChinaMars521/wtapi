import { sum,wtapi,wtaxios} from './index.js'

// var a1 = wtapi("https://www.baidu.com");
// console.log('From native', a1)
var a2 = wtaxios({method:"GETTEXT",url:"https://api.hbqgame.com/game/sggzc/comm/experienceUrl"})
console.log('From native', a2)