const path = require('node:path');
const fs = require('node:fs');
const child_process = require('node:child_process');

const targets = {
    'x86_64-pc-windows-msvc': {
        folder: 'win64',
        files: ['steam_api64.dll', 'steam_api64.lib'],
        platform: 'win32',
        arch: 'x64'
    }
};

const targetArg = process.argv.at(-1);
const target = targets[targetArg] || targets['x86_64-pc-windows-msvc'];

const dist = path.join(__dirname, 'dist', target.folder);
const redist = path.join(__dirname, 'sdk/redistributable_bin', target.folder);
target.files.forEach(file => {
    const [source, dest] = [path.join(redist, file), path.join(dist, file)];
    try {
        fs.mkdirSync(path.dirname(dest), { recursive: true });
    } catch {}
    fs.copyFileSync(source, dest);
});

const relative = path.relative(process.cwd(), dist);
const params = [
    'build',
    '--platform',
    '--release',
    '--no-dts-header',
    '--js', 'false',
    '--dts', '../../client.d.ts',
    relative
];

child_process.spawn('napi', params, { stdio: 'inherit', shell: true })
    .on('exit', err => {
        if (err) {
            throw new Error(`Build failed with exit code ${err}`);
        }
    });