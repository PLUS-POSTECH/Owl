import React, { useState, useEffect } from "react";
import moment from "moment";
import { Header, Container, List } from "semantic-ui-react";
import { Line } from "react-chartjs-2";

import { prisma } from "../generated/prisma-client";
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

const Scoreboard: React.FC = () => {
  const status = useAwait(
    async () =>
      await prisma.teams({
        orderBy: "score_DESC"
      })
  );

  return (
    <Loader
      status={status}
      render={teamList => (
        <Container>
          <List divided relaxed ordered>
            {teamList.map(team => (
              <List.Item key={team.id}>
                {team.name} ({team.score})
              </List.Item>
            ))}
          </List>
        </Container>
      )}
    />
  );
};

const ScoreTimeline: React.FC = () => {
  const tension = 0.07;

  let randomColorGenerator = function() {
    const red = Math.floor(Math.random() * 256);
    const green = Math.floor(Math.random() * 256);
    const blue = Math.floor(Math.random() * 256);
    return [
      `rgba(${red},${green},${blue},0.7)`,
      `rgba(${red},${green},${blue},0.3)`
    ];
  };

  const colors = [];
  for (let i = 0; i < 10; i++) {
    colors.push(randomColorGenerator());
  }

  const data = {
    labels: ["17:00", "17:05", "17:10"],
    datasets: [
      {
        label: "PPP",
        fill: false,
        tension: tension,
        borderColor: colors[0][0],
        backgroundColor: colors[0][1],
        data: [100, 150, 400]
      },
      {
        label: "r00timentary",
        fill: false,
        tension: tension,
        borderColor: colors[1][0],
        backgroundColor: colors[1][1],
        data: [200, 300, 270]
      }
    ]
  };

  return (
    <Container>
      <Line data={data} />
    </Container>
  );
};

export const Overview: React.FC = () => (
  <>
    <RoundDisplay />
    <Scoreboard />
    <ScoreTimeline />
  </>
);
