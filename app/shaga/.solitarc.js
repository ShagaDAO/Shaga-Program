const path = require('path');
const programDir = path.join(__dirname, '..', 'programs/shagajoe');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'shaga',
  programId: 'HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
