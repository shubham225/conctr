// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
import fs from 'node:fs';
const base = process.argv[2] || 'http://127.0.0.1:18083';
const debug = process.argv[3] || 'http://127.0.0.1:19224';
const targets = await (await fetch(`${debug}/json`)).json();
const ws = new WebSocket(targets.find(t => t.type === 'page').webSocketDebuggerUrl);
await new Promise(resolve => { ws.onopen = resolve; });
let sequence = 0;
const pending = new Map();
const errors = [];
let heldFontRequests = 0;
ws.onmessage = ({data}) => {
 const message = JSON.parse(data);
 // Hold the old CLI template's external font request to reproduce offline startup.
 if (message.method === 'Fetch.requestPaused') heldFontRequests++;
 if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails.text);
 if (message.id) { pending.get(message.id)?.(message); pending.delete(message.id); }
};
async function call(method, params = {}) {
 const id = ++sequence;
 const result = new Promise(resolve => pending.set(id, resolve));
 ws.send(JSON.stringify({id, method, params}));
 const message = await result;
 if (message.error) throw Error(JSON.stringify(message.error));
 return message.result;
}
async function evaluate(expression) {
 const result = await call('Runtime.evaluate', {expression, returnByValue: true, awaitPromise: true});
 if (result.exceptionDetails) throw Error(JSON.stringify(result.exceptionDetails));
 return result.result.value;
}
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
async function ready(selector, attempts = 120) {
 for (let i = 0; i < attempts; i++) {
  await delay(500);
  if (await evaluate(`!!document.querySelector(${JSON.stringify(selector)})`)) return;
 }
 throw Error(`Timed out waiting for ${selector}`);
}
const checks = [];
const upstreamIssues = [];
async function check(name, expression) {
 if (!await evaluate(expression)) throw Error(`Failed: ${name}`);
 checks.push(name); console.log(`PASS ${name}`);
}
async function navigate(path, selector) {
 errors.length = 0;
 await call('Page.navigate', {url: base + path});
 await ready(selector);
 await ready('nav a[data-dioxus-id]');
 await delay(500);
}

const hostWindow=await call('Browser.getWindowForTarget');
await call('Browser.setWindowBounds',{windowId:hostWindow.windowId,bounds:{width:1600,height:1400}});
await call('Page.enable');await call('Runtime.enable');await call('Emulation.setFocusEmulationEnabled',{enabled:true});
await call('Emulation.setDeviceMetricsOverride',{width:1440,height:1100,deviceScaleFactor:1,mobile:false});
await navigate('/components/visual-field-mapping','[data-visual-mapper]');

const root='document.querySelector("[data-visual-mapper]")';
async function button(text){ await evaluate(`[...${root}.querySelectorAll('button')].find(b=>b.textContent.trim().endsWith(${JSON.stringify(text)})).click()`);await delay(150); }
async function fill(label,value){ await evaluate(`var el=document.querySelector('[aria-label="${label}"]');Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(el,${JSON.stringify(value)});el.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(100); }
async function drag(from,to){
 const points=await evaluate(`(()=>{const a=document.querySelector(${JSON.stringify(from)}),b=document.querySelector(${JSON.stringify(to)});a.scrollIntoView({block:'center'});const x=a.getBoundingClientRect(),y=b.getBoundingClientRect();return {x:x.x+x.width/2,y:x.y+x.height/2,tx:y.x+y.width/2,ty:y.y+y.height*.8}})()`);
 await call('Input.dispatchMouseEvent',{type:'mouseMoved',x:points.x,y:points.y});
 await call('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',buttons:1,clickCount:1,x:points.x,y:points.y});
 for(let i=1;i<=15;i++){await call('Input.dispatchMouseEvent',{type:'mouseMoved',button:'left',buttons:1,x:points.x+(points.tx-points.x)*i/15,y:points.y+(points.ty-points.y)*i/15});await delay(30);}
 await call('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',buttons:0,clickCount:1,x:points.tx,y:points.ty});await delay(250);
}
await check('editor mounts delegated gestures',`${root}.dataset.editorReady==='true'`);
await check('no persistent three dot menus',`!${root}.querySelector('details summary[aria-label]')`);
await check('node actions start disabled',`[...${root}.querySelectorAll('button')].find(b=>b.textContent==='Edit node').disabled`);
await evaluate(`${root}.querySelector('[data-side="source"][data-field-path*=name]').click()`);await delay(100);
await check('selection enables node toolbar',`![...${root}.querySelectorAll('button')].find(b=>b.textContent==='Edit node').disabled`);
await button('Edit node');await check('node modal opens',`document.querySelector('[data-node-dialog]').open`);
await fill('Node name','Customer display name');await fill('Node path',"$['customer']['display']");await evaluate(`document.querySelector('[data-node-dialog] form').requestSubmit()`);await delay(150);
await check('edit updates node and closes dialog',`!document.querySelector('[data-node-dialog]') && ${root}.textContent.includes('Customer display name')`);
await button('New source');await fill('Node name','External reference');await fill('Node path',"$['reference']");await evaluate(`document.querySelector('[data-node-dialog] form').requestSubmit()`);await delay(150);
await check('new source node is created',`${root}.querySelectorAll('[data-column="source"] [data-node-path]').length===5`);
await button('New target');await fill('Node name','Destination reference');await fill('Node path',"$['destination']");await evaluate(`document.querySelector('[data-node-dialog] form').requestSubmit()`);await delay(150);
await check('new target node is created',`${root}.querySelectorAll('[data-column="target"] [data-node-path]').length===5`);
await drag('[data-column="source"] [data-node-path]:last-child [data-drag-handle]','[data-column="source"] [data-node-path]:first-child');
await check('real pointer drag moves source node within column',`${root}.querySelectorAll('[data-column="source"] [data-node-path]')[1].textContent.includes('External reference')`);
await drag('[data-column="target"] [data-node-path]:last-child [data-drag-handle]','[data-column="target"] [data-node-path]:first-child');
await check('real pointer drag moves target node within column',`${root}.querySelectorAll('[data-column="target"] [data-node-path]')[1].textContent.includes('Destination reference')`);
await drag('[data-column="source"] [data-node-path]:nth-child(2) [data-link-port]','[data-column="target"] [data-node-path]:nth-child(2)');
await check('real pointer handle drag creates link',`${root}.querySelectorAll('[data-link-id]').length===2 && !!${root}.querySelector('[data-transform-picker]')`);
await button('Add transformer');await fill('Search transformers','trim');await evaluate(`document.querySelector('[data-transform-option="trim"]').click()`);await delay(150);
await button('Add transformer');await fill('Search transformers','uppercase');await evaluate(`document.querySelector('[data-transform-option="uppercase"]').click()`);await delay(150);
await check('pipeline supports multiple steps',`${root}.querySelectorAll('[data-transform-step]').length===2`);
await evaluate(`document.querySelector('[aria-label="Move transform 2 up"]').click()`);await delay(150);
await check('pipeline order changes',`${root}.querySelector('[data-transform-step]').dataset.transformStep==='uppercase'`);
await check('YAML includes pipeline',`${root}.querySelector('pre').textContent.includes('uppercase')`);
await evaluate(`${root}.querySelector('[data-column="source"] [data-node-path]:nth-child(2) [data-field-path]').click()`);await delay(100);
await button('Ignore / restore');
await check('ignore prunes selected node links',`${root}.querySelectorAll('[data-link-id]').length===1 && ${root}.querySelector('[data-column="source"] [data-node-path]:nth-child(2)').dataset.ignored==='true'`);
await button('Ignore / restore');await check('restore does not recreate links',`${root}.querySelectorAll('[data-link-id]').length===1 && ${root}.querySelector('[data-column="source"] [data-node-path]:nth-child(2)').dataset.ignored==='false'`);
await button('Remove node');await check('node removal updates column',`${root}.querySelectorAll('[data-column="source"] [data-node-path]').length===4`);
await evaluate(`${root}.querySelector('[data-connection]').click()`);await delay(100);await button('Remove link');
await check('toolbar removes selected link',`${root}.querySelectorAll('[data-link-id]').length===0`);
await button('New link');await evaluate(`${root}.querySelector('[data-column="source"] [data-field-path]').click()`);await delay(100);await evaluate(`${root}.querySelector('[data-column="target"] [data-field-path]').click()`);await delay(150);
await check('New link supports keyboard-accessible card actions',`${root}.querySelectorAll('[data-link-id]').length===1`);
await evaluate(`[...document.querySelectorAll('main button')].find(b=>b.textContent.trim()==='Load normalized XML example').click()`);await delay(150);
await check('caller reimport overwrites inventories and links',`${root}.querySelectorAll('[data-link-id]').length===0 && ${root}.querySelectorAll('[data-column="source"] [data-node-path]').length===3 && ${root}.textContent.includes('Order service')`);
await button('New source');await call('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await call('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await delay(150);
await check('Escape dismisses modal',`!document.querySelector('[data-node-dialog]')`);
await check('read-only example has no mutation toolbar',`![...document.querySelectorAll('[data-visual-mapper]')[1].querySelectorAll('button')].some(b=>b.textContent==='New source')`);
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);await delay(150);
await check('dark theme applies',`getComputedStyle(${root}).backgroundColor==='rgb(18, 25, 51)'`);
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-mapping-editor.png',Buffer.from(r.data,'base64')));
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});await delay(150);
await check('mobile workspace scrolls without page overflow',`document.documentElement.scrollWidth<=390`);
console.log(JSON.stringify({checks:checks.length,errors},null,2));if(errors.length)process.exitCode=1;ws.close();
