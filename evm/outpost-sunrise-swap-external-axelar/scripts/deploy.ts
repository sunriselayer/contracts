import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);

  try {
    const contractFactory = await ethers.getContractFactory(
      "SunriseSwapWithAxelarGMP"
    );
    const contract = await contractFactory.deploy(
      "0xe432150cce91c13a887f7D836923d5597adD8E31", // axelar gateway
      "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6", // axelar gas service
      "ethereum-sepolia" // sepolia chain name
    );

    console.log(
      "SunriseSwapWithAxelarGMP contract address:",
      await contract.getAddress()
    );
  } catch (error) {
    console.error("Deployment failed:", error);
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
