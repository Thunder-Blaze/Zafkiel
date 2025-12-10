import torrentStream from 'torrent-stream';
import http from 'http';
import rangeParser from 'range-parser';

const magnet = process.argv[2];

console.log(JSON.stringify({ debug: "Script started with torrent-stream", args: process.argv }));

if (!magnet) {
    console.error(JSON.stringify({ error: "No magnet link provided" }));
    process.exit(1);
}

const engine = torrentStream(magnet, {
    path: '/tmp/zafkiel_downloads',
    connections: 100,
    uploads: 10,
    verify: true,
    dht: true,
    tracker: true
});

engine.on('ready', () => {
    console.log(JSON.stringify({ debug: "Engine ready, finding files..." }));

    // Find the largest file (video)
    const file = engine.files.reduce((a, b) => a.length > b.length ? a : b);

    console.log(JSON.stringify({ debug: "Selected file", name: file.name, length: file.length }));

    file.select(); // Prioritize this file

    const server = http.createServer((req, res) => {
        const range = req.headers.range;

        if (!range) {
            // 200 OK - Full content
            res.writeHead(200, {
                'Content-Length': file.length,
                'Content-Type': 'video/mp4', // Assuming mp4/mkv, browser might sniff or we can detect
                'Accept-Ranges': 'bytes'
            });
            const stream = file.createReadStream();
            stream.pipe(res);
        } else {
            // 206 Partial Content
            const ranges = rangeParser(file.length, range);

            if (ranges === -1 || ranges === -2 || ranges.length !== 1) {
                res.statusCode = 416;
                res.end();
                return;
            }

            const { start, end } = ranges[0];

            res.writeHead(206, {
                'Content-Range': `bytes ${start}-${end}/${file.length}`,
                'Accept-Ranges': 'bytes',
                'Content-Length': (end - start) + 1,
                'Content-Type': 'video/mp4'
            });

            const stream = file.createReadStream({ start, end });
            stream.pipe(res);
        }
    });

    server.listen(0, () => {
        const port = (server.address() as any).port;
        const url = `http://localhost:${port}`;

        console.log(JSON.stringify({
            success: true,
            url: url,
            filename: file.name,
            infoHash: engine.infoHash
        }));
    });
});

engine.on('error', (err: any) => {
    console.error(JSON.stringify({ error: err.message || "Unknown torrent error" }));
});

// Keep process alive
setInterval(() => { }, 10000);
