import React, { useEffect, useState } from "react";
import { Header, List } from "semantic-ui-react";
import { prisma, User } from "./generated/prisma-client";

import Loader from "./loader";

const User: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [userList, setUserList] = useState<User[]>([]);

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
      <List divided relaxed size="large">
        {userList.map(user => (
          <List.Item key={user.id}>
            {user.name} ({user.id})
          </List.Item>
        ))}
      </List>
    </Loader>
  );
};

export = User;
