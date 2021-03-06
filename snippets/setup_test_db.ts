import { prisma } from "../generated/prisma-client";
import child_process, { ExecSyncOptions } from "child_process";
import slugify from "slugify";

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const setupDays = async () => {
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

const setupTeams = async () => {
  await prisma.createTeam({
    name: "PLUS",
    score: 500
  });

  await prisma.createTeam({
    name: "KoreanBadass",
    score: 200
  });

  await prisma.createTeam({
    name: "r00timentary",
    score: 400
  });

  await prisma.createTeam({
    name: "PPP",
    score: 666
  });
};

const setupServices = async () => {
  await prisma.createService({
    name: "Easy BOF",
    description: "Can you solve this easy BOF problem?",
    enabled: true
  });

  await prisma.createService({
    name: "Invisible",
    description: "This problem should not be visible",
    enabled: false
  });

  await prisma.createService({
    name: "Hard ROP",
    description: "Return-oriented-programming",
    enabled: true
  });

  await prisma.createService({
    name: "Rust Crypto",
    description: "Rust crypto problem",
    enabled: true
  });
};

const setupEndpoints = async () => {
  const teams = await prisma.teams();
  const services = await prisma.services();

  for (const team of teams) {
    for (const service of services) {
      if (Math.random() < 0.5) {
        // create endpoints
        await prisma.createEndpoint({
          team: {
            connect: {
              id: team.id
            }
          },
          service: {
            connect: {
              id: service.id
            }
          },
          connectionString: `${slugify(service.name, {
            lower: true
          })}.${slugify(team.name, { lower: true })}.defconctf.team`
        });
      }
    }
  }
};

const INHERIT_STDIO: ExecSyncOptions = {
  stdio: "inherit"
};

// reset database
child_process.execSync("docker-compose down", INHERIT_STDIO);
child_process.execSync("docker volume rm -f owl_postgres", INHERIT_STDIO);
child_process.execSync("docker-compose up -d", INHERIT_STDIO);

async function main() {
  await sleep(3000);
  child_process.execSync("prisma deploy", INHERIT_STDIO);
  await setupDays();
  await setupTeams();
  await setupServices();
  await setupEndpoints();
}

main();
