// Export MASA survey markdown to PDF via pandoc + puppeteer
const puppeteer = require('puppeteer');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const MD_FILE = path.resolve(__dirname, 'runs/2026-06-07T1011Z/07_survey.md');
const PDF_FILE = MD_FILE.replace(/\.md$/, '.pdf');

const CSS = `
@page {
  size: A4;
  margin: 2.2cm 2.5cm 2.2cm 2.5cm;
  @bottom-center {
    content: counter(page);
    font-size: 9pt;
    color: #666;
    font-family: Georgia, 'Times New Roman', serif;
  }
}
html { font-size: 10.5pt; }
body {
  font-family: Georgia, 'Times New Roman', serif;
  line-height: 1.65;
  color: #111;
  text-align: justify;
  hyphens: auto;
  orphans: 3;
  widows: 3;
}
h1.title {
  font-size: 17pt;
  text-align: center;
  margin: 0 0 0.8cm 0;
  padding-top: 1.5cm;
  line-height: 1.3;
  page-break-before: avoid;
}
h1:not(.title) {
  font-size: 13pt;
  margin: 1.2em 0 0.4em 0;
  border-bottom: 1px solid #bbb;
  padding-bottom: 3px;
  page-break-after: avoid;
}
h2 { font-size: 11.5pt; margin: 1em 0 0.3em 0; page-break-after: avoid; }
h3 { font-size: 10.5pt; margin: 0.8em 0 0.2em 0; page-break-after: avoid; }
p { margin: 0.4em 0; }
ul, ol { margin: 0.3em 0 0.3em 1.8em; }
li { margin: 0.15em 0; }
table {
  border-collapse: collapse; width: 100%; margin: 0.6em 0;
  font-size: 9pt; page-break-inside: avoid;
}
th, td {
  border: 1px solid #999; padding: 3px 5px; text-align: left; vertical-align: top;
}
th { background: #eee; font-weight: bold; }
code {
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 8.5pt; background: #f5f5f5; padding: 1px 3px;
}
pre {
  background: #f8f8f8; border: 1px solid #ddd; border-left: 3px solid #0366d6;
  padding: 0.5em 0.8em; font-size: 8pt; line-height: 1.35;
  overflow-x: auto; page-break-inside: avoid;
}
pre code { background: none; padding: 0; }
blockquote {
  margin: 0.4em 0; padding: 0.2em 0.8em;
  border-left: 3px solid #0366d6; color: #555; background: #f8f9fa;
}
a { color: #0366d6; text-decoration: none; }
img { max-width: 100%; }
`;

async function main() {
  if (!fs.existsSync(MD_FILE)) {
    console.error(`Error: ${MD_FILE} not found`);
    process.exit(1);
  }

  console.log(`Input:  ${MD_FILE}`);
  console.log(`Output: ${PDF_FILE}`);

  // Step 1: pandoc markdown → HTML
  console.log('Converting to HTML via pandoc...');
  const htmlPath = MD_FILE.replace(/\.md$/, '.html');
  execSync(
    `pandoc "${MD_FILE}" --from markdown+smart+pipe_tables+grid_tables+fenced_code_blocks+backtick_code_blocks+autolink_bare_uris+tex_math_dollars --to html5 --mathjax --wrap preserve -o "${htmlPath}"`,
    { stdio: 'pipe', encoding: 'utf-8' }
  );
  console.log('  pandoc OK');

  // Read HTML and inject CSS
  let html = fs.readFileSync(htmlPath, 'utf-8');
  html = html.replace('</head>', `<style>${CSS}</style></head>`);
  // Add title class to first h1
  html = html.replace('<h1>', '<h1 class="title">');
  fs.writeFileSync(htmlPath, html, 'utf-8');
  console.log('  CSS injected');

  // Step 2: puppeteer HTML → PDF
  console.log('Rendering PDF via puppeteer...');
  const browser = await puppeteer.launch({
    headless: 'new',
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
    userDataDir: require('os').tmpdir() + '/puppeteer_masa_' + Date.now(),
  });
  const page = await browser.newPage();
  await page.goto('file://' + htmlPath.replace(/\\/g, '/'), { waitUntil: 'networkidle0' });

  await page.pdf({
    path: PDF_FILE,
    format: 'A4',
    margin: { top: '2.2cm', bottom: '2.2cm', left: '2.5cm', right: '2.5cm' },
    printBackground: true,
    displayHeaderFooter: true,
    headerTemplate: '<span></span>',
    footerTemplate: '<div style="width:100%;text-align:center;font-size:9pt;color:#666;font-family:Georgia,serif;"><span class="pageNumber"></span></div>',
  });

  await browser.close();

  // Cleanup HTML
  try { fs.unlinkSync(htmlPath); } catch {}

  const sizeKB = (fs.statSync(PDF_FILE).size / 1024).toFixed(0);
  console.log(`\nDone! PDF: ${PDF_FILE}`);
  console.log(`Size: ${sizeKB} KB`);
}

main().catch(err => { console.error(err); process.exit(1); });
