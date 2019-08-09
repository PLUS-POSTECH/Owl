import * as cluster from "cluster";
import { cpus } from "os";
import { run as runMaster } from "./master";
import { run as runWorker } from "./worker";

if (cluster.isMaster) {
  const numCpus = cpus().length;
  for (let i = 0; i < numCpus; i++) {
    cluster.fork();
  }
  runMaster();
} else {
  runWorker();
}
