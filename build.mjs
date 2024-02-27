
import yargs from 'yargs/yargs';
import { hideBin } from 'yargs/helpers';
import { execSync } from 'child_process';
import * as fs from 'fs';

const addition = `
export type TagVec = string[];
`;

const argv = yargs(hideBin(process.argv)).argv;

async function build(args) {
  const flags = ['--platform', '--dts', 'index.d.ts', '--js', 'index.js'];
  if (args.release) {
    flags.push('--release');
  }
  if (args.target) {
    flags.push('--target');
    flags.push(args.target);
  }
  
  const command = `npx napi build ${flags.join(' ')}`;
  
  try {
    execSync(command, { stdio: 'inherit' });
    await fs.promises.appendFile('index.d.ts', addition, { encoding: 'utf-8' });
    console.log("build.mjs: Modified generated typings");
  } catch (error) {
    console.error('Build failed:', error);
    process.exit(1);
  }
}

build(argv);
