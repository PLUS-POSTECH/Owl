import React, { useEffect, useState } from "react";
import {
  Container,
  Header,
  List,
  Loader,
  Menu,
  Icon,
} from "semantic-ui-react"
import ReactDOM from "react-dom";
import { prisma, Team } from "./generated/prisma-client"

const App = () => {
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

  return <div>
    <Menu fixed="top" inverted>
      <Container>
        <Menu.Item header>
          <Icon.Group size="big" style={{ marginRight: "0.5em", textAlign: "center" }}>
            <Icon color="yellow" size="large" name="circle"></Icon>
            <Icon color="black" size="large" name="circle" style={{ marginLeft: "0.1em"}}></Icon>
            <Icon circular name="user secret" style={{ marginLeft: "0.2em"}}></Icon>
          </Icon.Group>
          Owl
        </Menu.Item>
        <Menu.Item active as="a">Teams</Menu.Item>
      </Container>
    </Menu>

    <Container text style={{ paddingTop: "7em" }}>
      {isLoading ? <Loader active inline="centered" />
      : <>
        <Header as="h1">Team List ({teamList.length} teams)</Header>
        <List divided relaxed size="large">
          {teamList.map((team) => <List.Item key={team.id}>{team.name} ({team.score})</List.Item>)}
        </List>
      </>}
    </Container>
  </div>;
};

ReactDOM.render(<App />, document.getElementById("root"));
