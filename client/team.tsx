import React from "react";
import { Header, Menu } from "semantic-ui-react";
import { prisma } from "./generated/prisma-client";

import { Loader, useAwait } from "./common";

export const TeamPath = "/team/";

export const Team: React.FC = () => {
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
        <>
          <Header as="h1">Team List ({teamList.length} teams)</Header>
          <Menu size="large" fluid vertical>
            {teamList.map(team => (
              <Menu.Item key={team.id}>
                {team.name} ({team.score})
              </Menu.Item>
            ))}
          </Menu>
        </>
      )}
    />
  );
};
