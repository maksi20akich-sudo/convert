// convert.js
const fs = require('fs');
const path = require('path');
const sharp = require('sharp');
const glob = require('glob');

async function convertPngToJpg(inputPath, outputPath, quality, resizeW, resizeH) {
    let pipeline = sharp(inputPath);
    if (resizeW > 0 && resizeH > 0) {
        pipeline = pipeline.resize(resizeW, resizeH);
    }
    await pipeline
        .jpeg({ quality: quality })
        .toFile(outputPath);
}

function findPngFiles(inputs, recursive) {
    const files = [];
    for (const item of inputs) {
        if (fs.existsSync(item) && fs.statSync(item).isFile() && item.toLowerCase().endsWith('.png')) {
            files.push(item);
        } else if (fs.existsSync(item) && fs.statSync(item).isDirectory()) {
            const pattern = recursive ? `${item}/**/*.png` : `${item}/*.png`;
            const matches = glob.sync(pattern, { nodir: true });
            files.push(...matches);
        } else if (item.includes('*')) {
            const matches = glob.sync(item);
            for (const m of matches) {
                if (m.toLowerCase().endsWith('.png')) {
                    files.push(m);
                }
            }
        }
    }
    return files;
}

function processFiles(inputs, quality, resizeW, resizeH, outputDir, overwrite, recursive) {
    const files = findPngFiles(inputs, recursive);
    if (files.length === 0) {
        console.log('Не найдено PNG-файлов.');
        return;
    }
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    const total = files.length;
    console.log(`Найдено ${total} PNG-файлов.`);
    (async () => {
        for (let i=0; i<total; i++) {
            const inputFile = files[i];
            const outName = path.basename(inputFile, '.png') + '.jpg';
            const outPath = path.join(outputDir, outName);
            if (fs.existsSync(outPath) && !overwrite) {
                console.log(`[${i+1}/${total}] ${outPath} уже существует, пропуск.`);
                continue;
            }
            console.log(`[${i+1}/${total}] Конвертация ${inputFile} -> ${outPath}`);
            try {
                await convertPngToJpg(inputFile, outPath, quality, resizeW, resizeH);
            } catch (err) {
                console.error(`  Ошибка при конвертации ${inputFile}: ${err.message}`);
            }
        }
        console.log('Готово!');
    })();
}

function main() {
    const args = process.argv.slice(2);
    if (args.length === 0) {
        console.log('Использование: node convert.js <PNG-файлы/папки> [--quality N] [--resize ШxВ] [--output DIR] [--overwrite] [--recursive]');
        return;
    }
    let quality = 85;
    let resizeW = 0, resizeH = 0;
    let outputDir = '.';
    let overwrite = false;
    let recursive = false;
    const inputs = [];
    for (let i=0; i<args.length; i++) {
        switch (args[i]) {
            case '--quality':
                if (i+1 < args.length) quality = parseInt(args[++i]);
                break;
            case '--resize':
                if (i+1 < args.length) {
                    const s = args[++i];
                    const parts = s.split('x');
                    if (parts.length === 2) {
                        resizeW = parseInt(parts[0]);
                        resizeH = parseInt(parts[1]);
                    }
                }
                break;
            case '--output':
                if (i+1 < args.length) outputDir = args[++i];
                break;
            case '--overwrite':
                overwrite = true;
                break;
            case '--recursive':
                recursive = true;
                break;
            default:
                inputs.push(args[i]);
        }
    }
    processFiles(inputs, quality, resizeW, resizeH, outputDir, overwrite, recursive);
}

main();
