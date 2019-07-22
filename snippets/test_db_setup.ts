import { prisma } from "../generated/prisma-client";
import child_process, { ExecSyncOptions } from "child_process";

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const setupTestDays = async () => {
  await prisma.deleteManyDays();

  const startHour = 10; // 10 AM in the morning
  const endHour = 28; // 4 AM in the night
  const roundDuration = 10 * 60; // 10 minutes
  const totalDays = 4;
  const todayIndex = 2;

  let firstDayStart = new Date();
  firstDayStart.setHours(startHour, 0, 0, 0);
  firstDayStart.setDate(firstDayStart.getDate() - (todayIndex - 1));

  for (let i = 0; i < totalDays; i++) {
    let startTime = new Date(firstDayStart);
    startTime.setDate(startTime.getDate() + i);

    let endTime = new Date(startTime);
    endTime.setHours(endTime.getHours() + (endHour - startHour));

    await prisma.createDay({
      name: `Day ${i + 1}`,
      startTime: startTime,
      endTime: endTime,
      roundDurationInSeconds: roundDuration
    });
  }
};

const OPTION: ExecSyncOptions = {
  stdio: "inherit"
};

// reset database
child_process.execSync("docker-compose down", OPTION);
child_process.execSync("docker volume rm -f owl_postgres", OPTION);
child_process.execSync("docker-compose up -d", OPTION);

async function main() {
  await sleep(3000);
  child_process.execSync("prisma deploy", OPTION);
  await setupTestDays();
}

main();
