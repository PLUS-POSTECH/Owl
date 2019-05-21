import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';
import { prisma, User } from './generated/prisma-client'

const App = () => {
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

    return () => { canceled = true; };
  }, []);

  if (isLoading) {
    return <p>Loading...</p>;
  } else {
    return <div>
      <h1>User List</h1>
      <ul>{
        userList.map((user) => <li key={user.id}>{user.name} ({user.id})</li>)
      }</ul>
    </div>;
  }
};

ReactDOM.render(<App />, document.getElementById('root'));
