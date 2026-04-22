const fs = require('fs');
const path = require('path');

function walk(dir) {
    let results = [];
    const list = fs.readdirSync(dir);
    list.forEach(file => {
        file = path.resolve(dir, file);
        const stat = fs.statSync(file);
        if (stat && stat.isDirectory()) {
            results = results.concat(walk(file));
        } else {
            if (file.endsWith('.html')) results.push(file);
        }
    });
    return results;
}

const files = walk('.');
const fabString = `  <a id="fab" class="fab-modern" href="auth.html">
    <svg width="20" height="20" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
      <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
    </svg>
    <span>Get Started</span>
  </a>

</body>`;

files.forEach(file => {
    let content = fs.readFileSync(file, 'utf8');
    
    // First, clean up any previous failed attempts
    content = content.replace(/<a id="fab" class="fab-modern"[\s\S]*?<\/a>[\s\S]*?(?=<\/body>)/gi, '');
    content = content.replace(/`n/g, ''); // Remove the literal backtick n artifacts
    
    // Clean up excessive whitespace before body
    content = content.replace(/\s+<\/body>/g, '\n\n</body>');
    
    // Insert correctly
    if (!content.includes('id="fab"')) {
        content = content.replace('</body>', fabString);
    }
    
    // Version bump
    content = content.replace(/\.js\?v=\d+/g, '.js?v=4');
    content = content.replace(/\.css\?v=\d+/g, '.css?v=4');
    
    fs.writeFileSync(file, content);
});

console.log('Cleanup complete.');
