import { createRequire } from 'node:module';
import fs from 'node:fs';
const require = createRequire('D:/Projects/SpaceOooops/e2e/');
const { chromium } = require('@playwright/test');
fs.writeFileSync('logo700.html',`<!doctype html><style>html,body{margin:0;width:700px;height:450px;overflow:hidden;background:#070b16}
img{display:block;width:700px;height:450px}
.wm{position:absolute;right:8px;top:8px;font-family:Arial,sans-serif;font-weight:700;font-size:14px;color:#fff;background:rgba(7,11,22,.82);border:1px solid rgba(255,255,255,.55);padding:4px 8px;border-radius:5px}</style>
<img src="team-logo.png"><div class="wm">AI-generated image</div>`);
const b=await chromium.launch(); const p=await b.newPage({viewport:{width:700,height:450}});
await p.goto('file:///'+process.cwd().split(String.fromCharCode(92)).join('/')+'/logo700.html'); await p.waitForTimeout(300);
await p.screenshot({path:'team-logo-700x450.png'}); await b.close(); fs.unlinkSync('logo700.html');
