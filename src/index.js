import inquirer from 'inquirer';

import { makeFolder } from './folder.js';
import { processDiff, processMock } from './process.js';

const sourceFile = 'source.har';
const sourceFolder = 'source';
const targetFile = 'target.har';
const targetFolder = 'target';

const questions = [
  {
    type: 'input',
    name: 'baseUrl',
    message: 'Enter base URL:',
    default: 'https://api.dev.laudio.io'
  },
];

async function main() {
  inquirer
    .prompt(questions)
    .then(async (answers) => {
      const { baseUrl } = answers;
      await makeFolder(sourceFolder);
      await makeFolder(targetFolder);
      await genDiff(baseUrl);
    })
    .catch((error) => {
      console.error('Error occurred:', error);
      process.exit(1);
    });
}


async function genDiff(baseUrl) {
  await processMock(sourceFile, sourceFolder, baseUrl);
  await processMock(targetFile, targetFolder, baseUrl);
  await processDiff();
}

main()
