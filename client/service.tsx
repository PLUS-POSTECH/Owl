import React, { useEffect, useState } from "react";
import { Header, List } from "semantic-ui-react";
import { prisma, Service as ServiceObj } from "./generated/prisma-client";

import Loader from "./loader";

export const Service: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [serviceList, setServiceList] = useState<ServiceObj[]>([]);

  useEffect(() => {
    let canceled = false;

    const fetchData = async () => {
      const services = await prisma.services({
        orderBy: "createdAt_DESC"
      });
      if (!canceled) {
        setIsLoading(false);
        setServiceList(services);
      }
    };

    fetchData();

    return () => {
      canceled = true;
    };
  }, []);

  return (
    <Loader isLoading={isLoading}>
      <Header as="h1">Service List ({serviceList.length} services)</Header>
      <List divided relaxed size="large">
        {serviceList.map(service => (
          <List.Item key={service.id}>{service.name}</List.Item>
        ))}
      </List>
    </Loader>
  );
};
