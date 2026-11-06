import {test,expect} from '@playwright/test';
import fs from 'node:fs/promises';
import {gunzipSync}from'node:zlib';
const originalLines=gunzipSync(await fs.readFile(new URL('../../context/waves/2026-11-06-public-replay-self-service/validation/original-and-stale-export-v1/3-native.stdout.gz',import.meta.url))).toString('utf8').trimEnd().split('\n');
const nativeFrames=originalLines.slice(0,8).map(line=>JSON.parse(line).Ok);

test('a pending frame cannot regain a stale export when its filter changes',async({page})=>{
 await page.route('**/worker.js',async route=>{
  const response=await route.fetch();const original=await response.text();
  await route.fulfill({response,body:original+`\nconst originalHandler=self.onmessage;self.onmessage=async event=>{if(event.data.step===4)return new Promise(()=>{});return originalHandler(event);};\n`});
 });
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();
 await page.locator('#step').evaluate(e=>{e.value='4';e.dispatchEvent(new Event('input'));});
 await expect(page.locator('#export')).toBeDisabled();
 await page.locator('#filter').selectOption('sources');
 await expect(page.locator('#export')).toBeDisabled();
 await expect(page.locator('#events')).toHaveText('—');
 await expect(page.locator('#timeline li')).toHaveCount(0);
});

async function setStep(page,value){await page.locator('#step').evaluate((e,n)=>{e.value=String(n);e.dispatchEvent(new Event('input'));},value);}
async function observe(page){
 await page.addInitScript(()=>{window.__workers=[];const RealWorker=window.Worker;window.Worker=class extends RealWorker{
  constructor(...args){super(...args);this.sent=[];this.received=[];this.retired=false;window.__workers.push(this);this.addEventListener('message',e=>this.received.push(structuredClone(e.data)));}
  postMessage(data,...args){this.sent.push(structuredClone(data));return super.postMessage(data,...args);}
  terminate(){this.retired=true;return super.terminate();}
 };});
}
async function stall(page,stage){
 await page.route('**/worker.js',async route=>{const response=await route.fetch(),body=await response.text();
  const condition=stage==='init'?"event.data.type==='init'":"event.data.type==='frame'&&event.data.step===4";
  const block=stage==='cpu'?'while(true){}':'return new Promise(()=>{});';
  await route.fulfill({response,body:body+`\nconst original=self.onmessage;self.onmessage=event=>{if(${condition}){self.postMessage({type:'test-stalled',generation:event.data.generation,id:event.data.id});${block}}return original(event);};\n`});
 });
}
async function blank(page){
 for(const id of ['events','sources','receipts'])await expect(page.locator('#'+id)).toHaveText('—');
 for(const id of ['export','raw-export'])await expect(page.locator('#'+id)).toBeDisabled();
 await expect(page.locator('#timeline li')).toHaveCount(0);await expect(page.locator('#checkpoint')).toHaveText('');
}
async function bounded(page){expect(await page.evaluate(()=>window.__workers.filter(w=>!w.retired).length)).toBeLessThanOrEqual(1);}
async function late(page,index=0){
 await page.evaluate(({index,raw})=>{const worker=window.__workers[index],init=worker.sent.find(v=>v.type==='init'),request=worker.sent.findLast(v=>v.type==='frame');
  worker.onmessage({data:{type:'ready',generation:init.generation}});
  worker.onmessage({data:{type:'frame',generation:init.generation,id:request?.id,raw,ms:0}});
  worker.onmessage({data:{type:'error',generation:init.generation,id:request?.id,message:'RETIRED ERROR',fatal:true}});
 },{index,raw:nativeFrames[0]});
}
async function blocked(page){await expect.poll(()=>page.evaluate(()=>window.__workers.some(w=>w.received.some(v=>v.type==='test-stalled')))).toBe(true);}
async function download(page,id){const pending=page.waitForEvent('download');await page.locator('#'+id).click();const file=await pending;return {name:file.suggestedFilename(),bytes:await fs.readFile(await file.path())};}

test('every public prefix exports the exact native raw frame and original formatted JSON',async({browser,page,baseURL})=>{
 // Each independent prefix gets a fresh download context: no burst of sixteen
 // downloads shares Chromium's automatic-download policy state.
 for(let step=0;step<=7;step++){
  const context=await browser.newContext({baseURL,acceptDownloads:true});
  try{
   const prefix=await context.newPage();await prefix.goto('/WITNESS/?step='+step);
   await expect(prefix.locator('#events')).toHaveText(String(step));await expect(prefix.locator('#export')).toBeEnabled();
   await prefix.locator('#filter').selectOption(step%2?'evidence':'sources');
   const raw=await download(prefix,'raw-export'),formatted=await download(prefix,'export');
   expect(raw.name).toBe('witness-public-frame.raw.json');expect(raw.bytes.equals(Buffer.from(nativeFrames[step]))).toBe(true);
   expect(formatted.name).toBe('witness-public-frame.json');expect(formatted.bytes.equals(Buffer.from(JSON.stringify(JSON.parse(nativeFrames[step]),null,2)))).toBe(true);
  }finally{await context.close();}
 }
 await page.goto('/WITNESS/?step=7');await expect(page.locator('#events')).toHaveText('7');
 await page.locator('#step').focus();await page.keyboard.press('Home');await expect(page.locator('#events')).toHaveText('0');
 await page.keyboard.press('ArrowRight');await expect(page.locator('#events')).toHaveText('1');
 const keyboardRaw=await download(page,'raw-export');expect(keyboardRaw.bytes.equals(Buffer.from(nativeFrames[1]))).toBe(true);
});

test('Stop terminates a CPU-blocked worker and Retry keeps the selected step and filter',async({page})=>{
 await observe(page);await stall(page,'cpu');await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();
 await page.locator('#filter').selectOption('evidence');await setStep(page,4);await blocked(page);await page.locator('#stop-engine').click();
 await blank(page);await expect(page.locator('#status')).toContainText('engine stopped');await expect(page.locator('#retry')).toBeVisible();
 expect(await page.evaluate(()=>window.__workers[0].retired)).toBe(true);await late(page);await expect(page.locator('#status')).toContainText('engine stopped');
 await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('4');
 await expect(page.locator('#filter')).toHaveValue('evidence');await expect(page.locator('#validation')).toHaveText('Validation owed');await bounded(page);
});

test('initialization deadline retires a stalled engine and ignores late readiness',async({page})=>{
 await observe(page);await page.clock.install();await stall(page,'init');await page.goto('/WITNESS/');await blocked(page);
 await page.clock.fastForward(15001);await expect(page.locator('#status')).toContainText('loading timed out');await blank(page);
 expect(await page.evaluate(()=>window.__workers[0].retired)).toBe(true);await late(page);await expect(page.locator('#status')).toContainText('loading timed out');
 await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('0');await bounded(page);
});

test('projection deadline runs on the main thread while the worker is blocked',async({page})=>{
 await observe(page);await page.clock.install();await stall(page,'cpu');await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await setStep(page,4);await blocked(page);
 await page.clock.fastForward(10001);await expect(page.locator('#status')).toContainText('timed out after 10 seconds');await blank(page);
 expect(await page.evaluate(()=>window.__workers[0].retired)).toBe(true);await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('4');
 await late(page);await expect(page.locator('#events')).toHaveText('4');await bounded(page);
});

test('WASM load failure retries the original selected prefix without reload',async({page})=>{
 await observe(page);await page.route('**/*.wasm',route=>route.fulfill({status:503,body:'unavailable'}));await page.goto('/WITNESS/?step=6');
 await expect(page.locator('#status')).toContainText('could not load');await expect(page.locator('#retry')).toBeVisible();await blank(page);await expect(page.locator('#step')).toHaveValue('6');
 expect(await page.evaluate(()=>window.__workers[0].retired)).toBe(true);
 await page.unroute('**/*.wasm');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('6');await expect(page.locator('#checkpoint')).toContainText('sample:checkpoint');await bounded(page);
});

test('worker crash stops timed playback and does not restart itself',async({page})=>{
 await observe(page);await page.clock.install();await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await page.locator('#play').click();
 await page.evaluate(()=>window.__workers.at(-1).onerror({preventDefault(){}}));await expect(page.locator('#retry')).toBeVisible();await blank(page);
 const count=await page.evaluate(()=>window.__workers.length);await page.clock.fastForward(4000);expect(await page.evaluate(()=>window.__workers.length)).toBe(count);await expect(page.locator('#play')).toHaveText('Play trace');
 await late(page);await expect(page.locator('#status')).toContainText('could not load or run');await page.locator('#retry').click();await expect(page.locator('#export')).toBeEnabled();await bounded(page);
});

test('unreadable current response retires the engine and filters cannot revive a frame',async({page})=>{
 await observe(page);await stall(page,'promise');await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await setStep(page,4);await blocked(page);
 await page.evaluate(()=>{const w=window.__workers.at(-1),r=w.sent.findLast(v=>v.type==='frame');w.onmessage({data:{type:'frame',generation:r.generation,id:r.id,raw:'{"step":4,"events":[]}',ms:0}});});
 await expect(page.locator('#status')).toContainText('could not be read');await expect(page.locator('#retry')).toBeVisible();await page.locator('#filter').selectOption('sources');await blank(page);
 await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('4');await bounded(page);
});

test('rapid selections keep one live worker and reject retired results',async({page})=>{
 await observe(page);await stall(page,'promise');await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await setStep(page,4);await blocked(page);
 await page.locator('#step').evaluate(e=>{for(const n of [1,7,6,2,3]){e.value=String(n);e.dispatchEvent(new Event('input'));}});
 await expect(page.locator('#events')).toHaveText('3');await late(page);await expect(page.locator('#events')).toHaveText('3');await expect(page.locator('#step')).toHaveValue('3');await bounded(page);
});

test('late clipboard completion cannot replace newer frame status',async({page})=>{
 await page.addInitScript(()=>Object.defineProperty(navigator,'clipboard',{value:{writeText:()=>new Promise(resolve=>window.__releaseClipboard=resolve)},configurable:true}));
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await page.locator('#share').click();await expect.poll(()=>page.evaluate(()=>typeof window.__releaseClipboard)).toBe('function');
 await setStep(page,3);await expect(page.locator('#events')).toHaveText('3');await page.evaluate(()=>window.__releaseClipboard());await expect(page.locator('#status')).toContainText('step 3 of 7');
});

test('runtime trap retires the engine while retaining the selected step',async({page})=>{
 await observe(page);await page.route('**/worker.js',async route=>{const response=await route.fetch();const body=(await response.text()).replace('const raw=frame_json(data.step);',"const raw=(()=>{if(data.step===4)throw new WebAssembly.RuntimeError('injected trap before native call');return frame_json(data.step);})();");await route.fulfill({response,body});});
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await setStep(page,4);await expect(page.locator('#status')).toContainText('engine stopped');await blank(page);await expect(page.locator('#retry')).toBeVisible();
 await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#events')).toHaveText('4');await bounded(page);
});

test('ordinary native error strings retain the warm engine and recover on another step',async({page})=>{
 await observe(page);await page.route('**/worker.js',async route=>{const response=await route.fetch();const body=(await response.text()).replace('frame_json(data.step)','frame_json(data.step===4?8:data.step)');await route.fulfill({response,body});});
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();await setStep(page,4);await expect(page.locator('#status')).toContainText('between 0 and 7');await blank(page);
 expect(await page.evaluate(()=>window.__workers[0].retired)).toBe(false);await setStep(page,3);await expect(page.locator('#events')).toHaveText('3');expect(await page.evaluate(()=>window.__workers.length)).toBe(1);
});

test('runtime assets belong to one exact source revision',async({page})=>{
 const urls=[];page.on('response',response=>urls.push(response.url()));
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();
 const manifest=await(await page.request.get('/WITNESS/BUILD-MANIFEST.json')).json();
 expect(manifest.runtime).toBe(`assets/${manifest.commit}`);
 expect(await page.locator('script[type="module"]').getAttribute('src')).toBe(`${manifest.runtime}/app.js`);
 for(const name of ['app.js','replay-session.js','worker.js','witness_web.js','witness_web_bg.wasm','style.css']){
  const matches=urls.filter(url=>url.endsWith('/'+name));expect(matches.length,name).toBeGreaterThan(0);
  expect(matches.every(url=>url.includes(`/WITNESS/${manifest.runtime}/`)),name).toBeTruthy();
 }
});

test('source and exact native sample frames remain available without JavaScript',async({browser,baseURL})=>{
 const context=await browser.newContext({javaScriptEnabled:false,baseURL}),page=await context.newPage();
 try{
  await page.goto('/WITNESS/');await page.getByRole('link',{name:'Build guide & downloads'}).click();
  await expect(page.getByRole('heading',{name:'Receive without Git'})).toBeVisible();
  await expect(page.getByRole('link',{name:'Complete committed source (tar.xz)'})).toHaveAttribute('href','source.tar.xz');
  const response=await page.request.get('/WITNESS/native-frames.ndjson');expect(response.ok()).toBe(true);
  expect((await response.body()).equals(Buffer.from(nativeFrames.join('\n')+'\n'))).toBe(true);
  const receiver=await page.request.get('/WITNESS/receiver.zip');expect(receiver.ok()).toBe(true);
  expect((await receiver.body()).length).toBeLessThan(100000);
 }finally{await context.close();}
});
