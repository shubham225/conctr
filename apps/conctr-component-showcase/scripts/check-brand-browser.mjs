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
await navigate('/components/input-otp', '[data-slot="input-otp"] input');
await evaluate(`document.querySelector('[data-slot="input-otp"] input').focus()`);
await call('Input.insertText', {text:'123456'}); await delay(200);
await check('native OTP editing updates Rust UI slot overlays', `[...document.querySelectorAll('[data-slot="input-otp"] span.pointer-events-none')].map(e=>e.textContent).join('')==='123456'`);
await check('OTP requires no missing fixed-path script', `!document.querySelector('script[src="/components/otp.js"]')`);
await navigate('/components/conctr-workflow', 'section[aria-label="Workflow builder"]');
await check('Conctr workflow uses provided source hub target artwork', `['REST source','Conctr mapping','SOAP target'].every(text=>document.querySelector('main').textContent.includes(text)) && document.querySelectorAll('main img').length===3`);
const zoom = await evaluate(`document.querySelector('section[aria-label="Workflow builder"]').textContent.match(/\d+%/)?.[0]`);
await evaluate(`([...document.querySelectorAll('section[aria-label="Workflow builder"] button')].find(b=>b.textContent.trim()==='+')).click()`); await delay(200);
await check('workflow zoom changes canvas state', `!document.querySelector('section[aria-label="Workflow builder"]').textContent.includes(${JSON.stringify(zoom)})`);
await call('Emulation.setEmulatedMedia', {features:[{name:'prefers-reduced-motion',value:'reduce'}]});
await check('reduced motion is respected', `getComputedStyle(document.querySelector('button')).transitionDuration.split(',').every(t=>parseFloat(t)<0.001)`);
await navigate('/components/badge', 'main span');
await check('badge uses kit pill radius and height', `(()=>{const b=document.querySelector('main span');return parseFloat(getComputedStyle(b).borderRadius)>=12 && b.getBoundingClientRect().height===24})()`);
await navigate('/components/alert', 'main [role="alert"]');
await check('alert uses kit radius and soft info surface', `(()=>{const a=document.querySelector('main [role="alert"]');return getComputedStyle(a).borderRadius==='10px' && getComputedStyle(a).backgroundColor!== 'rgba(0, 0, 0, 0)'})()`);
await navigate('/components/toast', 'main button');
await evaluate(`document.querySelector('main button').click()`);await delay(200);
await check('toast opens and has kit radius', `getComputedStyle(document.querySelector('[role="status"]')).borderRadius==='10px'`);
await evaluate(`document.querySelector('[aria-label="Dismiss notification"]').click()`);await delay(200);
await check('toast dismisses', `!document.querySelector('[role="status"]')`);
for (const slug of ['badges-and-shapes','feedback','empty-state','connection-flow']) {
 await navigate('/components/'+slug,'main h1');
 await check('composed '+slug, `document.querySelector('main').textContent.includes('View example source')`);
}
console.log(JSON.stringify({checks:checks.length}));ws.close();
