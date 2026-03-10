const fs = require('fs');
const path = 'd:\\Files\\Projects\\Zafkiel\\src\\lib\\components\\player\\VideoPlayer.svelte';

let content = fs.readFileSync(path, 'utf8');

const old2 = `\t\t\tconsole.error('[mpv] init error:', e);\n\t\t\thasError = true;\n\t\t\terrorMessage = e instanceof Error ? e.message : 'Failed to initialize player';\n\t\t}`;

const new2 = `\t\t\tconsole.error('[mpv] init error:', e);\n\t\t\thasError = true;\n\t\t\tconst errStr = e instanceof Error ? e.message : String(e);\n\t\t\t// Detect missing native library\n\t\t\tif (errStr.includes('libmpv-wrapper') || errStr.includes('FFI error') || errStr.includes('Failed to load')) {\n\t\t\t\terrorMessage = 'Native mpv library not found. On Windows, place libmpv-wrapper.dll and libmpv-2.dll next to the app executable.';\n\t\t\t} else {\n\t\t\t\terrorMessage = errStr || 'Failed to initialize player';\n\t\t\t}\n\t\t}`;

if (content.includes(old2)) {
  console.log('FOUND, replacing...');
  content = content.replace(old2, new2);
  fs.writeFileSync(path, content, 'utf8');
  console.log('Done');
} else {
  console.log('NOT FOUND');
  const idx = content.indexOf('init error');
  console.log('Context:', JSON.stringify(content.substring(idx - 10, idx + 200)));
}
