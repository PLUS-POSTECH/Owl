import React, { useEffect, useState } from "react";
import {
  Header,
  List,
  Loader,
} from "semantic-ui-react"
import { prisma, Team } from "./generated/prisma-client"

const Team = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [teamList, setTeamList] = useState<Team[]>([]);

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

    return () => { canceled = true; };
  }, []);

  if (isLoading) {
    return <Loader active inline="centered" />;
  } else {
    return <>
      <Header as="h1">Team List ({teamList.length} teams)</Header>
      <List divided relaxed size="large">
        {teamList.map((team) => <List.Item key={team.id}>{team.name} ({team.score})</List.Item>)}
      </List>
    </>;
  }
};

export = Team;
