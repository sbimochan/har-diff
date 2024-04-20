import fs from 'fs';
import path from 'path';
import crypto from 'crypto';

function calculateChecksum(filePath) {
  return new Promise((resolve, reject) => {
    const hash = crypto.createHash('sha256');
    const stream = fs.createReadStream(filePath);

    stream.on('error', reject);

    stream.on('data', (data) => {
      hash.update(data);
    });

    stream.on('end', () => {
      const checksum = hash.digest('hex');
      resolve(checksum);
    });
  });
}

async function findDuplicateFiles(folder) {
  const checksums = {};
  const duplicates = [];

  async function processFiles(dir) {
    const files = await fs.promises.readdir(dir);

    for (const file of files) {
      const filePath = path.join(dir, file);
      const stats = await fs.promises.stat(filePath);

      if (stats.isDirectory()) {
        await processFiles(filePath); // Recursively process subdirectories
      } else {
        const checksum = await calculateChecksum(filePath);
        if (checksum in checksums) {
          duplicates.push(filePath); // Add duplicate file path to duplicates array
        } else {
          checksums[checksum] = filePath;
        }
      }
    }
  }

  await processFiles(folder);
  return duplicates;
}

async function deleteDuplicateFiles(folder) {
  const duplicates = await findDuplicateFiles(folder);

  for (const duplicate of duplicates) {
    try {
      await fs.promises.unlink(duplicate); // Delete duplicate file
      console.log(`Deleted duplicate file: ${duplicate}`);
    } catch (error) {
      console.error(`Error deleting file ${duplicate}:`, error);
    }
  }
}

export async function crushDuplicateFiles(folderToSearch) {
  deleteDuplicateFiles(folderToSearch)
    .then(() => {
      console.log('Duplicate files deleted successfully.');
    })
    .catch((error) => {
      console.error('Error:', error);
    });
}
