import { execSync } from 'child_process';

export async function processMock(file, folder, url) {
  try {
    const mockGenerator = `npx har-to-mocks ./${file} ./${folder} --url=${url}`;
    await execSync(mockGenerator);
    console.log('Mocks generated. Sorting all json files');
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
  execSync(`git commit -m "Temp commit of source and target folder ${Date.now()})}"`);
  execSync(`cp -R source/* target/`);
}
