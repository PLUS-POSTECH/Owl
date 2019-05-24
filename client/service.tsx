import React from "react";
import { Route, RouteChildrenProps } from "react-router";
import { Link } from "react-router-dom";
import { Header, Menu, Table, Segment, Divider } from "semantic-ui-react";
import { prisma } from "./generated/prisma-client";

import { Loader, useAwait } from "./common";

const ServiceList: React.FC<RouteChildrenProps> = ({ match }) => {
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
                to={`${match!.url}/${service.id}`}
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
  connectionString: String;
  team: {
    name: String;
  };
}

type ServiceDetailProps = RouteChildrenProps<{ id: string }>;

const ServiceDetail: React.FC<ServiceDetailProps> = ({ match }) => {
  const fetchService = async () => {
    const result = await prisma.service({
      id: match!.params.id
    });
    if (result === null) {
      throw `Service "${match!.params.id}" not found`;
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
        connectionString
        team {
          name
        }
      }
    `);

  const status = useAwait([fetchService, fetchEndpoints]);

  return (
    <Loader
      status={status}
      render={([service, endpoints]) => (
        <>
          <Segment>
            <Header as="h1">{service.name}</Header>
            <p>{service.description}</p>
          </Segment>
          <Table basic="very" celled>
            <Table.Header>
              <Table.Row>
                <Table.HeaderCell>Team</Table.HeaderCell>
                <Table.HeaderCell>Connection</Table.HeaderCell>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {endpoints.map(endpoint => (
                <Table.Row>
                  <Table.Cell>{endpoint.team.name}</Table.Cell>
                  <Table.Cell>{endpoint.connectionString}</Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table>
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
