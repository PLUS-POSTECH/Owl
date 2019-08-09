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

import { AccessFailure } from "./common";
import { Overview, OverviewPath } from "./overview";
import { Service, ServicePath } from "./service";
import { ExploitPath, Exploit } from "./exploit";

const panes = [
  { text: "Overview", path: OverviewPath, component: Overview },
  { text: "Services", path: ServicePath, component: Service },
  { text: "Exploit", path: ExploitPath, component: Exploit }
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

ReactDOM.render(<App />, document.getElementById("root"));
