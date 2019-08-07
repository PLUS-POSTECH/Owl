import { prisma, TaskStatus } from "../generated/prisma-client";
import child_process, { ExecSyncOptions } from "child_process";
import {
  subDays,
  addDays,
  addHours,
  addMilliseconds,
  differenceInMilliseconds
} from "date-fns";
import slugify from "slugify";

const NUM_SCORE_UPDATES = 30;
const NUM_TASK_PER_ENDPOINT = 50;

const teams = ["PLUS", "KoreanBadass", "r00timentary", "PPP"];

const startHour = 10; // 10 AM in the morning
const endHour = 28; // 4 AM in the night
const roundDuration = 10 * 60; // 10 minutes

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const setupDays = async () => {
  const totalDays = 4;
  const todayIndex = 2;

  let firstDayStart = subDays(
    new Date().setHours(startHour, 0, 0, 0),
    todayIndex - 1
  );

  for (let i = 0; i < totalDays; i++) {
    let startTime = addDays(firstDayStart, i);
    let endTime = addHours(startTime, endHour - startHour);

    await prisma.createDay({
      name: `Day ${i + 1}`,
      startTime: startTime,
      endTime: endTime,
      roundDurationInSeconds: roundDuration
    });
  }
};

const setupTeams = async () => {
  for (const teamName of teams) {
    await prisma.createTeam({
      name: teamName
    });
  }
};

const setupScores = async () => {
  const startDate = new Date();
  startDate.setHours(startHour, 0, 0, 0);

  const endDate = new Date();
  endDate.setHours(endHour, 0, 0, 0);

  for (let i = 0; i < NUM_SCORE_UPDATES; i++) {
    let teamIndex = Math.floor(Math.random() * teams.length);
    await prisma.createScoreUpdateLog({
      team: {
        connect: {
          name: teams[teamIndex]
        }
      },
      score: Math.floor(Math.random() * 1000),
      time: addMilliseconds(
        startDate,
        Math.random() * differenceInMilliseconds(endDate, startDate)
      )
    });
  }
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
};

const setupTasks = async () => {
  const teams = await prisma.teams();
  const services = await prisma.services();

  const startDate = new Date();
  startDate.setHours(startHour, 0, 0, 0);

  const endDate = new Date();
  endDate.setHours(endHour, 0, 0, 0);

  const exploit = await prisma.createExploit({
    target: {
      connect: {
        id: services[0].id
      }
    },
    name: "test_exploit"
  });

  const statusCandidates: TaskStatus[] = [
    "SUBMITTING",
    "SUBMIT_CORRECT",
    "SUBMIT_WRONG",
    "EXPLOIT_ERROR"
  ];

  let promises = [];
  for (const team of teams) {
    for (const service of services) {
      for (let i = 0; i < NUM_TASK_PER_ENDPOINT; i++) {
        const endpointArray = await prisma.endpoints({
          where: {
            team: {
              id: team.id
            },
            service: {
              id: service.id
            }
          }
        });

        if (endpointArray.length == 0) {
          continue;
        }

        const endpoint = endpointArray[0];
        const randomDate = addMilliseconds(
          startDate,
          Math.random() * differenceInMilliseconds(endDate, startDate)
        );
        const randomStatus: TaskStatus =
          statusCandidates[Math.floor(Math.random() * statusCandidates.length)];

        promises.push(
          prisma.createTask({
            exploit: {
              connect: {
                id: exploit.id
              }
            },
            endpoint: {
              connect: {
                id: endpoint.id
              }
            },
            lastUpdate: randomDate,
            status: randomStatus
          })
        );
      }
    }
  }

  await Promise.all(promises).catch(error => console.log(error));
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
  await setupScores();
  await setupServices();
  await setupEndpoints();
  await setupTasks();
}

main();
