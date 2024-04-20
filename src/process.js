import { execSync } from 'child_process';

import { crushDuplicateFiles } from './check-duplicates.js';

export async function processMock(file, folder, url) {
  try {
    const mockGenerator = `~/projects/har-to-mocks/bin/run ./${file} ./${folder} --url=${url} --uniqueFiles`;
    await execSync(mockGenerator);
    console.log('Mocks generated. Sorting all json files');
    await crushDuplicateFiles(folder);
    await execSync(`npx json-sort-cli ${folder}*`);
    console.log('Sorting files completed');
  } catch (error) {
    console.error(`Error executing npm script: ${error.message}`);
    throw error;
  }
}

export function processDiff() {
  execSync(`git add source target`);
  console.log('Stage source and target files');
  execSync(`git commit -m "Temp commit of source and target folder ${Date.now()}"`);
  execSync(`cp -R source/* target/`);
}
