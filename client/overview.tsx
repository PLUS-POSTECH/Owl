import React, { useState, useEffect } from "react";
import {
  isWithinRange,
  differenceInSeconds,
  addSeconds,
  format,
  distanceInWordsToNow
} from "date-fns";
import { Header, Container, Table, Icon } from "semantic-ui-react";
import { Line } from "react-chartjs-2";

import { prisma, Team, TaskStatus } from "../generated/prisma-client";
import { Loader, useAwait } from "./common";

export const OverviewPath = "/overview";

interface AttackBoardProps {
  startTime: Date;
  endTime: Date;
}

const AttackBoard: React.FC<AttackBoardProps> = ({ startTime, endTime }) => {
  const fetchTasks = async () => {
    const teamList: Team[] = await prisma.teams({
      orderBy: "id_ASC"
    });

    const serviceList: Service[] = await prisma.services({
      orderBy: "createdAt_DESC",
      where: {
        enabled: true
      }
    });

    interface TaskDetail {
      id: string;
      exploit: {
        id: string;
      };
      endpoint: {
        team: {
          id: string;
        };
        service: {
          id: string;
          enabled: boolean;
        };
      };
      status: TaskStatus;
    }

    const taskMap: {
      [teamId: string]: { [serviceId: string]: TaskDetail[] };
    } = {};

    for (const team of teamList) {
      taskMap[team.id] = {};
      for (const service of serviceList) {
        taskMap[team.id][service.id] = [];
      }
    }

    const tasks: TaskDetail[] = await prisma.tasks({
      where: {
        AND: {
          lastUpdate_gte: startTime,
          lastUpdate_lte: endTime
        }
      }
    }).$fragment(`
      fragment TaskDetail on Task {
        id
        exploit {
          id
        }
        endpoint {
          team {
            id
          }
          service {
            id
            enabled
          }
        }
        status
        flag
      }
    `);

    for (const task of tasks) {
      if (task.endpoint.service.enabled) {
        taskMap[task.endpoint.team.id][task.endpoint.service.id].push(task);
      }
    }

    const cellContent = (teamId: string, serviceId: string) => {
      let taskArray = taskMap[teamId][serviceId];
      if (taskArray.length == 0) {
        return "-";
      }
      if (taskArray.some(task => task.status == "SUBMIT_CORRECT")) {
        return <Icon name="checkmark" color="green" />;
      }
      if (
        taskArray.some(
          task =>
            task.status == "EXPLOITING" ||
            task.status == "PENDING" ||
            task.status == "SUBMITTING"
        )
      ) {
        return <Icon loading name="spinner" />;
      }
      return <Icon name="x" color="red" />;
    };

    return (
      <>
        <Table celled basic="very">
          <Table.Header>
            <Table.Row>
              <Table.HeaderCell />
              {serviceList.map(service => (
                <Table.HeaderCell key={service.id} textAlign="center">
                  {service.name}
                </Table.HeaderCell>
              ))}
            </Table.Row>
          </Table.Header>

          <Table.Body>
            {teamList.map(team => (
              <Table.Row key={team.id}>
                <Table.Cell textAlign="right">{team.name}</Table.Cell>
                {serviceList.map(service => {
                  return (
                    <Table.Cell key={service.id} selectable textAlign="center">
                      {cellContent(team.id, service.id)}
                    </Table.Cell>
                  );
                })}
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      </>
    );
  };

  const status = useAwait(fetchTasks);
  return <Loader status={status} render={component => component} />;
};

const RoundDisplay: React.FC = () => {
  const status = useAwait(
    async () =>
      await prisma.days({
        orderBy: "endTime_ASC"
      })
  );
  const [currentDate, setCurrentDate] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => setCurrentDate(new Date()), 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Loader
      status={status}
      render={dayList => {
        let displayOption = {
          addSuffix: true,
          includeSeconds: true
        };

        // CTF is running
        for (let idx = 0; idx < dayList.length; idx++) {
          const today = dayList[idx];

          const startTimeDate = new Date(today.startTime);
          const endTimeDate = new Date(today.endTime);

          if (isWithinRange(currentDate, startTimeDate, endTimeDate)) {
            const roundNumber =
              Math.floor(
                differenceInSeconds(currentDate, startTimeDate) /
                  today.roundDurationInSeconds
              ) + 1;

            let roundStart = addSeconds(
              startTimeDate,
              today.roundDurationInSeconds * (roundNumber - 1)
            );
            let roundEnd = addSeconds(roundStart, today.roundDurationInSeconds);

            return (
              <>
                <Header as="h1" textAlign="center">
                  {today.name} - Round {roundNumber}
                  <Header.Subheader>
                    {format(roundStart, "HH:mm:ss")} ~{" "}
                    {format(roundEnd, "HH:mm:ss")} (
                    {distanceInWordsToNow(roundEnd, displayOption)})
                  </Header.Subheader>
                </Header>
                <AttackBoard startTime={roundStart} endTime={roundEnd} />
              </>
            );
          }
        }

        // Preparation time between days
        let lastEndTime = null;
        for (let idx = 0; idx < dayList.length; idx++) {
          const tomorrow = dayList[idx];

          const startTimeDate = new Date(tomorrow.startTime);
          const endTimeDate = new Date(tomorrow.endTime);

          if (
            (lastEndTime === null || lastEndTime <= currentDate) &&
            currentDate < startTimeDate
          ) {
            return (
              <Header as="h1" textAlign="center">
                {tomorrow.name} preparation
                <Header.Subheader>
                  starts at {format(startTimeDate, "HH:mm")} (
                  {distanceInWordsToNow(startTimeDate, displayOption)})
                </Header.Subheader>
              </Header>
            );
          }

          lastEndTime = endTimeDate;
        }

        return (
          <Header as="h1" textAlign="center">
            Contest has finished
          </Header>
        );
      }}
    />
  );
};

const ScoreTimeline: React.FC = () => {
  interface ScoreUpdateLogWithTeam {
    id: string;
    score: number;
    time: Date;
    team: {
      id: string;
      name: string;
    };
  }

  const fetchScoreUpdateLogs = async (): Promise<ScoreUpdateLogWithTeam[]> =>
    await prisma.scoreUpdateLogs({
      orderBy: "time_ASC"
    }).$fragment(`
      fragment ScoreUpdateLogWithTeam on ScoreUpdateLog {
        id
        score
        time
        team {
          id
          name
        }
      }`);

  const status = useAwait([
    async () => await prisma.teams(),
    fetchScoreUpdateLogs
  ]);

  return (
    <Loader
      status={status}
      render={([teams, scoreUpdateLogs]) => {
        interface TeamItem {
          team: Team;
          data: Array<{ x: Date; y: number }>;
        }

        let teamDict: { [id: string]: TeamItem } = {};
        for (const team of teams) {
          teamDict[team.id] = {
            team: team,
            data: []
          };
        }

        for (const scoreLog of scoreUpdateLogs) {
          teamDict[scoreLog.team.id].data.push({
            x: scoreLog.time,
            y: scoreLog.score
          });
        }

        let randomColorGenerator = function() {
          const red = Math.floor(Math.random() * 256);
          const green = Math.floor(Math.random() * 256);
          const blue = Math.floor(Math.random() * 256);
          return [
            `rgba(${red},${green},${blue},0.7)`,
            `rgba(${red},${green},${blue},0.3)`
          ];
        };

        let teamData = [];
        for (const teamId in teamDict) {
          const team = teamDict[teamId];
          let color = randomColorGenerator();
          teamData.push({
            label: team.team.name,
            fill: false,
            tension: 0,
            borderColor: color[0],
            backgroundColor: color[1],
            data: team.data
          });
        }

        return (
          <Line
            data={{
              datasets: teamData
            }}
            options={{
              scales: {
                xAxes: [
                  {
                    type: "time"
                  }
                ]
              }
            }}
          />
        );
      }}
    />
  );
};

export const Overview: React.FC = () => (
  <div
    style={{
      marginLeft: 40,
      marginRight: 40
    }}
  >
    <RoundDisplay />
    <ScoreTimeline />
  </div>
);
