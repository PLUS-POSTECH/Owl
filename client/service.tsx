import React from "react";
import { Route, RouteChildrenProps } from "react-router";
import { Link } from "react-router-dom";
import { Header, Menu, Table, Segment, Container } from "semantic-ui-react";

import { prisma } from "./generated/prisma-client";
import { Loader, useAwait } from "./common";

export const ServicePath = "/service/";

const ServiceList: React.FC<RouteChildrenProps> = props => {
  const match = props.match!;

  const status = useAwait(
    async () =>
      await prisma.services({
        orderBy: "createdAt_DESC"
      })
  );

  return (
    <Loader
      status={status}
      render={serviceList => (
        <>
          <Header as="h1">Service List ({serviceList.length} services)</Header>
          <Menu size="large" fluid vertical>
            {serviceList.map(service => (
              <Menu.Item
                key={service.id}
                as={Link}
                to={`${match.url}${service.id}`}
              >
                {service.name}
              </Menu.Item>
            ))}
          </Menu>
        </>
      )}
    />
  );
};

interface EndpointWithTeam {
  id: string;
  connectionString: string;
  team: {
    name: string;
  };
}

type ServiceDetailProps = RouteChildrenProps<{ id: string }>;

const ServiceDetail: React.FC<ServiceDetailProps> = props => {
  const match = props.match!;

  const fetchServices = async () => {
    const result = await prisma.service({
      id: match.params.id
    });
    if (result === null) {
      throw `Service "${match.params.id}" not found`;
    }
    return result;
  };

  const fetchEndpoints = async (): Promise<EndpointWithTeam[]> =>
    await prisma.endpoints({
      where: {
        service: {
          id: match!.params.id
        }
      }
    }).$fragment(`
      fragment EndpointWithTeam on Endpoint {
        id
        connectionString
        team {
          name
        }
      }`);

  const status = useAwait([fetchServices, fetchEndpoints], [match.params.id]);

  return (
    <>
      <Link to={ServicePath}>&lt; back to service list</Link>
      <Loader
        status={status}
        render={([service, endpoints]) => (
          <>
            <Segment>
              <Header as="h1">{service.name}</Header>
              <p>{service.description}</p>
            </Segment>
            <Table celled>
              <Table.Header>
                <Table.Row>
                  <Table.HeaderCell>Team</Table.HeaderCell>
                  <Table.HeaderCell>Connection</Table.HeaderCell>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {endpoints.map(endpoint => (
                  <Table.Row key={endpoint.id}>
                    <Table.Cell>{endpoint.team.name}</Table.Cell>
                    <Table.Cell>{endpoint.connectionString}</Table.Cell>
                  </Table.Row>
                ))}
              </Table.Body>
            </Table>
          </>
        )}
      />
    </>
  );
};

export const Service: React.FC<RouteChildrenProps> = props => {
  const match = props.match!;
  return (
    <Container text>
      <Route path={match.path} exact component={ServiceList} />
      <Route path={`${match.path}:id`} component={ServiceDetail} />
    </Container>
  );
};
