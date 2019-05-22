import React, { useState } from "react";
import { Container, Menu, Icon } from "semantic-ui-react";
import ReactDOM from "react-dom";
import {
  BrowserRouter as Router,
  Route,
  Link,
  Redirect,
  withRouter
} from "react-router-dom";

import Team from "./team";
import User from "./user";

const panes = [
  { text: "Teams", url: "/teams", component: Team },
  { text: "Users", url: "/users", component: User }
];

const App = () => {
  const menu = panes.map(pane => {
    const Component = withRouter(props => (
      <Menu.Item
        as={Link}
        active={props.location.pathname.startsWith(pane.url)}
        to={pane.url}
      >
        {pane.text}
      </Menu.Item>
    ));

    return <Component key={pane.text} />;
  });

  const content = panes.map(pane => (
    <Route key={pane.text} path={pane.url} component={pane.component} />
  ));

  return (
    <Router>
      <div>
        <Menu fixed="top" inverted>
          <Container>
            <Menu.Item header>
              <Icon.Group
                size="big"
                style={{ marginRight: "0.5em", textAlign: "center" }}
              >
                <Icon color="yellow" size="large" name="circle" />
                <Icon
                  color="black"
                  size="large"
                  name="circle"
                  style={{ marginLeft: "0.1em" }}
                />
                <Icon
                  circular
                  name="user secret"
                  style={{ marginLeft: "0.2em" }}
                />
              </Icon.Group>
              Owl
            </Menu.Item>
            {menu}
          </Container>
        </Menu>

        <Container text style={{ paddingTop: "7em" }}>
          <Route
            path="/"
            exact
            component={() => <Redirect to={panes[0].url} />}
          />
          {content}
        </Container>
      </div>
    </Router>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
