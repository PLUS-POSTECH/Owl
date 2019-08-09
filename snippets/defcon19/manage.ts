import { prisma } from "../../generated/prisma-client";
import child_process, { ExecSyncOptions } from "child_process";
import { addDays } from "date-fns";
import yargs from "yargs";

interface Team {
  name: string;
  host: string;
}

const teamNames = [
  "A*0*E",
  "CGC",
  "HITCON⚔BFKinesiS",
  "hxp",
  "KaisHack GoN",
  "mhackeroni",
  "Plaid Parliament of Pwning",
  "r00timentary",
  "r3kapig",
  "saarsec",
  "Samurai",
  "Sauercloud",
  "SeoulPlusBadAss",
  "Shellphish",
  "Tea Deliverers",
  "TokyoWesterns"
];

const teamList: Team[] = teamNames.map((teamName, i) => {
  return {
    name: teamName,
    host: `10.13.37.${i+1}`
  };
});

yargs
  .scriptName("manage.ts")
  .command("init", "initialize the database", yargs => yargs, init)
  .command("clear", "clear all data", yargs => yargs, clear)
  .command(
    "add_service <service_name> <port>",
    "add a service",
    yargs =>
      yargs
        .positional("service_name", {
          describe: "name of new service",
          type: "string"
        })
        .positional("port", {
          describe: "port number of new service",
          type: "number"
        }),
    add_service
  )
  .demandCommand()
  .recommendCommands()
  .strict()
  .version(false)
  .help().argv;

async function init() {
  for (const teamName of teamNames) {
    await prisma.createTeam({
      name: teamName
    });
  }

  const fiveMinutesinInSeconds = 5 * 60;

  let firstDayStartTime = new Date(2019, 7, 9, 10, 0, 0, 0);
  let firstDayEndTime = new Date(2019, 7, 9, 17, 0, 0, 0);

  await prisma.createDay({
    name: "Day 1",
    startTime: firstDayStartTime,
    endTime: firstDayEndTime,
    roundDurationInSeconds: fiveMinutesinInSeconds
  });

  await prisma.createDay({
    name: "Day 2",
    startTime: addDays(firstDayStartTime, 1),
    endTime: addDays(firstDayEndTime, 1),
    roundDurationInSeconds: fiveMinutesinInSeconds
  });

  await prisma.createDay({
    name: "Day 3",
    startTime: addDays(firstDayStartTime, 2),
    endTime: addDays(firstDayEndTime, 2),
    roundDurationInSeconds: fiveMinutesinInSeconds
  });
}

async function clear() {
  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

  const INHERIT_STDIO: ExecSyncOptions = {
    stdio: "inherit"
  };

  // reset database
  child_process.execSync("docker-compose down", INHERIT_STDIO);
  child_process.execSync("docker volume rm -f owl_postgres", INHERIT_STDIO);
  child_process.execSync("docker-compose up -d", INHERIT_STDIO);

  await sleep(3000);
  child_process.execSync("prisma deploy", INHERIT_STDIO);
}

async function add_service(argv: any) {
  let service = await prisma.createService({
    name: argv.service_name
  });

  for (const team of teamList) {
    await prisma.createEndpoint({
      team: {
        connect: {
          name: team.name
        }
      },
      service: {
        connect: {
          id: service.id
        }
      },
      connectionString: `${team.host}:${argv.port}`
    });
  }
}
