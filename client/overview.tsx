import React, { useState, useEffect } from "react";
import moment from "moment";
import { Header, Container, List } from "semantic-ui-react";

import { prisma } from "./generated/prisma-client";
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
        <Container text>
          <Header as="h1">Scoreboard</Header>
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

export const Overview: React.FC = () => (
  <>
    <RoundDisplay />
    <Scoreboard />
  </>
);
