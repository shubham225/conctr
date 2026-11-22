// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
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
 await ready('main .cn-sidebar-provider[data-ready]'); await delay(500);
}
await call('Page.enable'); await call('Runtime.enable');
await call('Emulation.setFocusEmulationEnabled',{enabled:true});
await call('Emulation.setDeviceMetricsOverride', {width:1280,height:900,deviceScaleFactor:1,mobile:false});
await navigate('/components/sidebar', 'main .cn-sidebar-provider[data-ready]');
await check('desktop sidebar is 240px', `document.querySelector('main .cn-sidebar .cn-sidebar-desktop').getBoundingClientRect().width===240`);
await check('kit item geometry and active state', `(()=>{const a=document.querySelector('main .cn-sidebar .cn-sidebar-desktop a');const s=getComputedStyle(a);return a.getBoundingClientRect().height===36 && s.borderRadius==='6px' && a.getAttribute('aria-current')==='page' && s.backgroundColor==='rgb(238, 240, 255)'})()`);
await check('disabled item has no link', `!!document.querySelector('main .cn-sidebar .cn-sidebar-desktop span[aria-disabled="true"]')`);
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);
await delay(200);
await check('dark sidebar tokens and supplied logo', `(()=>{const a=document.querySelector('main .cn-sidebar .cn-sidebar-desktop a');return getComputedStyle(a).color==='rgb(129, 140, 248)' && getComputedStyle(document.querySelector('main .cn-sidebar .cn-sidebar-desktop .cn-sidebar-logo-dark')).display==='block'})()`);
await call('Emulation.setDeviceMetricsOverride', {width:390,height:844,deviceScaleFactor:1,mobile:true});
await check('mobile trigger replaces desktop sidebar', `document.querySelector('main .cn-sidebar .cn-sidebar-desktop').getBoundingClientRect().width===0 && document.querySelector('main [data-sidebar-trigger]').getBoundingClientRect().height>=44`);
await evaluate(`document.querySelector('main [data-sidebar-trigger]').focus();document.querySelector('main [data-sidebar-trigger]').click()`);
await check('drawer is modal with contained focus', `document.querySelector('main dialog').matches(':modal') && document.querySelector('main dialog').contains(document.activeElement) && document.querySelector('main [data-sidebar-trigger]').getAttribute('aria-expanded')==='true'`);
await call('Input.dispatchKeyEvent', {type:'keyDown', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
await call('Input.dispatchKeyEvent', {type:'keyUp', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
await delay(100);
await check('Escape closes and restores focus', `!document.querySelector('main dialog').open && document.activeElement.matches('[data-sidebar-trigger]') && document.activeElement.getAttribute('aria-expanded')==='false'`);
await evaluate(`document.querySelector('main [data-sidebar-trigger]').click();document.querySelector('main dialog a').click()`);
await check('navigation closes drawer', `!document.querySelector('main dialog').open`);
await evaluate(`document.querySelector('main [data-sidebar-trigger]').click();document.querySelector('main [data-sidebar-close]').click()`);
await check('close button dismisses drawer', `!document.querySelector('main dialog').open`);
if(errors.length) throw Error(errors.join('\n'));
console.log(`${checks.length} sidebar browser checks passed`); ws.close();
