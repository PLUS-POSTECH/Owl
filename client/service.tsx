import React, { useEffect, useState } from "react";
import { Route, RouteChildrenProps } from "react-router";
import { Link } from "react-router-dom";
import { Header, Menu } from "semantic-ui-react";
import { prisma, Service as ServiceObj } from "./generated/prisma-client";

import { Loader } from "./common";

const ServiceList: React.FC<RouteChildrenProps> = ({ match }) => {
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
      <Menu size="large" fluid vertical>
        {serviceList.map(service => (
          <Menu.Item
            key={service.id}
            as={Link}
            to={`${match!.url}/${service.id}`}
          >
            {service.name}
          </Menu.Item>
        ))}
      </Menu>
    </Loader>
  );
};

type ServiceDetailProps = RouteChildrenProps<{ id: string }>;

const ServiceDetail: React.FC<ServiceDetailProps> = ({ match }) => {
  const [isLoading, setIsLoading] = useState(true);
  const [service, setService] = useState<ServiceObj | null>(null);

  useEffect(() => {
    let canceled = false;

    const fetchData = async () => {
      const service = await prisma.service({
        id: match!.params.id
      });
      if (!canceled) {
        setIsLoading(false);
        setService(service);
      }
    };

    fetchData();

    return () => {
      canceled = true;
    };
  }, []);

  return (
    <Loader
      isLoading={isLoading}
      isError={service === null}
      render={() => (
        <>
          <Header as="h1">{service!.name}</Header>
          <p>{service!.description}</p>
        </>
      )}
    />
  );
};

export const Service: React.FC<RouteChildrenProps> = ({ match }) => {
  return (
    <>
      <Route path={match!.url} exact component={ServiceList} />
      <Route path={`${match!.url}/:id`} component={ServiceDetail} />
    </>
  );
};
