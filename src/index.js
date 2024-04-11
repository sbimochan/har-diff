import inquirer from 'inquirer'
import { exec } from 'child_process'

import { makeFolder } from './folder.js';
import { processDiff } from './process.js';

const questions = [
  {
    type: 'input',
    name: 'sourceFile',
    message: 'Enter source file name:',
  },
  {
    type: 'input',
    name: 'targetFile',
    message: 'Enter target file name:',
  },
  {
    type: 'input',
    name: 'baseUrl',
    message: 'Enter base URL:'
  }
];

// Prompt the user for source and target file names
inquirer.prompt(questions)
  .then(answers => {
    const { sourceFile, targetFile, baseUrl } = answers;
    const sourceFolder = sourceFile.replace('.har', '');
    const targetFolder = targetFile.replace('.har', '');

    makeFolder(sourceFolder)
    makeFolder(targetFolder)

    processDiff(sourceFile, sourceFolder, baseUrl)
    processDiff(targetFile, targetFolder, baseUrl)

  })
  .catch(error => {
    console.error('Error occurred:', error);
    process.exit(1);
  });
