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
.wm{position:absolute;right:14px;top:14px;font-family:Arial,sans-serif;font-weight:700;font-size:22px;color:#fff;background:rgba(7,11,22,.82);border:1px solid rgba(255,255,255,.55);padding:6px 12px;border-radius:6px;letter-spacing:.02em}
</style><img src="team-logo.png"><div class="wm">AI-generated image</div><div class="bar"></div><div class="t">Design it. Launch it. Find out why it failed.</div>`);
const b=await chromium.launch(); const p=await b.newPage({viewport:{width:1280,height:720}});
await p.goto('file:///'+process.cwd().split(String.fromCharCode(92)).join('/')+'/thumb.html'); await p.evaluate(()=>document.fonts.ready); await p.waitForTimeout(400);
await p.screenshot({path:'SpaceOooops-thumbnail.jpg',type:'jpeg',quality:92}); fs.writeFileSync('logo.html',`<!doctype html><style>html,body{margin:0;background:#070b16}img{display:block}.wm{position:absolute;right:10px;top:10px;font-family:Arial,sans-serif;font-weight:700;font-size:18px;color:#fff;background:rgba(7,11,22,.82);border:1px solid rgba(255,255,255,.55);padding:5px 10px;border-radius:6px}</style><img src="team-logo.png"><div class="wm">AI-generated image</div>`);
await p.setViewportSize({width:1024,height:657}); await p.goto('file:///'+process.cwd().split(String.fromCharCode(92)).join('/')+'/logo.html'); await p.waitForTimeout(300);
await p.screenshot({path:'team-logo-watermarked.png'}); await b.close();
