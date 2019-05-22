import React from "react";
import { Loader as SemanticLoader } from "semantic-ui-react";

interface IProps {
  isLoading: boolean;
}

const Loader: React.FC<IProps> = props => {
  if (props.isLoading) {
    return <SemanticLoader active inline="centered" />;
  } else {
    return <>{props.children}</>;
  }
};

export = Loader;
