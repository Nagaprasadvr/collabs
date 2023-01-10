const path = require("path");
const programDir = path.join(
  __dirname,
  "..",
  "collabs/programs/collabs"
);
const idlDir = path.join(__dirname, "collabs/target/idl");
const sdkDir = path.join(__dirname, "collabs/js/src", "generated");
const binaryInstallDir = path.join(__dirname, "collabs/.crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "collabs",
  programId: "4HYr7M3ytiSoqr3Zh3iK1VcNNm7ZgrNikwmWYJdGMvw4",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
  anchorRemainingAccounts: false,
};