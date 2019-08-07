import React, { useState, useEffect } from "react";
import {
  isWithinRange,
  differenceInSeconds,
  addSeconds,
  format,
  distanceInWordsToNow
} from "date-fns";
import { Header, Container } from "semantic-ui-react";
import { Line } from "react-chartjs-2";

import { prisma, Team } from "../generated/prisma-client";
import { Loader, useAwait } from "./common";

export const OverviewPath = "/overview";

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
              <Header as="h1" textAlign="center">
                {today.name} - Round {roundNumber}
                <Header.Subheader>
                  {format(roundStart, "HH:mm:ss")} ~{" "}
                  {format(roundEnd, "HH:mm:ss")} (
                  {distanceInWordsToNow(roundEnd, displayOption)})
                </Header.Subheader>
              </Header>
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
          <Container>
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
          </Container>
        );
      }}
    />
  );
};

export const Overview: React.FC = () => (
  <>
    <RoundDisplay />
    <ScoreTimeline />
  </>
);
