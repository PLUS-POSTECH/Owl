import * as cluster from "cluster";
import { synchdFn } from "synchd";
import { prisma, Task } from "../generated/prisma-client";
import { Message, MessageType } from "./types";

async function fetchTask() {
  const unscheduledTasks: Task[] = await prisma.tasks({
    where: {
      status: "PENDING"
    },
    orderBy: "createdAt_ASC",
    first: 1
  });

  if (unscheduledTasks.length === 0) return null;

  const targetTask = unscheduledTasks[0];
  await prisma.updateTask({
    where: {
      id: targetTask.id
    },
    data: {
      status: "EXPLOITING"
    }
  });

  return targetTask.id;
}

const fetchLock = {};
const exclusiveTaskFetch = synchdFn(fetchLock, fetchTask);

async function assignWorker(worker: cluster.Worker) {
  const taskId = await exclusiveTaskFetch();
  if (taskId === null) {
    const sleepMessage: Message = { type: MessageType.Sleep };
    worker.send(sleepMessage);
  } else {
    const taskMessage: Message = {
      type: MessageType.TaskPush,
      message: taskId
    };
    worker.send(taskMessage);
  }
}

function wakeAllWorkers() {
  const workers = cluster.workers;
  const wakeupMessage: Message = { type: MessageType.Wakeup };
  for (const id in workers) {
    workers[id]!.send(wakeupMessage);
  }
}

function watchTasks(subscription: AsyncIterator<Task>) {
  subscription.next().then(() => {
    wakeAllWorkers();
    watchTasks(subscription);
  });
}

export async function run() {
  const workers = cluster.workers;
  for (const id in workers) {
    const worker = workers[id]!;
    worker.on("message", (message: Message) => {
      switch (message.type) {
        case MessageType.TaskRequest: {
          assignWorker(worker);
          break;
        }
        default: {
        }
      }
    });
  }

  const newTaskIterator = await prisma.$subscribe
    .task({ mutation_in: ["CREATED"] })
    .node();
  watchTasks(newTaskIterator);
}
