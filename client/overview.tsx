import React, { useState, useEffect } from "react";
import moment from "moment";
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
  const [currentMoment, setCurrentMoment] = useState(moment());

  useEffect(() => {
    const interval = setInterval(() => setCurrentMoment(moment()), 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Loader
      status={status}
      render={dayList => {
        for (let idx = 0; idx < dayList.length; idx++) {
          const today = dayList[idx];
          const startMoment = moment(today.startTime);
          const endMoment = moment(today.endTime);

          if (currentMoment.isBetween(startMoment, endMoment)) {
            const roundNumber =
              Math.floor(
                currentMoment.diff(startMoment) /
                  1000 /
                  today.roundDurationInSeconds
              ) + 1;

            let roundStartMoment = moment(startMoment);
            roundStartMoment.add(
              today.roundDurationInSeconds * (roundNumber - 1),
              "seconds"
            );
            const roundEndMoment = moment(roundStartMoment);
            roundEndMoment.add(today.roundDurationInSeconds, "seconds");

            const duration = moment.duration(
              roundEndMoment.diff(currentMoment)
            );

            return (
              <Header as="h1" textAlign="center">
                {today.name} - Round {roundNumber}
                <Header.Subheader>
                  {roundStartMoment.format("HH:mm:ss")} ~{" "}
                  {roundEndMoment.format("HH:mm:ss")} ({duration.humanize(true)}
                  )
                </Header.Subheader>
              </Header>
            );
          }
        }

        let lastEndTime = null;
        for (let idx = 0; idx < dayList.length; idx++) {
          const tomorrow = dayList[idx];
          const startMoment = moment(tomorrow.startTime);
          const endMoment = moment(tomorrow.endTime);

          if (
            (lastEndTime === null || lastEndTime <= currentMoment) &&
            currentMoment < startMoment
          ) {
            const duration = moment.duration(startMoment.diff(currentMoment));
            return (
              <Header as="h1" textAlign="center">
                {tomorrow.name} preparation
                <Header.Subheader>
                  starts at {startMoment.format("HH:mm")} (
                  {duration.humanize(true)})
                </Header.Subheader>
              </Header>
            );
          }

          lastEndTime = endMoment;
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
    async () =>
      await prisma.days({
        orderBy: "endTime_ASC"
      }),
    async () => await prisma.teams(),
    fetchScoreUpdateLogs
  ]);

  return (
    <Loader
      status={status}
      render={([dayList, teams, scoreUpdateLogs]) => {
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
          console.log(scoreLog.time);
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
            tension: 0.07,
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
