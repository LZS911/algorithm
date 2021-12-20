import fs = require('fs-extra');
import path = require('path');
import prettier = require('prettier');
import robot = require('robotjs');

let launchJson = {
  version: '0.2.0',
  configurations: [
    {
      name: 'Current TS File',
      type: 'node',
      request: 'launch',
      runtimeArgs: ['-r', 'ts-node/register'],
      program: '${workspaceRoot}/src/demo/index.ts',
      args: ['${relativeFile}'],
      cwd: '${workspaceRoot}',
      protocol: 'inspector'
    }
  ]
};

const Write = () => {
  const pathSeparator = path.sep;

  const makeDir = (url: string) => {
    const urlArray = url.split(pathSeparator);
    let hasDir = 0;
    for (let i = urlArray.length; i > 0; i--) {
      const currentUrl = urlArray.slice(0, i);
      const has = fs.existsSync(currentUrl.join(pathSeparator));
      if (has) {
        hasDir = i + 1;
        break;
      }
    }
    for (let i = hasDir; i <= urlArray.length; i++) {
      fs.mkdirSync(urlArray.slice(0, i).join(pathSeparator));
    }
  };

  const writeFile = (src: string, content: string) => {
    try {
      makeDir(path.dirname(src));
      const file = prettier.format(content, {
        parser: 'json',
        tabWidth: 2,
        semi: true,
        printWidth: 80,
        trailingComma: 'none',
        arrowParens: 'avoid',
        proseWrap: 'preserve',
        useTabs: false,
        singleQuote: true,
        bracketSpacing: true,
        jsxBracketSameLine: false
      });
      fs.writeFileSync(src, file);
    } catch (error) {
      !!error && console.log(error);
    }
  };

  return { writeFile };
};

const replaceDebugPath = (source: string) => {
  return source.replace(/\/+.+/, _ => `/src/${debugPath}/index.ts`);
};
const debugConfigPath = path.resolve(__dirname, '../.vscode/launch.json');
const { writeFile } = Write();

const args = process.argv.slice(2);
if (args.length === 0) {
  if (!fs.existsSync(debugConfigPath)) {
    console.log('缺少调试对象参数...');
    process.exit(1);
  }
  robot.keyTap('d', ['command', 'shift']);
  robot.keyTap('f5');
  process.exit(0);
}

const [debugPath] = args;

if (!fs.existsSync(debugConfigPath)) {
  launchJson.configurations[0].program = replaceDebugPath(launchJson.configurations[0].program);
  writeFile(debugConfigPath, JSON.stringify(launchJson));
} else {
  try {
    launchJson = fs.readJSONSync(debugConfigPath);
    if (!launchJson?.configurations[0]?.args) {
      console.error('当前调试配置选项错误, 未找到 program项...');
      process.exit(1);
    }
  } catch (error) {
    console.error(error);
    process.exit(1);
  }

  launchJson.configurations[0].program = replaceDebugPath(launchJson.configurations[0].program);
  writeFile(debugConfigPath, JSON.stringify(launchJson));
}

robot.keyTap('d', ['command', 'shift']);
robot.keyTap('f5');
