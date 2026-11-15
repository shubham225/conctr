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
 await delay(300);
}
await call('Page.enable'); await call('Runtime.enable');
await call('Network.enable');
await call('Network.setCacheDisabled', {cacheDisabled: true});
await call('Fetch.enable', {patterns:[{urlPattern:'https://fonts.googleapis.com/*',requestStage:'Request'}]});
await call('Emulation.setDeviceMetricsOverride', {width: 1280, height: 900, deviceScaleFactor: 1, mobile: false});
await navigate('/', 'input[placeholder="Connection name"]');
if (heldFontRequests) throw Error('Startup requested Google Fonts');
await check('startup paints without external Google Fonts', `performance.getEntriesByType('paint').some(p=>p.name==='first-contentful-paint') && ![...document.querySelectorAll('style')].some(s=>s.textContent.includes('fonts.googleapis.com'))`);
await call('Fetch.disable');
await call('Network.setCacheDisabled', {cacheDisabled: false});
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


await navigate('/components/chips','main button');
await evaluate(`document.querySelector('main button').click()`);await delay(200);
await check('chips have kit selection color',`document.querySelector('main button').className.includes('text-primary')`);
await navigate('/components/command','[data-name="Command"]');
await check('command uses kit palette surface',`getComputedStyle(document.querySelector('[data-name="Command"]')).borderRadius==='20px'`);
await navigate('/components/input-phone','[data-name="InputPhoneWrapper"] input');
await check('phone input uses 40px control height',`document.querySelector('[data-name="InputPhoneWrapper"] input').getBoundingClientRect().height===40`);
await navigate('/components/tabs','[data-name="TabsTrigger"]');
await check('active tabs use underline treatment',`(()=>{const b=document.querySelector('[data-name="TabsTrigger"][data-state="Active"]');return b&&getComputedStyle(b).borderBottomWidth==='2px'})()`);
await navigate('/components/checkbox','[role="checkbox"]');
await delay(1000);await evaluate(`document.querySelector('[role="checkbox"]').click()`);await delay(200);
await check('uncontrolled checkbox toggles',`document.querySelector('[role="checkbox"]').getAttribute('aria-checked')==='true'`);
await navigate('/components/calendar','.cn-composed-calendar');
await evaluate(`document.querySelector('button[aria-label="2026-11-15"]').click()`);await delay(200);
await check('calendar selects date',`document.querySelector('button[aria-label="2026-11-15"]').className.includes('bg-primary')`);
await evaluate(`document.querySelector('button[aria-label="Next month"]').click()`);await delay(200);
await check('calendar navigates months',`document.querySelector('select[aria-label="Select month"]').value==='12'`);
await navigate('/components/composed-date-picker','button[aria-label="Choose date"]');
await evaluate(`document.querySelector('button[aria-label="Choose date"]').click()`);await ready('.cn-composed-calendar');
await evaluate(`document.querySelector('button[aria-label="2026-11-15"]').click()`);await delay(200);
await check('date picker selects and closes',`document.querySelector('button[aria-label="Choose date"]').textContent.includes('2026-11-15')&&!document.querySelector('.cn-composed-calendar')`);
await navigate('/components/tag-input','input[aria-label="Add tag"]');
await evaluate(`document.querySelector('input[aria-label="Add tag"]').focus()`);
await call('Input.insertText',{text:'Orders'});
await call('Input.dispatchKeyEvent',{type:'keyDown',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});await call('Input.dispatchKeyEvent',{type:'keyUp',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});await delay(200);
await check('tag input adds a tag',`!!document.querySelector('button[aria-label="Remove Orders"]')`);
await evaluate(`document.querySelector('button[aria-label="Remove Orders"]').click()`);await delay(200);
await check('tag input removes a tag',`!document.querySelector('button[aria-label="Remove Orders"]')`);
await navigate('/components/composed-input-group','.cn-composed-input-group');
await check('input group shares a single border',`getComputedStyle(document.querySelector('.cn-composed-input-group > input')).borderTopWidth==='0px'`);
await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'no-preference'}]});
await navigate('/components/connection-flow','.cn-flow-packet');
await check('connection packet animates',`getComputedStyle(document.querySelector('.cn-flow-packet')).animationName==='cn-packet-horizontal'`);
await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
await check('packet stops for reduced motion',`getComputedStyle(document.querySelector('.cn-flow-packet')).animationName==='none'`);
await navigate('/components/calendar','select[aria-label="Select month"]');
await evaluate(`(()=>{const s=document.querySelector('select[aria-label="Select month"]');s.value='2';s.dispatchEvent(new Event('change',{bubbles:true}))})()`);await delay(200);
await evaluate(`(()=>{const s=document.querySelector('select[aria-label="Select year"]');s.value='2028';s.dispatchEvent(new Event('change',{bubbles:true}))})()`);await delay(200);
await check('separate month/year selection renders leap day',`!!document.querySelector('button[aria-label="2028-02-29"]')`);
await navigate('/components/composed-date-picker','button[aria-label="Choose date"]');
await evaluate(`document.querySelector('button[aria-label="Choose date"]').click()`);await ready('.cn-composed-calendar');
await check('date picker preview reserves popup space',`(()=>{const p=document.querySelector('main [style*="min-height"]');const c=document.querySelector('.cn-composed-calendar');return p.getBoundingClientRect().height>=480&&getComputedStyle(p).overflow==='visible'&&c.getBoundingClientRect().bottom<=p.getBoundingClientRect().bottom})()`);
await navigate('/components/date-picker','[data-name="DatePicker"]');
await check('upstream date picker preview reserves space',`(()=>{const p=document.querySelector('main [style*="min-height"]');return p&&p.getBoundingClientRect().height>=480&&getComputedStyle(p).overflow==='visible'})()`);
await navigate('/components/multi-select','[data-name="MultiSelectTrigger"]');
await delay(800);
await evaluate(`document.querySelector('[data-name="MultiSelectTrigger"]').click()`);await delay(300);
await check('multiselect preview contains open options',`(()=>{const p=document.querySelector('main [style*="min-height"]');const c=document.querySelector('[data-name="MultiSelectContent"]');return p&&p.getBoundingClientRect().height>=480&&getComputedStyle(p).overflow==='visible'&&c.dataset.state==='open'&&c.getBoundingClientRect().bottom<=p.getBoundingClientRect().bottom})()`);
console.log(JSON.stringify({checks:checks.length}));ws.close();
