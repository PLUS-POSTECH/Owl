import React from "react";
import ReactDOM from "react-dom";
import {
  BrowserRouter as Router,
  Route,
  Link,
  Redirect,
  withRouter,
  Switch
} from "react-router-dom";
import { Container, Menu, Icon } from "semantic-ui-react";

import { prisma } from "./generated/prisma-client";
import { AccessFailure } from "./common";
import { Overview, OverviewPath } from "./overview";
import { Service, ServicePath } from "./service";
import { Endpoint, EndpointPath } from "./endpoint";
import { User, UserPath } from "./user";

const panes = [
  { text: "Overview", path: OverviewPath, component: Overview },
  { text: "Services", path: ServicePath, component: Service },
  { text: "Endpoints", path: EndpointPath, component: Endpoint },
  { text: "Users", path: UserPath, component: User }
];

const LogoIcon = () => (
  <Icon.Group size="big" style={{ marginRight: "0.5em", textAlign: "center" }}>
    <Icon color="yellow" size="large" name="circle" />
    <Icon
      color="black"
      size="large"
      name="circle"
      style={{ marginLeft: "0.1em" }}
    />
    <Icon circular name="user secret" style={{ marginLeft: "0.2em" }} />
  </Icon.Group>
);

const App = () => {
  const menu = panes.map(pane => {
    const Component = withRouter(props => (
      <Menu.Item
        as={Link}
        active={props.location.pathname.startsWith(pane.path)}
        to={pane.path}
      >
        {pane.text}
      </Menu.Item>
    ));

    return <Component key={pane.text} />;
  });

  const content = panes.map(pane => (
    <Route key={pane.text} path={pane.path} component={pane.component} />
  ));

  return (
    <Router>
      <div>
        <Menu fixed="top" inverted>
          <Container>
            <Menu.Item header>
              <LogoIcon />
              Owl
            </Menu.Item>
            {menu}
          </Container>
        </Menu>

        <Container fluid style={{ paddingTop: "7em" }}>
          <Switch>
            <Route
              path="/"
              exact
              component={() => <Redirect to={panes[0].path} />}
            />
            {content}
            <Route component={AccessFailure} />
          </Switch>
        </Container>
      </div>
    </Router>
  );
};

// Creates a fake days data, should be removed in production
const setupTestDays = async () => {
  await prisma.deleteManyDays();

  const startHour = 10; // 10 AM in the morning
  const endHour = 28; // 4 AM in the night
  const roundDuration = 10 * 60; // 10 minutes
  const totalDays = 4;
  const todayIndex = 2;

  let firstDayStart = new Date();
  firstDayStart.setHours(startHour, 0, 0, 0);
  firstDayStart.setDate(firstDayStart.getDate() - (todayIndex - 1));

  for (let i = 0; i < totalDays; i++) {
    let startTime = new Date(firstDayStart);
    startTime.setDate(startTime.getDate() + i);

    let endTime = new Date(startTime);
    endTime.setHours(endTime.getHours() + (endHour - startHour));

    await prisma.createDay({
      name: `Day ${i + 1}`,
      startTime: startTime,
      endTime: endTime,
      roundDurationInSeconds: roundDuration
    });
  }
};

setupTestDays().then(() => {
  ReactDOM.render(<App />, document.getElementById("root"));
});
