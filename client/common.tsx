import React, { useState, useEffect } from "react";
import {
  Loader as SemanticLoader,
  Segment,
  Header,
  Icon
} from "semantic-ui-react";

interface LoaderProps<T> {
  status: AsyncStatus<T>;
  render?: (arg0: T) => React.ReactElement | null;
}

export function Loader<T>(props: LoaderProps<T>): React.ReactElement | null {
  if (props.status.pending) {
    return <SemanticLoader active inline="centered" />;
  } else if (props.status.error) {
    return <AccessFailure />;
  } else {
    if (props.render !== undefined) {
      return props.render(props.status.result);
    } else {
      return null;
    }
  }
}

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

type AsyncStatus<T> =
  | {
      pending: true;
    }
  | {
      pending: false;
      error: true;
    }
  | {
      pending: false;
      error: false;
      result: T;
    };

export function useAwait<T>(
  async: () => T | PromiseLike<T>,
  deps?: React.DependencyList
): AsyncStatus<T>;

export function useAwait<T>(
  async: (() => T | PromiseLike<T>)[],
  deps?: React.DependencyList
): AsyncStatus<T[]>;

export function useAwait<T1, T2>(
  async: [() => T1 | PromiseLike<T1>, () => T2 | PromiseLike<T2>],
  deps?: React.DependencyList
): AsyncStatus<[T1, T2]>;

export function useAwait<T1, T2, T3>(
  async: [
    () => T1 | PromiseLike<T1>,
    () => T2 | PromiseLike<T2>,
    () => T3 | PromiseLike<T3>
  ],
  deps?: React.DependencyList
): AsyncStatus<[T1, T2, T3]>;

export function useAwait(
  async: any,
  deps?: React.DependencyList
): AsyncStatus<any> {
  const [result, setResult] = useState<AsyncStatus<any>>({
    pending: true
  });

  deps = deps || [];

  useEffect(() => {
    let canceled = false;

    const fetchData = async () => {
      try {
        const result = Array.isArray(async)
          ? await Promise.all(async.map(fn => fn()))
          : await async();
        if (!canceled) {
          setResult({
            pending: false,
            error: false,
            result: result
          });
        }
      } catch (e) {
        console.error(e);
        if (!canceled) {
          setResult({
            pending: false,
            error: true
          });
        }
      }
    };

    fetchData();

    return () => {
      canceled = true;
    };
  }, deps);

  return result;
}
