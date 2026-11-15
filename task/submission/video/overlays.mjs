import { createRequire } from 'node:module';
const require = createRequire('D:/Projects/SpaceOooops/e2e/');
const { chromium } = require('@playwright/test');
const F='file:///D:/Projects/SpaceOooops/assets/fonts/';
const css=(russo,atk)=>`
@font-face{font-family:R;src:url('${russo}')}
@font-face{font-family:A;src:url('${atk}')}
html,body{margin:0;width:1920px;height:1080px;background:transparent;font-family:A;color:#eef3fb}
.box{position:absolute;background:rgba(13,22,42,.86);border-top:4px solid #ffb23f;padding:22px 30px;box-sizing:border-box}
.d{font-family:R;letter-spacing:.04em}
.acc{color:#ffb23f}.mut{color:#a9b8d0}`;
const pages={
 o1:`<div class="box" style="left:88px;top:56px;width:800px;padding:16px 28px">
   <div style="font-size:24px"><span class="mut" style="letter-spacing:.08em">CHALLENGE</span> &nbsp;<span class="d" style="font-size:30px">Space Mission Design Game</span></div>
   <div style="font-size:24px;margin-top:8px"><span class="mut" style="letter-spacing:.08em">TEAM</span> &nbsp;<span class="d acc" style="font-size:30px">SpaceOooops</span></div>
   <div style="font-size:25px;line-height:1.32;margin-top:8px;display:grid;grid-template-columns:1fr 1fr;column-gap:24px">
   <span>Mykyta Kalachuk</span><span>Artem Voloshyn</span><span>Danilo Andronov</span><span>Mykhailo Korotaiev</span><span>Anastasiia Savenko</span><span>Mykola Krasovskyi</span></div></div>`,
 o2:`<div class="box" style="left:640px;top:110px;width:1210px;text-align:center">
   <div class="d acc" style="font-size:60px">SpaceOooops</div>
   <div class="d" style="font-size:38px;margin-top:6px">Design it. Launch it. Find out why it failed.</div></div>`,
 o3:`<div class="box" style="left:880px;top:70px;width:920px;text-align:center">
   <div class="mut" style="font-size:26px;letter-spacing:.08em">PLAY IT NOW, FREE, IN YOUR BROWSER</div>
   <div class="d acc" style="font-size:44px;margin-top:6px">kulunchick.github.io/SpaceOooops</div></div>`};
const fs=require('node:fs');
const russo=F+'russo-one/russo-one-latin-400-normal.woff2', atk=F+'atkinson-hyperlegible/atkinson-hyperlegible-latin-400-normal.woff2';
console.log(russo,atk);
const b=await chromium.launch(); const p=await b.newPage({viewport:{width:1920,height:1080}});
for (const [k,h] of Object.entries(pages)){ fs.writeFileSync(k+'.html',`<!doctype html><style>${css(russo,atk)}</style>${h}`); await p.goto('file:///'+process.cwd().split(String.fromCharCode(92)).join('/')+'/'+k+'.html'); await p.evaluate(()=>document.fonts.ready); await p.waitForTimeout(300); await p.screenshot({path:k+'.png',omitBackground:true}); }
await b.close();
