import fs = require('fs-extra');
import path = require('path');

const testPath = path.join(__dirname, './.vscode/launch.json');

const json = {
  test: { a: 123 }
};
fs.writeFileSync(testPath, JSON.stringify(json));
