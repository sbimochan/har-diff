import inquirer from 'inquirer'
import { exec } from 'child_process'

import { makeFolder } from './folder.js';
import { processDiff } from './process.js';

const sourceFile = 'source.har'
const sourceFolder = 'source'
const targetFile = 'target.har'
const targetFolder = 'target'

const questions = [
  {
    type: 'input',
    name: 'baseUrl',
    message: 'Enter base URL:'
  }
];

inquirer.prompt(questions)
  .then(answers => {
    const { baseUrl } = answers;
    makeFolder(sourceFolder)
    makeFolder(targetFolder)

    processDiff(sourceFile, sourceFolder, baseUrl)
    processDiff(targetFile, targetFolder, baseUrl)

  })
  .catch(error => {
    console.error('Error occurred:', error);
    process.exit(1);
  });
