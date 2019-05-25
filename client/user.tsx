import React from "react";
import { Header, Menu } from "semantic-ui-react";
import { prisma } from "./generated/prisma-client";

import { Loader, useAwait } from "./common";

export const UserPath = "/user/";

export const User: React.FC = () => {
  const status = useAwait(async () => await prisma.users());

  return (
    <Loader
      status={status}
      render={userList => (
        <>
          <Header as="h1">User List ({userList.length} users)</Header>
          <Menu size="large" fluid vertical>
            {userList.map(user => (
              <Menu.Item key={user.id}>
                {user.name} ({user.id})
              </Menu.Item>
            ))}
          </Menu>
        </>
      )}
    />
  );
};
