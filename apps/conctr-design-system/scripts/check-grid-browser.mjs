// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
import fs from 'node:fs';
const base = process.argv[2] || 'http://127.0.0.1:18080';
const debug = process.argv[3] || 'http://127.0.0.1:19222';
const targets = await (await fetch(`${debug}/json`)).json();
const ws = new WebSocket(targets.find(t => t.type === 'page').webSocketDebuggerUrl);
await new Promise(resolve => { ws.onopen = resolve; });
let sequence = 0;
const pending = new Map();
const errors = [];
ws.onmessage = ({data}) => {
 const message = JSON.parse(data);
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
 await delay(300);
}
await call('Page.enable'); await call('Runtime.enable');
await call('Emulation.setDeviceMetricsOverride', {width: 1280, height: 900, deviceScaleFactor: 1, mobile: false});
await navigate('/', 'input[placeholder="Connection name"]');
await check('light brand primary, background and supplied logo', `getComputedStyle(document.querySelector('.cn-body')).getPropertyValue('--cn-primary').trim().toUpperCase()==='#4F46E5' && document.querySelector('header img').src.includes('horizontal-color')`);
await check('Sora headings and Inter interface', `getComputedStyle(document.querySelector('h1')).fontFamily.includes('Sora') && getComputedStyle(document.querySelector('.cn-body')).fontFamily.includes('Inter')`);
await check('40px button, 10px radius and disabled state', `(()=>{const b=[...document.querySelectorAll('button')].find(b=>b.textContent==='Test connection');return b.getBoundingClientRect().height===40 && getComputedStyle(b).borderRadius==='10px' && [...document.querySelectorAll('button')].some(b=>b.disabled)})()`);
await delay(2000);
const toggleRect = await evaluate(`(()=>{const r=document.querySelector('button[aria-label="Switch to dark theme"]').getBoundingClientRect();return {x:r.x+r.width/2,y:r.y+r.height/2}})()`);
await call('Input.dispatchMouseEvent',{type:'mouseMoved',...toggleRect});
await call('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,...toggleRect});
await call('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,...toggleRect}); await delay(300);
await check('controlled dark palette and dark logo', `document.querySelector('.cn-body').dataset.theme==='dark' && getComputedStyle(document.querySelector('.cn-body')).getPropertyValue('--cn-primary').trim().toUpperCase()==='#818CF8' && document.querySelector('header img').src.includes('color-on-dark')`);
await evaluate(`(()=>{const n=document.createElement('div');n.id='theme-check';n.dataset.theme='light';n.className='cn-body bg-background';document.querySelector('.cn-body').append(n)})()`);
await check('nested light scope resets palette', `getComputedStyle(document.querySelector('#theme-check')).getPropertyValue('--cn-primary').trim().toUpperCase()==='#4F46E5'`);
await call('Emulation.setDeviceMetricsOverride', {width: 390, height: 844, deviceScaleFactor: 1, mobile: true});
await check('narrow viewport has no document overflow', `document.documentElement.scrollWidth <= innerWidth`);
await call('Emulation.setDeviceMetricsOverride', {width: 1280, height: 900, deviceScaleFactor: 1, mobile: false});

await navigate('/components/data-grid','[data-name="DataGrid"]');
await evaluate(`document.querySelector('button[aria-label="Switch to dark theme"]').click()`);await delay(300);
for (let i=0;i<3;i++) {
 await evaluate(`document.querySelector('nav a[href="/components/button"]').click()`);await delay(400);
 await call('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,x:600,y:300});
 await call('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,x:600,y:300});await delay(100);
 if(errors.length) throw Error('Grid unmount click errors: '+JSON.stringify(errors));
 await evaluate(`document.querySelector('nav a[href="/components/data-grid"]').click()`);await ready('[data-name="DataGrid"]');
}
await check('grid listeners survive SPA unmount and remount',`!!document.querySelector('[data-name="DataGrid"]')`);
await check('grid borders use kit tokens',`getComputedStyle(document.querySelector('[data-name="DataGrid"]')).borderTopColor === getComputedStyle(document.querySelector('header')).borderBottomColor`);
await navigate('/components/table','main table');
await check('table headers use kit sunken surface',`getComputedStyle(document.querySelector('main th')).backgroundColor !== 'rgba(0, 0, 0, 0)'`);
await navigate('/components/command','[data-name="Command"]');
await evaluate(`document.querySelector('button[aria-label="Switch to dark theme"]').click()`);await delay(300);
await evaluate(`document.querySelector('[data-name="CommandItemLink"]').setAttribute('aria-selected','true')`);
await check('dark command selection uses primary text',`getComputedStyle(document.querySelector('[data-name="CommandItemLink"][aria-selected="true"]')).color === (()=>{const e=document.createElement('span');e.style.color='var(--cn-primary)';document.querySelector('.cn-body').append(e);const c=getComputedStyle(e).color;e.remove();return c})()`);
console.log(JSON.stringify({checks:checks.length,errors}));ws.close();
