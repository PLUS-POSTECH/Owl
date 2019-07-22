import React from "react";
import { Header, Table, Container, Icon } from "semantic-ui-react";

import { prisma } from "../generated/prisma-client";
import { Loader, useAwait } from "./common";

export const EndpointPath = "/endpoint/";

interface EndpointWithTeamService {
  id: string;
  connectionString: string;
  team: {
    id: string;
  };
  service: {
    id: string;
  };
}

export const Endpoint: React.FC = () => {
  const fetchTeams = async () =>
    await prisma.teams({
      orderBy: "id_ASC"
    });

  const fetchServices = async () =>
    await prisma.services({
      orderBy: "createdAt_DESC",
      where: {
        enabled: true
      }
    });

  const fetchEndpoints = async (): Promise<EndpointWithTeamService[]> =>
    await prisma.endpoints().$fragment(`
    fragment EndpointWithTeamService on Endpoint {
      id
      connectionString
      team {
        id
      }
      service {
        id
      }
    }`);

  const status = useAwait([fetchTeams, fetchServices, fetchEndpoints]);

  return (
    <Loader
      status={status}
      render={([teamList, serviceList, endpointList]) => {
        const endpointMap: {
          [teamId: string]: { [serviceId: string]: EndpointWithTeamService };
        } = {};

        for (const team of teamList) {
          endpointMap[team.id] = {};
        }

        for (const endpoint of endpointList) {
          endpointMap[endpoint.team.id][endpoint.service.id] = endpoint;
        }

        return (
          <Container>
            <Header as="h1">Endpoints</Header>
            <Table celled basic="very">
              <Table.Header>
                <Table.Row>
                  <Table.HeaderCell />
                  {serviceList.map(service => (
                    <Table.HeaderCell key={service.id} textAlign="center">
                      {service.name}
                    </Table.HeaderCell>
                  ))}
                </Table.Row>
              </Table.Header>

              <Table.Body>
                {teamList.map(team => (
                  <Table.Row key={team.id}>
                    <Table.Cell textAlign="right">{team.name}</Table.Cell>
                    {serviceList.map(service => {
                      if (endpointMap[team.id][service.id]) {
                        return (
                          <Table.Cell
                            key={service.id}
                            selectable
                            textAlign="center"
                          >
                            <Icon name="linkify" />
                          </Table.Cell>
                        );
                      } else {
                        return (
                          <Table.Cell
                            key={service.id}
                            textAlign="center"
                            disabled
                          >
                            -
                          </Table.Cell>
                        );
                      }
                    })}
                  </Table.Row>
                ))}
              </Table.Body>
            </Table>
          </Container>
        );
      }}
    />
  );
};
