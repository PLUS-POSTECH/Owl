import React from "react";
import {
  Loader as SemanticLoader,
  Segment,
  Header,
  Icon
} from "semantic-ui-react";

interface LoaderProps {
  isLoading: boolean;
  isError?: boolean;
  render?: () => React.ReactNode;
}

export const Loader: React.FC<LoaderProps> = props => {
  if (props.isLoading) {
    return <SemanticLoader active inline="centered" />;
  } else if (!!props.isError) {
    return <AccessFailure />;
  } else {
    return (
      <>
        {props.render !== undefined && props.render()}
        {props.children}
      </>
    );
  }
};

export const AccessFailure: React.FC = () => {
  return (
    <Segment basic placeholder textAlign="center">
      <Header icon>
        <Icon style={{ padding: 12 }}>🤷</Icon>
        We found nothing at this location
      </Header>
      <p>Plaese attack CTF services instead of our infractructure</p>
    </Segment>
  );
};
