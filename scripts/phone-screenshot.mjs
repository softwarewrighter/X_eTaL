// Screenshot the live demo as a phone shows it (390 by 844, touch,
// twice the pixels), over the DevTools protocol: headless Chrome will
// not make a window that narrow from the command line.
//   node scripts/phone-screenshot.mjs URL OUT.png
import { spawn } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const [url, out] = process.argv.slice(2);
const chrome = spawn('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', [
  '--headless=new', '--remote-debugging-port=9339',
  '--user-data-dir=' + mkdtempSync(join(tmpdir(), 'phone-')), 'about:blank',
], { stdio: 'ignore' });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
await sleep(1500);
const tabs = await (await fetch('http://127.0.0.1:9339/json')).json();
const ws = new WebSocket(tabs.find((t) => t.type === 'page').webSocketDebuggerUrl);
await new Promise((r) => (ws.onopen = r));
let id = 0;
const wait = {};
ws.onmessage = (m) => { const d = JSON.parse(m.data); if (wait[d.id]) { wait[d.id](d); delete wait[d.id]; } };
const cmd = (method, params = {}) =>
  new Promise((r) => { const i = ++id; wait[i] = r; ws.send(JSON.stringify({ id: i, method, params })); });
await cmd('Emulation.setDeviceMetricsOverride', { width: 390, height: 844, deviceScaleFactor: 2, mobile: true });
await cmd('Emulation.setTouchEmulationEnabled', { enabled: true });
await cmd('Page.navigate', { url });
await sleep(4000);
writeFileSync(out, Buffer.from((await cmd('Page.captureScreenshot')).result.data, 'base64'));
console.log(out);
ws.close();
chrome.kill();
