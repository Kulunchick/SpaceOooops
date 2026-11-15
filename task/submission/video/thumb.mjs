import { createRequire } from 'node:module';
import fs from 'node:fs';
const require = createRequire('D:/Projects/SpaceOooops/e2e/');
const { chromium } = require('@playwright/test');
const F='file:///D:/Projects/SpaceOooops/assets/fonts/russo-one/russo-one-latin-400-normal.woff2';
fs.writeFileSync('thumb.html',`<!doctype html><style>
@font-face{font-family:R;src:url('${F}')}
html,body{margin:0;width:1280px;height:720px;overflow:hidden;background:#070b16}
img{position:absolute;left:0;top:-101px;width:1280px}
.bar{position:absolute;left:0;right:0;bottom:0;height:105px;background:linear-gradient(transparent,rgba(7,11,22,.92) 55%)}
.t{position:absolute;left:0;right:0;bottom:14px;text-align:center;font-family:R;font-size:38px;letter-spacing:.03em;color:#ffb23f;text-shadow:0 3px 0 #1a1203,0 0 18px rgba(0,0,0,.8)}
</style><img src="team-logo.png"><div class="bar"></div><div class="t">Design it. Launch it. Find out why it failed.</div>`);
const b=await chromium.launch(); const p=await b.newPage({viewport:{width:1280,height:720}});
await p.goto('file:///'+process.cwd().split(String.fromCharCode(92)).join('/')+'/thumb.html'); await p.evaluate(()=>document.fonts.ready); await p.waitForTimeout(400);
await p.screenshot({path:'SpaceOooops-thumbnail.png'}); await p.screenshot({path:'SpaceOooops-thumbnail.jpg',type:'jpeg',quality:92}); await b.close();
