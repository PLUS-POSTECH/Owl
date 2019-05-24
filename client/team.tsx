import React, { useEffect, useState } from "react";
import { Header, Menu } from "semantic-ui-react";
import { prisma, Team as TeamObj } from "./generated/prisma-client";

import { Loader } from "./common";

export const Team: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [teamList, setTeamList] = useState<TeamObj[]>([]);

  useEffect(() => {
    let canceled = false;

    const fetchData = async () => {
      const teams = await prisma.teams({
        orderBy: "score_DESC"
      });
      if (!canceled) {
        setIsLoading(false);
        setTeamList(teams);
      }
    };

    fetchData();

    return () => {
      canceled = true;
    };
  }, []);

  return (
    <Loader isLoading={isLoading}>
      <Header as="h1">Team List ({teamList.length} teams)</Header>
      <Menu size="large" fluid vertical>
        {teamList.map(team => (
          <Menu.Item key={team.id}>
            {team.name} ({team.score})
          </Menu.Item>
        ))}
      </Menu>
    </Loader>
  );
};
