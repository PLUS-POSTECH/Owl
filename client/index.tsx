import React, { useState } from "react";
import { Container, Menu, Icon } from "semantic-ui-react";
import ReactDOM from "react-dom";

import Team from "./team";
import User from "./user";

const panes = [
  { name: "Teams", render: () => <Team /> },
  { name: "Users", render: () => <User /> }
];

const App: React.FC = () => {
  const [active, setActive] = useState(panes[0].name);

  const menu = panes.map(pane => (
    <Menu.Item
      key={pane.name}
      as="a"
      active={pane.name == active}
      onClick={() => setActive(pane.name)}
    >
      {pane.name}
    </Menu.Item>
  ));

  const content = panes
    .filter(pane => pane.name == active)
    .map(pane => pane.render())[0];

  return (
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
        {content}
      </Container>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
