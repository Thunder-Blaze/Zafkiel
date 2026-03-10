with open(r'd:\Files\Projects\Zafkiel\src\lib\components\player\VideoPlayer.svelte', 'rb') as f:
    content = f.read()

old2 = (
    b"\t\t\tconsole.error('[mpv] init error:', e);\n"
    b"\t\t\thasError = true;\n"
    b"\t\t\terrorMessage = e instanceof Error ? e.message : 'Failed to initialize player';\n"
    b"\t\t}"
)
new2 = (
    b"\t\t\tconsole.error('[mpv] init error:', e);\n"
    b"\t\t\thasError = true;\n"
    b"\t\t\tconst errStr = e instanceof Error ? e.message : String(e);\n"
    b"\t\t\t// Detect missing native library\n"
    b"\t\t\tif (errStr.includes('libmpv-wrapper') || errStr.includes('FFI error') || errStr.includes('Failed to load')) {\n"
    b"\t\t\t\terrorMessage = 'Native mpv library not found. On Windows, place libmpv-wrapper.dll and libmpv-2.dll next to the app executable.';\n"
    b"\t\t\t} else {\n"
    b"\t\t\t\terrorMessage = errStr || 'Failed to initialize player';\n"
    b"\t\t\t}\n"
    b"\t\t}"
)

if old2 in content:
    print('FOUND, replacing...')
    content = content.replace(old2, new2)
    with open(r'd:\Files\Projects\Zafkiel\src\lib\components\player\VideoPlayer.svelte', 'wb') as f:
        f.write(content)
    print('Done')
else:
    print('NOT FOUND')
    idx = content.find(b"init error")
    print(repr(content[idx-10:idx+200]))
