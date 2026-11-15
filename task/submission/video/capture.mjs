import { createRequire } from 'node:module';
import { mkdirSync, writeFileSync, rmSync } from 'node:fs';
const require = createRequire('D:/Projects/SpaceOooops/e2e/');
const { chromium } = require('@playwright/test');
const SHARE='v1.crop-watch.multispectral-imager.cubesat-6u.solar-deployable~f.battery-m~f.reaction-wheels~f.comms-s-band~f.f.sso-600.cubesat-rideshare';
const base='http://localhost:4173/SpaceOooops/';
const take = process.argv[2];
const b = await chromium.launch({ headless: true, args: ['--ignore-gpu-blocklist','--use-angle=d3d11','--enable-gpu','--autoplay-policy=no-user-gesture-required','--mute-audio'] });
const ctx = await b.newContext({ viewport: { width: 1920, height: 1080 } });
const page = await ctx.newPage();
await page.addInitScript((v)=>{ Math.random=()=>v; }, ((take.startsWith('fx')?Number(take.slice(2)):(take==='4'||take==='5'?19:28))+0.5)/4294967296);
const cdp = await ctx.newCDPSession(page);
const dir = `take${take}`; rmSync(dir,{recursive:true,force:true}); mkdirSync(dir);
const frames=[]; let t0=null; const marks=[];
cdp.on('Page.screencastFrame', async (f)=>{
  const ts=f.metadata.timestamp; if (t0===null) t0=ts;
  const name=`f${String(frames.length).padStart(5,'0')}.jpg`;
  writeFileSync(`${dir}/${name}`, Buffer.from(f.data,'base64')); frames.push([name, ts-t0]);
  cdp.send('Page.screencastFrameAck',{sessionId:f.sessionId}).catch(()=>{});
});
const start=()=>cdp.send('Page.startScreencast',{format:'jpeg',quality:93,everyNthFrame:1});
const mark=(m)=>{ const t=frames.length?frames[frames.length-1][1]:0; marks.push([m,t]); console.log(m,t.toFixed(2)); };
const wait=(ms)=>page.waitForTimeout(ms);
if (take==='1') {
  await page.goto(base); await wait(3000); await start(); await wait(7000);
} else if (take==='2') {
  await page.goto(base+'#share='+SHARE); await wait(2500);
  await page.getByRole('button',{name:/hide hints/i}).click(); await wait(800);
  await start(); await wait(300); mark('design');
  // turn the spacecraft
  await page.mouse.move(1250,450); await page.mouse.down(); await page.mouse.move(1500,470,{steps:40}); await page.mouse.up();
  await wait(1200);
  const sar=page.getByRole('button',{name:/^Synthetic-aperture radar/});
  await sar.evaluate(e=>e.scrollIntoView({behavior:'smooth',block:'center'})); await wait(1100); mark('sar-visible');
  await sar.click(); mark('sar-click'); await wait(2600);
  await page.getByRole('button',{name:'Launch Review'}).click(); mark('review'); await wait(3500);
} else if (take.startsWith('fx')) {
  const kind={133:'visitor',14:'hit',6:'panelShading',48:'bonusGlow',42:'linkInterference'}[Number(take.slice(2))];
  await page.goto(base+'#share='+SHARE); await wait(2500);
  await page.getByRole('button',{name:/hide hints/i}).click();
  await page.getByRole('button',{name:'Launch Review'}).click(); await wait(1500);
  await page.getByRole('button',{name:'Launch',exact:true}).click();
  const sk=page.getByRole('button',{name:'Skip launch'}); await sk.waitFor({state:'visible',timeout:8000}); await sk.click();
  await page.locator('.run-screen').waitFor(); await wait(300);
  await start(); mark('run');
  const cv=page.locator('canvas.scene-canvas'); const t1=Date.now(); let on=false;
  while (Date.now()-t1<40000) { const e=(await cv.getAttribute('data-effects'))||''; if (!on && e.includes(kind)) { on=true; mark('fx '+kind+' xy='+(await cv.getAttribute('data-craft-xy'))); } else if (on && !e.includes(kind)) { mark('fxend'); break; } await wait(30); }
  await wait(600);
} else if (take==='5' || take==='6') {
  const maxN = take==='5' ? 3 : 1;
  await page.goto(base+'#share='+SHARE); await wait(2500);
  await page.getByRole('button',{name:/hide hints/i}).click();
  await page.getByRole('button',{name:'Launch Review'}).click(); await wait(1500);
  await page.getByRole('button',{name:'Launch',exact:true}).click();
  const sk=page.getByRole('button',{name:'Skip launch'}); await sk.waitFor({state:'visible',timeout:8000}); await sk.click();
  await page.locator('.run-screen').waitFor(); await wait(600);
  await start(); await wait(300); mark('run');
  const cv=page.locator('canvas.scene-canvas'); const fx=()=>cv.getAttribute('data-effects');
  const opt=page.locator('.decision-option').first(); let n=0; const t0w=Date.now();
  while (n<maxN && Date.now()-t0w<120000) {
    if (await opt.isVisible()) { n++;
      await page.getByRole('button',{name:'1×'}).click();
      mark('prompt'+n+' '+(await page.locator('.decision-prompt').innerText()).split(String.fromCharCode(10)).slice(0,2).join(' ').slice(0,80)); await wait(1200);
      await opt.click(); mark('answered'+n);
      let seen=''; const t1=Date.now();
      while (Date.now()-t1<6000) { const e=await fx(); if (e && e!==seen) { seen=e; mark('fx '+e); } if (seen && !e && Date.now()-t1>1500) break; await wait(50); }
      mark('fxend'+n); await wait(400);
      if (n<maxN) await page.getByRole('button',{name:'4×'}).click();
    } else await wait(80);
  }
  await wait(500);
} else if (take==='4') {
  await page.goto(base+'#share='+SHARE); await wait(2500);
  await page.getByRole('button',{name:/hide hints/i}).click();
  await page.getByRole('button',{name:'Launch Review'}).click(); await wait(1500);
  await page.getByRole('button',{name:'Launch',exact:true}).click();
  const sk=page.getByRole('button',{name:'Skip launch'}); await sk.waitFor({state:'visible',timeout:8000}); await sk.click();
  await page.locator('.run-screen').waitFor(); await wait(600);
  await start(); await wait(400); mark('run');
  const opt=page.locator('.decision-option').first(); let n=0; const tStart=Date.now();
  while (n<5 && Date.now()-tStart<90000) {
    if (await opt.isVisible()) { n++; const txt=(await page.locator('.decision-prompt').innerText()).replace(/\s+/g,' ').slice(0,110); mark('prompt'+n+' '+txt); await wait(1900); await opt.click(); mark('answered'+n); await wait(1300);
      if (n===1) { await page.getByRole('button',{name:'4×'}).click(); mark('4x'); } }
    else await wait(100);
  }
  await wait(1200); await page.getByRole('button',{name:/Skip/}).click(); mark('skip'); await wait(500);
  const d=page.getByRole('button',{name:'Debrief'});
  for (let i=0;i<30 && !(await page.locator('.verdict-title').isVisible());i++){ if (await opt.isVisible()) await opt.click(); else if (await d.isVisible()&&await d.isEnabled()) await d.click(); await wait(200);}
  mark('debrief'); await wait(3000);
  await page.locator('.verdict-title').evaluate(()=>{ const el=[...document.querySelectorAll('*')].find(e=>e.scrollHeight>e.clientHeight+200 && getComputedStyle(e).overflowY!=='visible'); (el||document.scrollingElement).scrollBy({top:700,behavior:'smooth'}); });
  mark('scroll'); await wait(4500);
} else {
  await page.goto(base+'#share='+SHARE); await wait(2500);
  await page.getByRole('button',{name:/hide hints/i}).click();
  await page.getByRole('button',{name:'Launch Review'}).click(); await wait(1500);
  await start(); await wait(500);
  await page.getByRole('button',{name:'Launch',exact:true}).click(); mark('launch');
  await page.locator('.decision-prompt').waitFor({state:'visible',timeout:40000}); mark('prompt'); await wait(1800);
  await page.locator('.decision-option').first().click(); mark('answered'); await wait(1500);
  await page.getByRole('button',{name:'4×'}).click(); mark('4x');
  for (let i=0;i<40;i++){ await wait(250); if (await page.locator('.decision-option').first().isVisible()) { mark('prompt2'); await wait(700); await page.locator('.decision-option').first().click(); } if (frames[frames.length-1][1]-marks.find(m=>m[0]==='4x')[1] > 7) break; }
  await page.getByRole('button',{name:/Skip/}).click(); mark('skip'); await wait(600);
  const d=page.getByRole('button',{name:'Debrief'}); if (await d.isVisible() && await d.isEnabled()) await d.click();
  for (let i=0;i<20 && !(await page.locator('.verdict-title').isVisible());i++){ if (await page.locator('.decision-option').first().isVisible()) await page.locator('.decision-option').first().click(); else if (await d.isVisible()&&await d.isEnabled()) await d.click(); await wait(200);}
  mark('debrief'); await wait(2500);
  await page.locator('.verdict-title').evaluate(()=>{ const el=[...document.querySelectorAll('*')].find(e=>e.scrollHeight>e.clientHeight+200 && getComputedStyle(e).overflowY!=='visible'); (el||document.scrollingElement).scrollBy({top:700,behavior:'smooth'}); });
  mark('scroll'); await wait(4000);
}
await cdp.send('Page.stopScreencast'); await wait(300);
const end=frames[frames.length-1][1];
let list=''; for (let i=0;i<frames.length;i++){ const dur=(i+1<frames.length?frames[i+1][1]:end+0.033)-frames[i][1]; list+=`file '${frames[i][0]}'\nduration ${Math.max(dur,0.001).toFixed(4)}\n`; }
writeFileSync(`${dir}/list.txt`,list); writeFileSync(`${dir}/marks.json`,JSON.stringify(marks));
console.log('frames',frames.length,'dur',end.toFixed(2),'fps',(frames.length/end).toFixed(1));
await b.close();
