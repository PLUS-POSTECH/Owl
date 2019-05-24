import React, { useEffect, useState } from "react";
import { Header, Menu } from "semantic-ui-react";
import { prisma, User as UserObj } from "./generated/prisma-client";

import { Loader } from "./common";

export const User: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [userList, setUserList] = useState<UserObj[]>([]);

  useEffect(() => {
    let canceled = false;

    const fetchData = async () => {
      const users = await prisma.users();
      if (!canceled) {
        setIsLoading(false);
        setUserList(users);
      }
    };

    fetchData();

    return () => {
      canceled = true;
    };
  }, []);

  return (
    <Loader isLoading={isLoading}>
      <Header as="h1">User List ({userList.length} users)</Header>
      <Menu size="large" fluid vertical>
        {userList.map(user => (
          <Menu.Item key={user.id}>
            {user.name} ({user.id})
          </Menu.Item>
        ))}
      </Menu>
    </Loader>
  );
};
